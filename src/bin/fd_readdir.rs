use libc;
use misc_tests::open_scratch_directory;
use misc_tests::wasi_wrappers::{wasi_fd_filestat_get, wasi_fd_readdir, wasi_path_open};
use std::{cmp::min, env, mem, process, slice, str};
use wasi::wasi_unstable;

const BUF_LEN: usize = 256;

#[derive(Debug)]
struct DirEntry {
    dirent: wasi_unstable::Dirent,
    name: String,
}

// Manually reading the output from fd_readdir is tedious and repetitive,
// so encapsulate it into an iterator
struct ReadDir<'a> {
    buf: &'a [u8],
}

impl<'a> ReadDir<'a> {
    fn from_slice(buf: &'a [u8]) -> Self {
        Self { buf }
    }
}

impl<'a> Iterator for ReadDir<'a> {
    type Item = DirEntry;

    fn next(&mut self) -> Option<DirEntry> {
        unsafe {
            if self.buf.len() == 0 {
                return None;
            }

            // Read the data
            let dirent_ptr = self.buf.as_ptr() as *const wasi_unstable::Dirent;
            let dirent = *dirent_ptr;
            let name_ptr = dirent_ptr.offset(1) as *const u8;
            // NOTE Linux syscall returns a NULL-terminated name, but WASI doesn't
            let namelen = dirent.d_namlen as usize;
            let slice = slice::from_raw_parts(name_ptr, namelen);
            let name = str::from_utf8(slice).expect("invalid utf8").to_owned();

            // Update the internal state
            let delta = mem::size_of_val(&dirent) + namelen;
            self.buf = &self.buf[delta..];

            DirEntry { dirent, name }.into()
        }
    }
}

fn test_fd_readdir(dir_fd: wasi_unstable::Fd) {
    let mut stat: wasi_unstable::FileStat = unsafe { mem::zeroed() };
    let status = wasi_fd_filestat_get(dir_fd, &mut stat);
    assert_eq!(
        status,
        wasi_unstable::ESUCCESS,
        "reading scratch directory stats"
    );

    // Check the behavior in an empty directory
    let mut buf: [u8; BUF_LEN] = [0; BUF_LEN];
    let mut bufused = unsafe { mem::zeroed() };
    let status = wasi_fd_readdir(dir_fd, &mut buf, BUF_LEN, 0, &mut bufused);
    assert_eq!(status, wasi_unstable::ESUCCESS, "fd_readdir");

    let sl = unsafe { slice::from_raw_parts(buf.as_ptr(), min(BUF_LEN, bufused)) };
    let mut dirs: Vec<_> = ReadDir::from_slice(sl).collect();
    println!("{:?}", dirs);
    assert_eq!(dirs.len(), 2, "expected two entries in an empty directory");
    dirs.sort_by_key(|d| d.name.clone());
    let mut dirs = dirs.into_iter();

    // the first entry should be `.`
    let dir = dirs.next().expect("first entry is None");
    assert_eq!(dir.name, ".", "first name");
    assert_eq!(
        dir.dirent.d_type,
        wasi_unstable::FILETYPE_REGULAR_FILE,
        "first type"
    ); // WHY??
    assert_eq!(dir.dirent.d_ino, stat.st_ino);
    assert_eq!(dir.dirent.d_namlen, 1);

    // the second entry should be `..`
    let dir = dirs.next().expect("second entry is None");
    assert_eq!(dir.name, "..", "second name");
    assert_eq!(
        dir.dirent.d_type,
        wasi_unstable::FILETYPE_REGULAR_FILE,
        "second type"
    ); // WHY??

    assert!(
        dirs.next().is_none(),
        "the directory should be seen as empty"
    );

    // Add a file and check the behavior
    let mut file_fd = wasi_unstable::Fd::max_value() - 1;
    let status = wasi_path_open(
        dir_fd,
        0,
        "file",
        wasi_unstable::O_CREAT,
        wasi_unstable::RIGHT_FD_READ | wasi_unstable::RIGHT_FD_WRITE,
        0,
        0,
        &mut file_fd,
    );
    assert_eq!(status, wasi_unstable::ESUCCESS, "opening a file");
    assert!(
        file_fd > libc::STDERR_FILENO as wasi_unstable::Fd,
        "file descriptor range check",
    );

    let status = wasi_fd_filestat_get(file_fd, &mut stat);
    assert_eq!(status, wasi_unstable::ESUCCESS, "reading file stats");

    println!("Executing another readdir");
    // Execute another readdir
    let status = wasi_fd_readdir(dir_fd, &mut buf, BUF_LEN, 0, &mut bufused);
    assert_eq!(status, wasi_unstable::ESUCCESS, "fd_readdir");

    let sl = unsafe { slice::from_raw_parts(buf.as_ptr(), min(BUF_LEN, bufused)) };
    let mut dirs: Vec<_> = ReadDir::from_slice(sl).collect();
    println!("{:?}", dirs);
    assert_eq!(dirs.len(), 3, "expected three entries");
    dirs.sort_by_key(|d| d.name.clone());
    let mut dirs = dirs.into_iter();

    let dir = dirs.next().expect("first entry is None");
    assert_eq!(dir.name, ".", "first name");
    let dir = dirs.next().expect("second entry is None");
    assert_eq!(dir.name, "..", "second name");
    let dir = dirs.next().expect("third entry is None");

    // check the file info
    assert_eq!(dir.name, "file", "file name doesn't match");
    assert_eq!(
        dir.dirent.d_type,
        wasi_unstable::FILETYPE_REGULAR_FILE,
        "second type"
    );
    assert_eq!(dir.dirent.d_ino, stat.st_ino);
}
fn main() {
    let mut args = env::args();
    let prog = args.next().unwrap();
    let arg = if let Some(arg) = args.next() {
        arg
    } else {
        eprintln!("usage: {} <scratch directory>", prog);
        process::exit(1);
    };

    // Open scratch directory
    let dir_fd = match open_scratch_directory(&arg) {
        Ok(dir_fd) => dir_fd,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1)
        }
    };

    // Run the tests.
    test_fd_readdir(dir_fd)
}

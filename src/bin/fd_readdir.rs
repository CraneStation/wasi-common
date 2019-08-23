use libc;
use misc_tests::open_scratch_directory;
use misc_tests::wasi_wrappers::{wasi_fd_filestat_get, wasi_fd_readdir};
use std::{cmp::min, env, mem, process, slice, str};

const BUF_LEN: usize = 256;

struct DirEntry {
    dirent: libc::__wasi_dirent_t,
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
            let dirent_ptr = self.buf.as_ptr() as *const libc::__wasi_dirent_t;
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

fn test_fd_readdir(dir_fd: libc::__wasi_fd_t) {
    let mut stat: libc::__wasi_filestat_t = unsafe { mem::zeroed() };
    let status = wasi_fd_filestat_get(dir_fd, &mut stat);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "reading scratch directory stats"
    );

    let mut buf: [u8; BUF_LEN] = [0; BUF_LEN];
    let mut bufused = unsafe { mem::zeroed() };
    let status = wasi_fd_readdir(dir_fd, &mut buf, BUF_LEN, 0, &mut bufused);
    assert_eq!(status, libc::__WASI_ESUCCESS, "fd_readdir");
    // Create a file in the scratch directory.

    let sl = unsafe { slice::from_raw_parts(buf.as_ptr(), min(BUF_LEN, bufused)) };
    let mut dirs = ReadDir::from_slice(sl);

    // the first entry should be `.`
    let dir = dirs.next().expect("first entry is None");
    assert_eq!(dir.name, ".", "first name");
    assert_eq!(
        dir.dirent.d_type,
        libc::__WASI_FILETYPE_REGULAR_FILE,
        "first type"
    ); // WHY??
    assert_eq!(dir.dirent.d_ino, stat.st_ino);
    assert_eq!(dir.dirent.d_namlen, 1);

    // the second entry should be `..`
    let dir = dirs.next().expect("second entry is None");
    assert_eq!(dir.name, "..", "second name");
    assert_eq!(
        dir.dirent.d_type,
        libc::__WASI_FILETYPE_REGULAR_FILE,
        "second type"
    ); // WHY??

    assert!(
        dirs.next().is_none(),
        "the directory should be seen as empty"
    );
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

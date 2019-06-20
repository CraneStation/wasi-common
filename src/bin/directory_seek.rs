use libc;
use misc_tests::open_scratch_directory;
use misc_tests::utils::{cleanup_dir, close_fd, create_dir};
use misc_tests::wasi::{wasi_fd_fdstat_get, wasi_fd_seek, wasi_path_open};
use std::{env, mem, process};

fn test_directory_seek(dir_fd: libc::__wasi_fd_t) {
    // Create a directory in the scratch directory.
    create_dir(dir_fd, "dir");

    // Open the directory and attempt to request rights for seeking.
    let mut fd: libc::__wasi_fd_t = libc::__wasi_fd_t::max_value() - 1;
    let mut status = wasi_path_open(
        dir_fd,
        0,
        "dir",
        0,
        libc::__WASI_RIGHT_FD_SEEK,
        0,
        0,
        &mut fd,
    );
    assert_eq!(status, libc::__WASI_ESUCCESS, "opening a file");
    assert!(
        fd > libc::STDERR_FILENO as libc::__wasi_fd_t,
        "file descriptor range check",
    );

    // Attempt to seek.
    let mut newoffset = 1;
    status = wasi_fd_seek(fd, 0, libc::__WASI_WHENCE_CUR, &mut newoffset);
    assert_eq!(status, libc::__WASI_ENOTCAPABLE, "seek on a directory");

    // Check if we obtained the right to seek.
    let mut fdstat: libc::__wasi_fdstat_t = unsafe { mem::zeroed() };
    status = wasi_fd_fdstat_get(fd, &mut fdstat);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "calling fd_fdstat on a directory"
    );
    assert!(
        fdstat.fs_filetype == libc::__WASI_FILETYPE_DIRECTORY,
        "expected the scratch directory to be a directory",
    );
    assert_eq!(
        (fdstat.fs_rights_base & libc::__WASI_RIGHT_FD_SEEK),
        0,
        "directory has the seek right",
    );

    // Clean up.
    close_fd(fd);
    cleanup_dir(dir_fd, "dir");
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
    test_directory_seek(dir_fd)
}

use libc;
use misc_tests::open_scratch_directory;
use misc_tests::wasi::{wasi_fd_close, wasi_fd_fdstat_get, wasi_fd_renumber};
use std::{env, mem, process};

fn test_close_preopen(dir_fd: libc::__wasi_fd_t) {
    let pre_fd: libc::__wasi_fd_t = (libc::STDERR_FILENO + 1) as libc::__wasi_fd_t;

    assert!(dir_fd > pre_fd, "dir_fd number");

    // Try to close a preopened directory handle.
    let mut status = wasi_fd_close(pre_fd);
    assert_eq!(
        status,
        libc::__WASI_ENOTSUP,
        "closing a preopened file descriptor",
    );

    // Try to renumber over a preopened directory handle.
    status = wasi_fd_renumber(dir_fd, pre_fd);
    assert_eq!(
        status,
        libc::__WASI_ENOTSUP,
        "renumbering over a preopened file descriptor",
    );

    // Ensure that dir_fd is still open.
    let mut dir_fdstat: libc::__wasi_fdstat_t = unsafe { mem::zeroed() };
    status = wasi_fd_fdstat_get(dir_fd, &mut dir_fdstat);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "calling fd_fdstat on the scratch directory"
    );
    assert!(
        dir_fdstat.fs_filetype == libc::__WASI_FILETYPE_DIRECTORY,
        "expected the scratch directory to be a directory",
    );

    // Try to renumber a preopened directory handle.
    status = wasi_fd_renumber(pre_fd, dir_fd);
    assert_eq!(
        status,
        libc::__WASI_ENOTSUP,
        "renumbering over a preopened file descriptor",
    );

    // Ensure that dir_fd is still open.
    status = wasi_fd_fdstat_get(dir_fd, &mut dir_fdstat);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "calling fd_fdstat on the scratch directory"
    );
    assert!(
        dir_fdstat.fs_filetype == libc::__WASI_FILETYPE_DIRECTORY,
        "expected the scratch directory to be a directory",
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
    test_close_preopen(dir_fd)
}

use libc;
use misc_tests::open_scratch_directory;
use misc_tests::utils::cleanup_file;
use misc_tests::wasi::{wasi_path_open, wasi_path_symlink};
use std::{env, process};

fn test_dangling_symlink(dir_fd: libc::__wasi_fd_t) {
    // First create a dangling symlink.
    let mut status = wasi_path_symlink("target", dir_fd, "symlink");
    assert_eq!(status, libc::__WASI_ESUCCESS, "creating a symlink");

    // Try to open it as a directory with O_NOFOLLOW.
    let mut file_fd: libc::__wasi_fd_t = libc::__wasi_fd_t::max_value() - 1;
    status = wasi_path_open(
        dir_fd,
        0,
        "symlink",
        libc::__WASI_O_DIRECTORY,
        0,
        0,
        0,
        &mut file_fd,
    );
    assert_eq!(
        status,
        libc::__WASI_ELOOP,
        "opening a dangling symlink as a directory",
    );
    assert_eq!(
        file_fd,
        libc::__wasi_fd_t::max_value(),
        "failed open should set the file descriptor to -1",
    );

    // Clean up.
    cleanup_file(dir_fd, "symlink");
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
    test_dangling_symlink(dir_fd)
}

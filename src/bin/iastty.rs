use libc;
use misc_tests::open_scratch_directory;
use misc_tests::utils::{cleanup_file, close_fd};
use misc_tests::wasi::wasi_path_open;
use std::{env, process};

fn test_isatty(dir_fd: libc::__wasi_fd_t) {
    // Create a file in the scratch directory and test if it's a tty.
    let mut file_fd: libc::__wasi_fd_t = libc::__wasi_fd_t::max_value() - 1;
    let status = wasi_path_open(
        dir_fd,
        0,
        "file",
        libc::__WASI_O_CREAT,
        0,
        0,
        0,
        &mut file_fd,
    );
    assert_eq!(status, libc::__WASI_ESUCCESS, "opening a file");
    assert!(
        file_fd > libc::STDERR_FILENO as libc::__wasi_fd_t,
        "file descriptor range check",
    );
    assert_eq!(
        unsafe { libc::isatty(file_fd as libc::c_int) },
        0,
        "file is a tty"
    );
    close_fd(file_fd);

    cleanup_file(dir_fd, "file");
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
    test_isatty(dir_fd)
}

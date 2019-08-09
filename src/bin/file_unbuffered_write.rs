use libc;
use misc_tests::open_scratch_directory;
use misc_tests::utils::{cleanup_file, close_fd, create_file};
use misc_tests::wasi::{wasi_fd_read, wasi_fd_write, wasi_path_open};
use std::{env, process};

fn test_file_unbuffered_write(dir_fd: libc::__wasi_fd_t) {
    // Create file
    create_file(dir_fd, "file");

    // Open file for reading
    let mut fd_read = libc::__wasi_fd_t::max_value() - 1;
    let mut status = wasi_path_open(
        dir_fd,
        0,
        "file",
        0,
        libc::__WASI_RIGHT_FD_READ,
        0,
        0,
        &mut fd_read,
    );
    assert_eq!(status, libc::__WASI_ESUCCESS, "opening a file");
    assert!(
        fd_read > libc::STDERR_FILENO as libc::__wasi_fd_t,
        "file descriptor range check",
    );

    // Open the same file but for writing
    let mut fd_write = libc::__wasi_fd_t::max_value() - 1;
    status = wasi_path_open(
        dir_fd,
        0,
        "file",
        0,
        libc::__WASI_RIGHT_FD_WRITE,
        0,
        0,
        &mut fd_write,
    );
    assert_eq!(status, libc::__WASI_ESUCCESS, "opening a file");
    assert!(
        fd_write > libc::STDERR_FILENO as libc::__wasi_fd_t,
        "file descriptor range check",
    );

    // Write to file
    let contents = &[1u8];
    let ciovec = libc::__wasi_ciovec_t {
        buf: contents.as_ptr() as *const libc::c_void,
        buf_len: contents.len(),
    };
    let mut nwritten = 0;
    status = wasi_fd_write(fd_write, &mut [ciovec], &mut nwritten);
    assert_eq!(status, libc::__WASI_ESUCCESS, "writing byte to file");
    assert_eq!(nwritten, 1, "nwritten bytes check");

    // Read from file
    let contents = &mut [0u8; 1];
    let iovec = libc::__wasi_iovec_t {
        buf: contents.as_mut_ptr() as *mut libc::c_void,
        buf_len: contents.len(),
    };
    let mut nread = 0;
    status = wasi_fd_read(fd_read, &[iovec], &mut nread);
    assert_eq!(status, libc::__WASI_ESUCCESS, "reading bytes from file");
    assert_eq!(nread, 1, "nread bytes check");
    assert_eq!(contents, &[1u8], "written bytes equal read bytes");

    // Clean up
    close_fd(fd_write);
    close_fd(fd_read);
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
    test_file_unbuffered_write(dir_fd)
}

use libc;
use misc_tests::open_scratch_directory;
use misc_tests::utils::close_fd;
use misc_tests::wasi::{wasi_fd_pread, wasi_fd_pwrite, wasi_path_open};
use std::{env, process};

fn test_file_pread_pwrite(dir_fd: libc::__wasi_fd_t) {
    // Create a file in the scratch directory.
    let mut file_fd = libc::__wasi_fd_t::max_value() - 1;
    let mut status = wasi_path_open(
        dir_fd,
        0,
        "file",
        libc::__WASI_O_CREAT,
        libc::__WASI_RIGHT_FD_READ | libc::__WASI_RIGHT_FD_WRITE,
        0,
        0,
        &mut file_fd,
    );
    assert_eq!(status, libc::__WASI_ESUCCESS, "opening a file");
    assert!(
        file_fd > libc::STDERR_FILENO as libc::__wasi_fd_t,
        "file descriptor range check",
    );

    let contents = &[0u8, 1, 2, 3];
    let ciovec = libc::__wasi_ciovec_t {
        buf: contents.as_ptr() as *const libc::c_void,
        buf_len: contents.len(),
    };
    let mut nwritten = 0;
    status = wasi_fd_pwrite(file_fd, &mut [ciovec], 0, &mut nwritten);
    assert_eq!(status, libc::__WASI_ESUCCESS, "writing bytes at offset 0");
    assert_eq!(nwritten, 4, "nwritten bytes check");

    let contents = &mut [0u8; 4];
    let iovec = libc::__wasi_iovec_t {
        buf: contents.as_mut_ptr() as *mut libc::c_void,
        buf_len: contents.len(),
    };
    let mut nread = 0;
    status = wasi_fd_pread(file_fd, &[iovec], 0, &mut nread);
    assert_eq!(status, libc::__WASI_ESUCCESS, "reading bytes at offset 0");
    assert_eq!(nread, 4, "nread bytes check");
    assert_eq!(contents, &[0u8, 1, 2, 3], "written bytes equal read bytes");

    let contents = &mut [0u8; 4];
    let iovec = libc::__wasi_iovec_t {
        buf: contents.as_mut_ptr() as *mut libc::c_void,
        buf_len: contents.len(),
    };
    let mut nread = 0;
    status = wasi_fd_pread(file_fd, &[iovec], 2, &mut nread);
    assert_eq!(status, libc::__WASI_ESUCCESS, "reading bytes at offset 2");
    assert_eq!(nread, 2, "nread bytes check");
    assert_eq!(contents, &[2u8, 3, 0, 0], "file cursor was overwritten");

    let contents = &[1u8, 0];
    let ciovec = libc::__wasi_ciovec_t {
        buf: contents.as_ptr() as *const libc::c_void,
        buf_len: contents.len(),
    };
    let mut nwritten = 0;
    status = wasi_fd_pwrite(file_fd, &mut [ciovec], 2, &mut nwritten);
    assert_eq!(status, libc::__WASI_ESUCCESS, "writing bytes at offset 2");
    assert_eq!(nwritten, 2, "nwritten bytes check");

    let contents = &mut [0u8; 4];
    let iovec = libc::__wasi_iovec_t {
        buf: contents.as_mut_ptr() as *mut libc::c_void,
        buf_len: contents.len(),
    };
    let mut nread = 0;
    status = wasi_fd_pread(file_fd, &[iovec], 0, &mut nread);
    assert_eq!(status, libc::__WASI_ESUCCESS, "reading bytes at offset 0");
    assert_eq!(nread, 4, "nread bytes check");
    assert_eq!(contents, &[0u8, 1, 1, 0], "file cursor was overwritten");

    close_fd(file_fd);
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
    test_file_pread_pwrite(dir_fd)
}

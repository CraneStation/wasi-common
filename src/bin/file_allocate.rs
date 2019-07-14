use libc;
use misc_tests::open_scratch_directory;
use misc_tests::utils::{cleanup_file, close_fd};
use misc_tests::wasi::{wasi_fd_allocate, wasi_fd_filestat_get, wasi_path_open};
use std::{env, process};

fn test_file_allocate(dir_fd: libc::__wasi_fd_t) {
    // Create a file in the scratch directory.
    let mut file_fd = libc::__wasi_fd_t::max_value() - 1;
    let status = wasi_path_open(
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

    // Check file size
    let mut stat = libc::__wasi_filestat_t {
        st_dev: 0,
        st_ino: 0,
        st_filetype: 0,
        st_nlink: 0,
        st_size: 0,
        st_atim: 0,
        st_mtim: 0,
        st_ctim: 0,
    };
    let status = wasi_fd_filestat_get(file_fd, &mut stat);
    assert_eq!(status, libc::__WASI_ESUCCESS, "reading file stats");
    assert_eq!(stat.st_size, 0, "file size should be 0");

    // Allocate some size
    let status = wasi_fd_allocate(file_fd, 0, 100);
    assert_eq!(status, libc::__WASI_ESUCCESS, "allocating size");

    let status = wasi_fd_filestat_get(file_fd, &mut stat);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "reading file stats after initial allocation"
    );
    assert_eq!(stat.st_size, 100, "file size should be 100");

    // Allocate should not modify if less than current size
    let status = wasi_fd_allocate(file_fd, 10, 10);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "allocating size less than current size"
    );

    let status = wasi_fd_filestat_get(file_fd, &mut stat);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "reading file stats after additional allocation was not required"
    );
    assert_eq!(
        stat.st_size, 100,
        "file size should remain unchanged at 100"
    );

    // Allocate should modify if offset+len > current_len
    let status = wasi_fd_allocate(file_fd, 90, 20);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "allocating size larger than current size"
    );

    let status = wasi_fd_filestat_get(file_fd, &mut stat);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "reading file stats after additional allocation was required"
    );
    assert_eq!(
        stat.st_size, 110,
        "file size should increase from 100 to 110"
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
    test_file_allocate(dir_fd)
}

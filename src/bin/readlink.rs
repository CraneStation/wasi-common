use libc;
use misc_tests::open_scratch_directory;
use misc_tests::utils::{cleanup_file, create_file};
use misc_tests::wasi::{wasi_path_readlink, wasi_path_symlink};
use std::{env, process};

fn test_readlink(dir_fd: libc::__wasi_fd_t) {
    // Create a file in the scratch directory.
    create_file(dir_fd, "target");

    // Create a symlink
    let mut status = wasi_path_symlink("target", dir_fd, "symlink");
    assert_eq!(status, libc::__WASI_ESUCCESS, "creating a symlink");

    // Read link into the buffer
    let buf = &mut [0u8; 10];
    let mut bufused: usize = 0;
    status = wasi_path_readlink(dir_fd, "symlink", buf, &mut bufused);
    assert_eq!(status, libc::__WASI_ESUCCESS, "readlink should succeed");
    assert_eq!(bufused, 6, "should use 6 bytes of the buffer");
    assert_eq!(
        &buf[..6],
        "target".as_bytes(),
        "buffer should contain 'target'"
    );
    assert_eq!(
        &buf[6..],
        &[0u8; 4],
        "the remaining bytes should be untouched"
    );

    // Read link into smaller buffer than the actual link's length
    let buf = &mut [0u8; 4];
    let mut bufused: usize = 0;
    status = wasi_path_readlink(dir_fd, "symlink", buf, &mut bufused);
    assert_eq!(status, libc::__WASI_ESUCCESS, "readlink should succeed");
    assert_eq!(bufused, 4, "should use all 4 bytes of the buffer");
    assert_eq!(buf, "targ".as_bytes(), "buffer should contain 'targ'");

    // Clean up.
    cleanup_file(dir_fd, "target");
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
    test_readlink(dir_fd)
}

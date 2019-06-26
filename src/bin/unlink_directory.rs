use libc;
use misc_tests::open_scratch_directory;
use misc_tests::utils::{cleanup_dir, create_dir};
use misc_tests::wasi::wasi_path_unlink_file;
use std::{env, process};

fn test_unlink_directory(dir_fd: libc::__wasi_fd_t) {
    // Create a directory in the scratch directory.
    create_dir(dir_fd, "dir");

    // Test that unlinking it fails.
    let status = wasi_path_unlink_file(dir_fd, "dir");
    assert_eq!(
        status,
        libc::__WASI_EISDIR,
        "unlink_file on a directory should fail"
    );

    // Clean up.
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
    test_unlink_directory(dir_fd)
}

use libc;
use misc_tests::open_scratch_directory;
use misc_tests::utils::cleanup_file;
use misc_tests::wasi::{wasi_path_readlink, wasi_path_symlink};
use std::{env, process};

fn test_readlink_no_buffer(dir_fd: libc::__wasi_fd_t) {
    // First create a dangling symlink.
    let mut status = wasi_path_symlink("target", dir_fd, "symlink");
    assert_eq!(status, libc::__WASI_ESUCCESS, "creating a symlink");

    // Readlink it into a non-existent buffer.
    let mut bufused: usize = 1;
    status = wasi_path_readlink(dir_fd, "symlink", &mut [], &mut bufused);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "readlink with a 0-sized buffer should succeed"
    );
    assert_eq!(
        bufused, 0,
        "readlink with a 0-sized buffer should return 'bufused' 0"
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
    test_readlink_no_buffer(dir_fd)
}

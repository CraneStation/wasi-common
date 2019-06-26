use libc;
use misc_tests::open_scratch_directory;
use misc_tests::utils::{cleanup_file, close_fd, create_dir};
use misc_tests::wasi::{wasi_path_open, wasi_path_remove_directory, wasi_path_symlink};
use std::{env, process};

fn test_nofollow_errors(dir_fd: libc::__wasi_fd_t) {
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

    // Create a directory for the symlink to point to.
    create_dir(dir_fd, "target");

    // Try to open it as a directory with O_NOFOLLOW again.
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
        "opening a directory symlink as a directory",
    );
    assert_eq!(
        file_fd,
        libc::__wasi_fd_t::max_value(),
        "failed open should set the file descriptor to -1",
    );

    // Try to open it with just O_NOFOLLOW.
    status = wasi_path_open(dir_fd, 0, "symlink", 0, 0, 0, 0, &mut file_fd);
    assert_eq!(
        status,
        libc::__WASI_ELOOP,
        "opening a symlink with O_NOFOLLOW should return ELOOP",
    );
    assert_eq!(
        file_fd,
        libc::__wasi_fd_t::max_value(),
        "failed open should set the file descriptor to -1",
    );

    // Try to open it as a directory without O_NOFOLLOW.
    status = wasi_path_open(
        dir_fd,
        libc::__WASI_LOOKUP_SYMLINK_FOLLOW,
        "symlink",
        libc::__WASI_O_DIRECTORY,
        0,
        0,
        0,
        &mut file_fd,
    );
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "opening a symlink as a directory"
    );
    assert!(
        file_fd > libc::STDERR_FILENO as libc::__wasi_fd_t,
        "file descriptor range check",
    );
    close_fd(file_fd);

    // Replace the target directory with a file.
    status = wasi_path_remove_directory(dir_fd, "target");
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "remove_directory on a directory should succeed"
    );
    status = wasi_path_open(
        dir_fd,
        0,
        "target",
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
    close_fd(file_fd);

    // Try to open it as a directory with O_NOFOLLOW again.
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
        "opening a directory symlink as a directory",
    );
    assert_eq!(
        file_fd,
        libc::__wasi_fd_t::max_value(),
        "failed open should set the file descriptor to -1",
    );

    // Try to open it with just O_NOFOLLOW.
    status = wasi_path_open(dir_fd, 0, "symlink", 0, 0, 0, 0, &mut file_fd);
    assert_eq!(
        status,
        libc::__WASI_ELOOP,
        "opening a symlink with O_NOFOLLOW should return ELOOP",
    );
    assert_eq!(
        file_fd,
        libc::__wasi_fd_t::max_value(),
        "failed open should set the file descriptor to -1",
    );

    // Try to open it as a directory without O_NOFOLLOW.
    status = wasi_path_open(
        dir_fd,
        libc::__WASI_LOOKUP_SYMLINK_FOLLOW,
        "symlink",
        libc::__WASI_O_DIRECTORY,
        0,
        0,
        0,
        &mut file_fd,
    );
    assert_eq!(
        status,
        libc::__WASI_ENOTDIR,
        "opening a symlink to a file as a directory",
    );
    assert_eq!(
        file_fd,
        libc::__wasi_fd_t::max_value(),
        "failed open should set the file descriptor to -1",
    );

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
    test_nofollow_errors(dir_fd)
}

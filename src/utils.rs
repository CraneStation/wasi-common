use crate::wasi_wrappers::*;
use wasi::wasi_unstable;

pub fn create_dir(dir_fd: wasi_unstable::Fd, dir_name: &str) {
    let status = wasi_path_create_directory(dir_fd, dir_name);
    assert_eq!(status, wasi_unstable::ESUCCESS, "creating a directory");
}

pub fn cleanup_dir(dir_fd: wasi_unstable::Fd, dir_name: &str) {
    let status = wasi_path_remove_directory(dir_fd, dir_name);
    assert_eq!(
        status,
        wasi_unstable::ESUCCESS,
        "remove_directory on an empty directory should succeed"
    );
}

/// Create an empty file with the given name.
pub fn create_file(dir_fd: wasi_unstable::Fd, file_name: &str) {
    let mut file_fd = wasi_unstable::Fd::max_value() - 1;
    let status = wasi_path_open(
        dir_fd,
        0,
        file_name,
        wasi_unstable::O_CREAT,
        0,
        0,
        0,
        &mut file_fd,
    );
    assert_eq!(status, wasi_unstable::ESUCCESS, "creating a file");
    assert!(
        file_fd > libc::STDERR_FILENO as wasi_unstable::Fd,
        "file descriptor range check",
    );
    close_fd(file_fd);
}

pub fn cleanup_file(dir_fd: wasi_unstable::Fd, file_name: &str) {
    let status = wasi_path_unlink_file(dir_fd, file_name);
    assert_eq!(
        status,
        wasi_unstable::ESUCCESS,
        "unlink_file on a symlink should succeed"
    );
}

pub fn close_fd(fd: wasi_unstable::Fd) {
    assert_eq!(
        wasi_unstable::fd_close(fd),
        wasi_unstable::ESUCCESS,
        "closing a file"
    );
}

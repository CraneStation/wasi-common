use crate::wasi::*;
use libc;

pub fn create_dir(dir_fd: libc::__wasi_fd_t, dir_name: &str) {
    let status = wasi_path_create_directory(dir_fd, dir_name);
    assert_eq!(status, libc::__WASI_ESUCCESS, "creating a directory");
}

pub fn cleanup_dir(dir_fd: libc::__wasi_fd_t, dir_name: &str) {
    let status = wasi_path_remove_directory(dir_fd, dir_name);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "remove_directory on an empty directory should succeed"
    );
}

/// Create an empty file with the given name.
pub fn create_file(dir_fd: libc::__wasi_fd_t, file_name: &str) {
    let mut file_fd = libc::__wasi_fd_t::max_value() - 1;
    let status = wasi_path_open(
        dir_fd,
        0,
        file_name,
        libc::__WASI_O_CREAT,
        0,
        0,
        0,
        &mut file_fd,
    );
    assert_eq!(status, libc::__WASI_ESUCCESS, "creating a file");
    assert!(
        file_fd > libc::STDERR_FILENO as libc::__wasi_fd_t,
        "file descriptor range check",
    );
    close_fd(file_fd);
}

pub fn cleanup_file(dir_fd: libc::__wasi_fd_t, file_name: &str) {
    let status = wasi_path_unlink_file(dir_fd, file_name);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "unlink_file on a symlink should succeed"
    );
}

pub fn close_fd(fd: libc::__wasi_fd_t) {
    assert_eq!(wasi_fd_close(fd), libc::__WASI_ESUCCESS, "closing a file");
}

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

use errno;
use libc;
use std::ffi::CString;
use std::{env, mem, process, ptr};

unsafe fn cleanup_dir(dir_fd: libc::__wasi_fd_t, dir_name: &str) {
    let status = libc::__wasi_path_remove_directory(
        dir_fd,
        dir_name.as_ptr() as *const libc::c_char,
        dir_name.len(),
    );
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "remove_directory on an empty directory should succeed"
    );
}

unsafe fn cleanup_file(dir_fd: libc::__wasi_fd_t, file_name: &str) {
    let status = libc::__wasi_path_unlink_file(
        dir_fd,
        file_name.as_ptr() as *const libc::c_char,
        file_name.len(),
    );
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "unlink_file on a symlink should succeed"
    );
}

unsafe fn test_sched_yield() {
    let status = libc::__wasi_sched_yield();
    assert_eq!(status, libc::__WASI_ESUCCESS, "sched_yield");
}

unsafe fn test_truncation_rights(dir_fd: libc::__wasi_fd_t) {
    // Create a file in the scratch directory.
    let mut file_fd: libc::__wasi_fd_t = libc::__wasi_fd_t::max_value() - 1;
    let mut status: libc::__wasi_errno_t = libc::__wasi_path_open(
        dir_fd,
        0,
        "file".as_ptr() as *const libc::c_char,
        "file".len(),
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
    assert_eq!(
        libc::__wasi_fd_close(file_fd),
        libc::__WASI_ESUCCESS,
        "closing a file"
    );

    // Get the rights for the scratch directory.
    let mut dir_fdstat: libc::__wasi_fdstat_t = mem::zeroed();
    status = libc::__wasi_fd_fdstat_get(dir_fd, &mut dir_fdstat);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "calling fd_fdstat on the scratch directory"
    );
    assert!(
        dir_fdstat.fs_filetype == libc::__WASI_FILETYPE_DIRECTORY,
        "expected the scratch directory to be a directory",
    );
    assert!(
        dir_fdstat.fs_flags == 0,
        "expected the scratch directory to have no special flags",
    );

    // If we have the right to set sizes from paths, test that it works.
    if (dir_fdstat.fs_rights_base & libc::__WASI_RIGHT_PATH_FILESTAT_SET_SIZE) == 0 {
        eprintln!("implementation doesn't support setting file sizes, skipping");
    } else {
        // Test that we can truncate the file.
        status = libc::__wasi_path_open(
            dir_fd,
            0,
            "file".as_ptr() as *const libc::c_char,
            "file".len(),
            libc::__WASI_O_TRUNC,
            0,
            0,
            0,
            &mut file_fd,
        );
        assert_eq!(status, libc::__WASI_ESUCCESS, "truncating a file");
        assert_eq!(
            libc::__wasi_fd_close(file_fd),
            libc::__WASI_ESUCCESS,
            "closing a file"
        );

        let mut rights_base: libc::__wasi_rights_t = dir_fdstat.fs_rights_base;
        let mut rights_inheriting: libc::__wasi_rights_t = dir_fdstat.fs_rights_inheriting;

        if (rights_inheriting & libc::__WASI_RIGHT_FD_FILESTAT_SET_SIZE) == 0 {
            eprintln!("implementation doesn't support setting file sizes through file descriptors, skipping");
        } else {
            rights_inheriting &= !libc::__WASI_RIGHT_FD_FILESTAT_SET_SIZE;
            status = libc::__wasi_fd_fdstat_set_rights(dir_fd, rights_base, rights_inheriting);
            assert_eq!(
                status,
                libc::__WASI_ESUCCESS,
                "droping fd_filestat_set_size inheriting right on a directory",
            );
        }

        // Test that we can truncate the file without the
        // __WASI_RIGHT_FD_FILESTAT_SET_SIZE right.
        status = libc::__wasi_path_open(
            dir_fd,
            0,
            "file".as_ptr() as *const libc::c_char,
            "file".len(),
            libc::__WASI_O_TRUNC,
            0,
            0,
            0,
            &mut file_fd,
        );
        assert_eq!(
            status,
            libc::__WASI_ESUCCESS,
            "truncating a file without fd_filestat_set_size right",
        );
        assert_eq!(
            libc::__wasi_fd_close(file_fd),
            libc::__WASI_ESUCCESS,
            "closing a file"
        );

        rights_base &= !libc::__WASI_RIGHT_PATH_FILESTAT_SET_SIZE;
        status = libc::__wasi_fd_fdstat_set_rights(dir_fd, rights_base, rights_inheriting);
        assert_eq!(
            status,
            libc::__WASI_ESUCCESS,
            "droping path_filestat_set_size base right on a directory",
        );

        // Test that we can't truncate the file without the
        // __WASI_RIGHT_PATH_FILESTAT_SET_SIZE right.
        status = libc::__wasi_path_open(
            dir_fd,
            0,
            "file".as_ptr() as *const libc::c_char,
            "file".len(),
            libc::__WASI_O_TRUNC,
            0,
            0,
            0,
            &mut file_fd,
        );
        assert_eq!(
            status,
            libc::__WASI_ENOTCAPABLE,
            "truncating a file without path_filestat_set_size right",
        );
        assert_eq!(
            file_fd,
            libc::__wasi_fd_t::max_value(),
            "failed open should set the file descriptor to -1",
        );
    }

    status =
        libc::__wasi_path_unlink_file(dir_fd, "file".as_ptr() as *const libc::c_char, "file".len());
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "unlink_file on a file should succeed"
    );
}

unsafe fn test_unlink_directory(dir_fd: libc::__wasi_fd_t) {
    // Create a directory in the scratch directory.
    let mut status: libc::__wasi_errno_t = libc::__wasi_path_create_directory(
        dir_fd,
        "dir".as_ptr() as *const libc::c_char,
        "dir".len(),
    );
    assert_eq!(status, libc::__WASI_ESUCCESS, "creating a directory");

    // Test that unlinking it fails.
    status = libc::__wasi_path_unlink_file(
        dir_fd,
        "dir".as_ptr() as *const libc::c_char,
        "dir".len(),
    );
    assert_eq!(
        status,
        libc::__WASI_EISDIR,
        "unlink_file on a directory should fail"
    );

    // Clean up.
    cleanup_dir(dir_fd, "dir");
}

unsafe fn test_remove_nonempty_directory(dir_fd: libc::__wasi_fd_t) {
    // Create a directory in the scratch directory.
    let mut status: libc::__wasi_errno_t = libc::__wasi_path_create_directory(
        dir_fd,
        "dir".as_ptr() as *const libc::c_char,
        "dir".len(),
    );
    assert_eq!(status, libc::__WASI_ESUCCESS, "opening a directory");

    // Create a directory in the directory we just created.
    status = libc::__wasi_path_create_directory(
        dir_fd,
        "dir/nested".as_ptr() as *const libc::c_char,
        "dir/nested".len(),
    );
    assert_eq!(status, libc::__WASI_ESUCCESS, "opening a directory");

    // Test that attempting to unlink the first directory returns the expected error code.
    status = libc::__wasi_path_remove_directory(
        dir_fd,
        "dir".as_ptr() as *const libc::c_char,
        "dir".len(),
    );
    assert_eq!(
        status,
        libc::__WASI_ENOTEMPTY,
        "remove_directory on a directory should return ENOTEMPTY",
    );

    // Removing the directories.
    status = libc::__wasi_path_remove_directory(
        dir_fd,
        "dir/nested".as_ptr() as *const libc::c_char,
        "dir/nested".len(),
    );
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "remove_directory on a nested directory should succeed",
    );
    status = libc::__wasi_path_remove_directory(
        dir_fd,
        "dir".as_ptr() as *const libc::c_char,
        "dir".len(),
    );
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "remove_directory on a directory should succeed"
    );
}

unsafe fn test_interesting_paths(dir_fd: libc::__wasi_fd_t, arg: &str) {
    // Create a directory in the scratch directory.
    let mut status: libc::__wasi_errno_t = libc::__wasi_path_create_directory(
        dir_fd,
        "dir".as_ptr() as *const libc::c_char,
        "dir".len(),
    );
    assert_eq!(status, libc::__WASI_ESUCCESS, "opening a directory");

    // Create a directory in the directory we just created.
    status = libc::__wasi_path_create_directory(
        dir_fd,
        "dir/nested".as_ptr() as *const libc::c_char,
        "dir/nested".len(),
    );
    assert_eq!(status, libc::__WASI_ESUCCESS, "opening a directory");

    // Create a file in the nested directory.
    let mut file_fd: libc::__wasi_fd_t = libc::__wasi_fd_t::max_value() - 1;
    status = libc::__wasi_path_open(
        dir_fd,
        0,
        "dir/nested/file".as_ptr() as *const libc::c_char,
        "dir/nested/file".len(),
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
    assert_eq!(
        libc::__wasi_fd_close(file_fd),
        libc::__WASI_ESUCCESS,
        "closing a file"
    );

    // Now open it with an absolute path.
    status = libc::__wasi_path_open(
        dir_fd,
        0,
        "/dir/nested/file".as_ptr() as *const libc::c_char,
        "/dir/nested/file".len(),
        0,
        0,
        0,
        0,
        &mut file_fd,
    );
    assert_eq!(
        status,
        libc::__WASI_ENOTCAPABLE,
        "opening a file with an absolute path"
    );
    assert_eq!(
        file_fd,
        libc::__wasi_fd_t::max_value(),
        "failed open should set the file descriptor to -1",
    );

    // Now open it with a path containing "..".
    status = libc::__wasi_path_open(
        dir_fd,
        0,
        "dir/.//nested/../../dir/nested/../nested///./file".as_ptr() as *const libc::c_char,
        "dir/.//nested/../../dir/nested/../nested///./file".len(),
        0,
        0,
        0,
        0,
        &mut file_fd,
    );
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "opening a file with \"..\" in the path"
    );
    assert!(
        file_fd > libc::STDERR_FILENO as libc::__wasi_fd_t,
        "file descriptor range check",
    );
    assert_eq!(
        libc::__wasi_fd_close(file_fd),
        libc::__WASI_ESUCCESS,
        "closing a file"
    );

    // Now open it with a trailing NUL.
    status = libc::__wasi_path_open(
        dir_fd,
        0,
        "dir/nested/file\0".as_ptr() as *const libc::c_char,
        "dir/nested/file\0".len(),
        0,
        0,
        0,
        0,
        &mut file_fd,
    );
    assert_eq!(
        status,
        libc::__WASI_EILSEQ,
        "opening a file with a trailing NUL"
    );
    assert_eq!(
        file_fd,
        libc::__wasi_fd_t::max_value(),
        "failed open should set the file descriptor to -1",
    );

    // Now open it with a trailing slash.
    status = libc::__wasi_path_open(
        dir_fd,
        0,
        "dir/nested/file/".as_ptr() as *const libc::c_char,
        "dir/nested/file/".len(),
        0,
        0,
        0,
        0,
        &mut file_fd,
    );
    assert_eq!(
        status,
        libc::__WASI_ENOTDIR,
        "opening a file with a trailing slash"
    );
    assert_eq!(
        file_fd,
        libc::__wasi_fd_t::max_value(),
        "failed open should set the file descriptor to -1",
    );

    // Now open the directory with a trailing slash.
    status = libc::__wasi_path_open(
        dir_fd,
        0,
        "dir/nested/".as_ptr() as *const libc::c_char,
        "dir/nested/".len(),
        0,
        0,
        0,
        0,
        &mut file_fd,
    );
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "opening a directory with a trailing slash"
    );
    assert!(
        file_fd > libc::STDERR_FILENO as libc::__wasi_fd_t,
        "file descriptor range check",
    );
    assert_eq!(
        libc::__wasi_fd_close(file_fd),
        libc::__WASI_ESUCCESS,
        "closing a file"
    );

    // Now open it with a path containing too many ".."s.
    let bad_path = format!("dir/nested/../../../{}/dir/nested/file", arg);
    status = libc::__wasi_path_open(
        dir_fd,
        0,
        bad_path.as_ptr() as *const libc::c_char,
        bad_path.len(),
        0,
        0,
        0,
        0,
        &mut file_fd,
    );
    assert_eq!(
        status,
        libc::__WASI_ENOTCAPABLE,
        "opening a file with too many \"..\"s in the path"
    );
    assert_eq!(
        file_fd,
        libc::__wasi_fd_t::max_value(),
        "failed open should set the file descriptor to -1",
    );
    status = libc::__wasi_path_unlink_file(
        dir_fd,
        "dir/nested/file".as_ptr() as *const libc::c_char,
        "dir/nested/file".len(),
    );
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "unlink_file on a symlink should succeed"
    );
    status = libc::__wasi_path_remove_directory(
        dir_fd,
        "dir/nested".as_ptr() as *const libc::c_char,
        "dir/nested".len(),
    );
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "remove_directory on a directory should succeed"
    );
    status = libc::__wasi_path_remove_directory(
        dir_fd,
        "dir".as_ptr() as *const libc::c_char,
        "dir".len(),
    );
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "remove_directory on a directory should succeed"
    );
}

unsafe fn test_symlink_loop(dir_fd: libc::__wasi_fd_t) {
    // Create a self-referencing symlink.
    let mut status: libc::__wasi_errno_t = libc::__wasi_path_symlink(
        "symlink".as_ptr() as *const libc::c_char,
        "symlink".len(),
        dir_fd,
        "symlink".as_ptr() as *const libc::c_char,
        "symlink".len(),
    );
    assert_eq!(status, libc::__WASI_ESUCCESS, "creating a symlink");

    // Try to open it.
    let mut file_fd: libc::__wasi_fd_t = libc::__wasi_fd_t::max_value() - 1;
    status = libc::__wasi_path_open(
        dir_fd,
        0,
        "symlink".as_ptr() as *const libc::c_char,
        "symlink".len(),
        0,
        0,
        0,
        0,
        &mut file_fd,
    );
    assert_eq!(
        status,
        libc::__WASI_ELOOP,
        "opening a self-referencing symlink",
    );

    // Clean up.
    cleanup_file(dir_fd, "symlink");
}

unsafe fn test_nofollow_errors(dir_fd: libc::__wasi_fd_t) {
    // First create a dangling symlink.
    let mut status: libc::__wasi_errno_t = libc::__wasi_path_symlink(
        "target".as_ptr() as *const libc::c_char,
        "target".len(),
        dir_fd,
        "symlink".as_ptr() as *const libc::c_char,
        "symlink".len(),
    );
    assert_eq!(status, libc::__WASI_ESUCCESS, "creating a symlink");

    // Try to open it as a directory with O_NOFOLLOW.
    let mut file_fd: libc::__wasi_fd_t = libc::__wasi_fd_t::max_value() - 1;
    status = libc::__wasi_path_open(
        dir_fd,
        0,
        "symlink".as_ptr() as *const libc::c_char,
        "symlink".len(),
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
    status = libc::__wasi_path_create_directory(
        dir_fd,
        "target".as_ptr() as *const libc::c_char,
        "target".len(),
    );
    assert_eq!(status, libc::__WASI_ESUCCESS, "creating a directory");

    // Try to open it as a directory with O_NOFOLLOW again.
    status = libc::__wasi_path_open(
        dir_fd,
        0,
        "symlink".as_ptr() as *const libc::c_char,
        "symlink".len(),
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
    status = libc::__wasi_path_open(
        dir_fd,
        0,
        "symlink".as_ptr() as *const libc::c_char,
        "symlink".len(),
        0,
        0,
        0,
        0,
        &mut file_fd,
    );
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
    status = libc::__wasi_path_open(
        dir_fd,
        libc::__WASI_LOOKUP_SYMLINK_FOLLOW,
        "symlink".as_ptr() as *const libc::c_char,
        "symlink".len(),
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
    assert_eq!(
        libc::__wasi_fd_close(file_fd),
        libc::__WASI_ESUCCESS,
        "closing a file"
    );

    // Replace the target directory with a file.
    status = libc::__wasi_path_remove_directory(
        dir_fd,
        "target".as_ptr() as *const libc::c_char,
        "target".len(),
    );
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "remove_directory on a directory should succeed"
    );
    status = libc::__wasi_path_open(
        dir_fd,
        0,
        "target".as_ptr() as *const libc::c_char,
        "target".len(),
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
    assert_eq!(
        libc::__wasi_fd_close(file_fd),
        libc::__WASI_ESUCCESS,
        "closing a file"
    );

    // Try to open it as a directory with O_NOFOLLOW again.
    status = libc::__wasi_path_open(
        dir_fd,
        0,
        "symlink".as_ptr() as *const libc::c_char,
        "symlink".len(),
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
    status = libc::__wasi_path_open(
        dir_fd,
        0,
        "symlink".as_ptr() as *const libc::c_char,
        "symlink".len(),
        0,
        0,
        0,
        0,
        &mut file_fd,
    );
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
    status = libc::__wasi_path_open(
        dir_fd,
        libc::__WASI_LOOKUP_SYMLINK_FOLLOW,
        "symlink".as_ptr() as *const libc::c_char,
        "symlink".len(),
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

unsafe fn test_close_preopen(dir_fd: libc::__wasi_fd_t) {
    let pre_fd: libc::__wasi_fd_t = (libc::STDERR_FILENO + 1) as libc::__wasi_fd_t;

    assert!(dir_fd > pre_fd, "dir_fd number");

    // Try to close a preopened directory handle.
    let mut status: libc::__wasi_errno_t = libc::__wasi_fd_close(pre_fd);
    assert_eq!(
        status,
        libc::__WASI_ENOTSUP,
        "closing a preopened file descriptor",
    );

    // Try to renumber over a preopened directory handle.
    status = libc::__wasi_fd_renumber(dir_fd, pre_fd);
    assert_eq!(
        status,
        libc::__WASI_ENOTSUP,
        "renumbering over a preopened file descriptor",
    );

    // Ensure that dir_fd is still open.
    let mut dir_fdstat: libc::__wasi_fdstat_t = mem::zeroed();
    status = libc::__wasi_fd_fdstat_get(dir_fd, &mut dir_fdstat);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "calling fd_fdstat on the scratch directory"
    );
    assert!(
        dir_fdstat.fs_filetype == libc::__WASI_FILETYPE_DIRECTORY,
        "expected the scratch directory to be a directory",
    );

    // Try to renumber a preopened directory handle.
    status = libc::__wasi_fd_renumber(pre_fd, dir_fd);
    assert_eq!(
        status,
        libc::__WASI_ENOTSUP,
        "renumbering over a preopened file descriptor",
    );

    // Ensure that dir_fd is still open.
    status = libc::__wasi_fd_fdstat_get(dir_fd, &mut dir_fdstat);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "calling fd_fdstat on the scratch directory"
    );
    assert!(
        dir_fdstat.fs_filetype == libc::__WASI_FILETYPE_DIRECTORY,
        "expected the scratch directory to be a directory",
    );
}

unsafe fn test_clock_time_get() {
    // Test that clock_time_get succeeds. Even in environments where it's not
    // desirable to expose high-precision timers, it should still succeed.
    // clock_res_get is where information about precision can be provided.
    let mut time: libc::__wasi_timestamp_t = 0;
    let mut status: libc::__wasi_errno_t =
        libc::__wasi_clock_time_get(libc::__WASI_CLOCK_MONOTONIC, 0, &mut time);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "clock_time_get with a precision of 0"
    );

    status = libc::__wasi_clock_time_get(libc::__WASI_CLOCK_MONOTONIC, 1, &mut time);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "clock_time_get with a precision of 1"
    );
}

unsafe fn test_readlink_no_buffer(dir_fd: libc::__wasi_fd_t) {
    // First create a dangling symlink.
    let mut status: libc::__wasi_errno_t = libc::__wasi_path_symlink(
        "target".as_ptr() as *const libc::c_char,
        "target".len(),
        dir_fd,
        "symlink".as_ptr() as *const libc::c_char,
        "symlink".len(),
    );
    assert_eq!(status, libc::__WASI_ESUCCESS, "creating a symlink");

    // Readlink it into a non-existent buffer.
    let mut bufused: usize = 1;
    status = libc::__wasi_path_readlink(
        dir_fd,
        "symlink".as_ptr() as *const libc::c_char,
        "symlink".len(),
        ptr::null_mut(),
        0,
        &mut bufused,
    );
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "readlink with a 0-sized buffer should succeed"
    );

    // Clean up.
    cleanup_file(dir_fd, "symlink");
}

unsafe fn test_isatty(dir_fd: libc::__wasi_fd_t) {
    // Create a file in the scratch directory and test if it's a tty.
    let mut file_fd: libc::__wasi_fd_t = libc::__wasi_fd_t::max_value() - 1;
    let status = libc::__wasi_path_open(
        dir_fd,
        0,
        "file".as_ptr() as *const libc::c_char,
        "file".len(),
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
    assert_eq!(libc::isatty(file_fd as libc::c_int), 0, "file is a tty");
    assert_eq!(
        libc::__wasi_fd_close(file_fd),
        libc::__WASI_ESUCCESS,
        "closing a file"
    );

    let status =
        libc::__wasi_path_unlink_file(dir_fd, "file".as_ptr() as *const libc::c_char, "file".len());
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "unlink_file on a file should succeed"
    );
}

unsafe fn test_directory_seek(dir_fd: libc::__wasi_fd_t) {
    // Create a directory in the scratch directory.
    let mut status: libc::__wasi_errno_t = libc::__wasi_path_create_directory(
        dir_fd,
        "dir".as_ptr() as *const libc::c_char,
        "dir".len(),
    );
    assert_eq!(status, libc::__WASI_ESUCCESS, "creating a directory");

    // Open the directory and attempt to request rights for seeking.
    let mut fd: libc::__wasi_fd_t = libc::__wasi_fd_t::max_value() - 1;
    status = libc::__wasi_path_open(
        dir_fd,
        0,
        "dir".as_ptr() as *const libc::c_char,
        "dir".len(),
        0,
        libc::__WASI_RIGHT_FD_SEEK,
        0,
        0,
        &mut fd,
    );
    assert_eq!(status, libc::__WASI_ESUCCESS, "opening a file");
    assert!(
        fd > libc::STDERR_FILENO as libc::__wasi_fd_t,
        "file descriptor range check",
    );

    // Attempt to seek.
    let mut newoffset = 1;
    status = libc::__wasi_fd_seek(fd, 0, libc::__WASI_WHENCE_CUR, &mut newoffset);
    assert_eq!(status, libc::__WASI_ENOTCAPABLE, "seek on a directory");

    // Check if we obtained the right to seek.
    let mut fdstat: libc::__wasi_fdstat_t = mem::zeroed();
    status = libc::__wasi_fd_fdstat_get(fd, &mut fdstat);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "calling fd_fdstat on a directory"
    );
    assert!(
        fdstat.fs_filetype == libc::__WASI_FILETYPE_DIRECTORY,
        "expected the scratch directory to be a directory",
    );
    assert_eq!(
        (fdstat.fs_rights_base & libc::__WASI_RIGHT_FD_SEEK), 0,
        "directory has the seek right",
    );

    // Clean up.
    assert_eq!(
        libc::__wasi_fd_close(fd),
        libc::__WASI_ESUCCESS,
        "closing a directory"
    );
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

    // Open the scratch directory.
    let dir_fd: libc::__wasi_fd_t = unsafe {
        libc::open(
            CString::new(arg.as_bytes()).unwrap().as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY,
        )
    } as libc::__wasi_fd_t;
    if (dir_fd as libc::c_int) < 0 {
        eprintln!(
            "error opening scratch directory '{}': {}",
            arg,
            errno::errno()
        );
        process::exit(1);
    }

    // Run the tests.
    unsafe {
        test_sched_yield();
        test_truncation_rights(dir_fd);
        test_unlink_directory(dir_fd);
        test_remove_nonempty_directory(dir_fd);
        test_interesting_paths(dir_fd, &arg);
        test_nofollow_errors(dir_fd);
        test_symlink_loop(dir_fd);
        test_close_preopen(dir_fd);
        test_clock_time_get();
        test_readlink_no_buffer(dir_fd);
        test_isatty(dir_fd);
        test_directory_seek(dir_fd);
    }
}

//! Minimal wrappers around WASI functions to allow use of `&str` rather than
//! pointer-length pairs, and to convert out parameters to multiple return
//! values.

use libc;

pub fn wasi_path_create_directory(
    dir_fd: libc::__wasi_fd_t,
    dir_name: &str,
) -> libc::__wasi_errno_t {
    unsafe {
        libc::__wasi_path_create_directory(
            dir_fd,
            dir_name.as_ptr() as *const libc::c_char,
            dir_name.len(),
        )
    }
}

pub fn wasi_path_remove_directory(
    dir_fd: libc::__wasi_fd_t,
    dir_name: &str,
) -> libc::__wasi_errno_t {
    unsafe {
        libc::__wasi_path_remove_directory(
            dir_fd,
            dir_name.as_ptr() as *const libc::c_char,
            dir_name.len(),
        )
    }
}

pub fn wasi_path_unlink_file(dir_fd: libc::__wasi_fd_t, file_name: &str) -> libc::__wasi_errno_t {
    unsafe {
        libc::__wasi_path_unlink_file(
            dir_fd,
            file_name.as_ptr() as *const libc::c_char,
            file_name.len(),
        )
    }
}

pub fn wasi_sched_yield() -> libc::__wasi_errno_t {
    unsafe { libc::__wasi_sched_yield() }
}

pub fn wasi_path_open(
    dirfd: libc::__wasi_fd_t,
    dirflags: libc::__wasi_lookupflags_t,
    path: &str,
    oflags: libc::__wasi_oflags_t,
    fs_rights_base: libc::__wasi_rights_t,
    fs_rights_inheriting: libc::__wasi_rights_t,
    fs_flags: libc::__wasi_fdflags_t,
    fd: &mut libc::__wasi_fd_t,
) -> libc::__wasi_errno_t {
    unsafe {
        libc::__wasi_path_open(
            dirfd,
            dirflags,
            path.as_ptr() as *const libc::c_char,
            path.len(),
            oflags,
            fs_rights_base,
            fs_rights_inheriting,
            fs_flags,
            fd,
        )
    }
}

pub fn wasi_fd_close(fd: libc::__wasi_fd_t) -> libc::__wasi_errno_t {
    unsafe { libc::__wasi_fd_close(fd) }
}

pub fn wasi_path_symlink(
    old_path: &str,
    dirfd: libc::__wasi_fd_t,
    new_path: &str,
) -> libc::__wasi_errno_t {
    unsafe {
        libc::__wasi_path_symlink(
            old_path.as_ptr() as *const libc::c_char,
            old_path.len(),
            dirfd,
            new_path.as_ptr() as *const libc::c_char,
            new_path.len(),
        )
    }
}

pub fn wasi_path_readlink(
    dirfd: libc::__wasi_fd_t,
    path: &str,
    buf: &mut [u8],
    bufused: &mut usize,
) -> libc::__wasi_errno_t {
    unsafe {
        libc::__wasi_path_readlink(
            dirfd,
            path.as_ptr() as *const libc::c_char,
            path.len(),
            buf.as_ptr() as *mut libc::c_char,
            buf.len(),
            bufused,
        )
    }
}

pub fn wasi_path_rename(
    old_dirfd: libc::__wasi_fd_t,
    old_path: &str,
    new_dirfd: libc::__wasi_fd_t,
    new_path: &str,
) -> libc::__wasi_errno_t {
    unsafe {
        libc::__wasi_path_rename(
            old_dirfd,
            old_path.as_ptr() as *const _,
            old_path.len(),
            new_dirfd,
            new_path.as_ptr() as *const _,
            new_path.len(),
        )
    }
}

pub fn wasi_fd_fdstat_get(
    fd: libc::__wasi_fd_t,
    fdstat: &mut libc::__wasi_fdstat_t,
) -> libc::__wasi_errno_t {
    unsafe { libc::__wasi_fd_fdstat_get(fd, fdstat) }
}

pub fn wasi_fd_renumber(from: libc::__wasi_fd_t, to: libc::__wasi_fd_t) -> libc::__wasi_errno_t {
    unsafe { libc::__wasi_fd_renumber(from, to) }
}

pub fn wasi_fd_fdstat_set_rights(
    fd: libc::__wasi_fd_t,
    fs_rights_base: libc::__wasi_rights_t,
    fs_rights_inheriting: libc::__wasi_rights_t,
) -> libc::__wasi_errno_t {
    unsafe { libc::__wasi_fd_fdstat_set_rights(fd, fs_rights_base, fs_rights_inheriting) }
}

pub fn wasi_fd_seek(
    fd: libc::__wasi_fd_t,
    offset: libc::__wasi_filedelta_t,
    whence: libc::__wasi_whence_t,
    newoffset: &mut libc::__wasi_filesize_t,
) -> libc::__wasi_errno_t {
    unsafe { libc::__wasi_fd_seek(fd, offset, whence, newoffset) }
}

pub fn wasi_fd_tell(
    fd: libc::__wasi_fd_t,
    offset: &mut libc::__wasi_filesize_t,
) -> libc::__wasi_errno_t {
    unsafe { libc::__wasi_fd_tell(fd, offset) }
}

pub fn wasi_clock_time_get(
    clock_id: libc::__wasi_clockid_t,
    precision: libc::__wasi_timestamp_t,
    time: &mut libc::__wasi_timestamp_t,
) -> libc::__wasi_errno_t {
    unsafe { libc::__wasi_clock_time_get(clock_id, precision, time) }
}

pub fn wasi_fd_filestat_get(
    fd: libc::__wasi_fd_t,
    filestat: &mut libc::__wasi_filestat_t,
) -> libc::__wasi_errno_t {
    unsafe { libc::__wasi_fd_filestat_get(fd, filestat) }
}

pub fn wasi_fd_filestat_set_size(
    fd: libc::__wasi_fd_t,
    st_size: libc::__wasi_filesize_t,
) -> libc::__wasi_errno_t {
    unsafe { libc::__wasi_fd_filestat_set_size(fd, st_size) }
}

pub fn wasi_fd_filestat_set_times(
    fd: libc::__wasi_fd_t,
    st_atim: libc::__wasi_timestamp_t,
    st_mtim: libc::__wasi_timestamp_t,
    fst_flags: libc::__wasi_fstflags_t,
) -> libc::__wasi_errno_t {
    unsafe { libc::__wasi_fd_filestat_set_times(fd, st_atim, st_mtim, fst_flags) }
}

pub fn wasi_fd_allocate(
    fd: libc::__wasi_fd_t,
    offset: libc::__wasi_filesize_t,
    len: libc::__wasi_filesize_t,
) -> libc::__wasi_errno_t {
    unsafe { libc::__wasi_fd_allocate(fd, offset, len) }
}

pub fn wasi_fd_write(
    fd: libc::__wasi_fd_t,
    iovs: &[libc::__wasi_ciovec_t],
    nwritten: &mut libc::size_t,
) -> libc::__wasi_errno_t {
    unsafe { libc::__wasi_fd_write(fd, iovs.as_ptr(), iovs.len(), nwritten) }
}

pub fn wasi_fd_read(
    fd: libc::__wasi_fd_t,
    iovs: &[libc::__wasi_iovec_t],
    nread: &mut libc::size_t,
) -> libc::__wasi_errno_t {
    unsafe { libc::__wasi_fd_read(fd, iovs.as_ptr(), iovs.len(), nread) }
}

pub fn wasi_fd_pread(
    fd: libc::__wasi_fd_t,
    iovs: &[libc::__wasi_iovec_t],
    offset: libc::__wasi_filesize_t,
    nread: &mut usize,
) -> libc::__wasi_errno_t {
    unsafe { libc::__wasi_fd_pread(fd, iovs.as_ptr(), iovs.len(), offset, nread) }
}

pub fn wasi_fd_pwrite(
    fd: libc::__wasi_fd_t,
    iovs: &mut [libc::__wasi_ciovec_t],
    offset: libc::__wasi_filesize_t,
    nwritten: &mut usize,
) -> libc::__wasi_errno_t {
    unsafe { libc::__wasi_fd_pwrite(fd, iovs.as_ptr(), iovs.len(), offset, nwritten) }
}

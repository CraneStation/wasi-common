//! Minimal wrappers around WASI functions to allow use of `&str` rather than
//! pointer-length pairs.

use wasi::wasi_unstable;

pub fn wasi_path_create_directory(
    dir_fd: wasi_unstable::Fd,
    dir_name: &str,
) -> wasi_unstable::Errno {
    wasi_unstable::path_create_directory(dir_fd, dir_name.as_bytes())
}

pub fn wasi_path_remove_directory(
    dir_fd: wasi_unstable::Fd,
    dir_name: &str,
) -> wasi_unstable::Errno {
    wasi_unstable::path_remove_directory(dir_fd, dir_name.as_bytes())
}

pub fn wasi_path_unlink_file(dir_fd: wasi_unstable::Fd, file_name: &str) -> wasi_unstable::Errno {
    wasi_unstable::path_unlink_file(dir_fd, file_name.as_bytes())
}

#[allow(clippy::too_many_arguments)]
pub fn wasi_path_open(
    dirfd: wasi_unstable::Fd,
    dirflags: wasi_unstable::LookupFlags,
    path: &str,
    oflags: wasi_unstable::OFlags,
    fs_rights_base: wasi_unstable::Rights,
    fs_rights_inheriting: wasi_unstable::Rights,
    fs_flags: wasi_unstable::FdFlags,
    fd: &mut wasi_unstable::Fd,
) -> wasi_unstable::Errno {
    unsafe {
        wasi_unstable::raw::__wasi_path_open(
            dirfd,
            dirflags,
            path.as_ptr(),
            path.len(),
            oflags,
            fs_rights_base,
            fs_rights_inheriting,
            fs_flags,
            fd,
        )
    }
}

pub fn wasi_path_symlink(
    old_path: &str,
    dirfd: wasi_unstable::Fd,
    new_path: &str,
) -> wasi_unstable::Errno {
    wasi_unstable::path_symlink(old_path.as_bytes(), dirfd, new_path.as_bytes())
}

pub fn wasi_path_readlink(
    dirfd: wasi_unstable::Fd,
    path: &str,
    buf: &mut [u8],
    bufused: &mut usize,
) -> wasi_unstable::Errno {
    unsafe {
        wasi_unstable::raw::__wasi_path_readlink(
            dirfd,
            path.as_ptr(),
            path.len(),
            buf.as_mut_ptr(),
            buf.len(),
            bufused,
        )
    }
}

pub fn wasi_path_rename(
    old_dirfd: wasi_unstable::Fd,
    old_path: &str,
    new_dirfd: wasi_unstable::Fd,
    new_path: &str,
) -> wasi_unstable::Errno {
    unsafe {
        wasi_unstable::raw::__wasi_path_rename(
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
    fd: wasi_unstable::Fd,
    fdstat: &mut wasi_unstable::FdStat,
) -> wasi_unstable::Errno {
    unsafe { wasi_unstable::raw::__wasi_fd_fdstat_get(fd, fdstat) }
}

pub fn wasi_fd_seek(
    fd: wasi_unstable::Fd,
    offset: wasi_unstable::FileDelta,
    whence: wasi_unstable::Whence,
    newoffset: &mut wasi_unstable::FileSize,
) -> wasi_unstable::Errno {
    unsafe { wasi_unstable::raw::__wasi_fd_seek(fd, offset, whence, newoffset) }
}

pub fn wasi_fd_tell(
    fd: wasi_unstable::Fd,
    offset: &mut wasi_unstable::FileSize,
) -> wasi_unstable::Errno {
    unsafe { wasi_unstable::raw::__wasi_fd_tell(fd, offset) }
}

pub fn wasi_clock_time_get(
    clock_id: wasi_unstable::ClockId,
    precision: wasi_unstable::Timestamp,
    time: &mut wasi_unstable::Timestamp,
) -> wasi_unstable::Errno {
    unsafe { wasi_unstable::raw::__wasi_clock_time_get(clock_id, precision, time) }
}

pub fn wasi_fd_filestat_get(
    fd: wasi_unstable::Fd,
    filestat: &mut wasi_unstable::FileStat,
) -> wasi_unstable::Errno {
    unsafe { wasi_unstable::raw::__wasi_fd_filestat_get(fd, filestat) }
}

pub fn wasi_fd_write(
    fd: wasi_unstable::Fd,
    iovs: &[wasi_unstable::CIoVec],
    nwritten: &mut libc::size_t,
) -> wasi_unstable::Errno {
    unsafe { wasi_unstable::raw::__wasi_fd_write(fd, iovs.as_ptr(), iovs.len(), nwritten) }
}

pub fn wasi_fd_read(
    fd: wasi_unstable::Fd,
    iovs: &[wasi_unstable::IoVec],
    nread: &mut libc::size_t,
) -> wasi_unstable::Errno {
    unsafe { wasi_unstable::raw::__wasi_fd_read(fd, iovs.as_ptr(), iovs.len(), nread) }
}

pub fn wasi_fd_pread(
    fd: wasi_unstable::Fd,
    iovs: &[wasi_unstable::IoVec],
    offset: wasi_unstable::FileSize,
    nread: &mut usize,
) -> wasi_unstable::Errno {
    unsafe { wasi_unstable::raw::__wasi_fd_pread(fd, iovs.as_ptr(), iovs.len(), offset, nread) }
}

pub fn wasi_fd_pwrite(
    fd: wasi_unstable::Fd,
    iovs: &mut [wasi_unstable::CIoVec],
    offset: wasi_unstable::FileSize,
    nwritten: &mut usize,
) -> wasi_unstable::Errno {
    unsafe { wasi_unstable::raw::__wasi_fd_pwrite(fd, iovs.as_ptr(), iovs.len(), offset, nwritten) }
}

pub fn wasi_path_filestat_get(
    fd: libc::__wasi_fd_t,
    dirflags: libc::__wasi_lookupflags_t,
    path: &str,
    path_len: usize,
    filestat: &mut libc::__wasi_filestat_t,
) -> libc::__wasi_errno_t {
    unsafe {
        libc::__wasi_path_filestat_get(
            fd,
            dirflags,
            path.as_ptr() as *const libc::c_char,
            path_len,
            filestat,
        )
    }
}

pub fn wasi_path_filestat_set_times(
    fd: libc::__wasi_fd_t,
    dirflags: libc::__wasi_lookupflags_t,
    path: &str,
    path_len: usize,
    st_atim: libc::__wasi_timestamp_t,
    st_mtim: libc::__wasi_timestamp_t,
    fst_flags: libc::__wasi_fstflags_t,
) -> libc::__wasi_errno_t {
    unsafe {
        libc::__wasi_path_filestat_set_times(
            fd,
            dirflags,
            path.as_ptr() as *const libc::c_char,
            path_len,
            st_atim,
            st_mtim,
            fst_flags,
        )
    }
}

pub fn wasi_fd_readdir(
    fd: wasi_unstable::Fd,
    buf: &mut [u8],
    buf_len: usize,
    cookie: wasi_unstable::DirCookie,
    buf_used: &mut usize,
) -> wasi_unstable::Errno {
    unsafe {
        wasi_unstable::raw::__wasi_fd_readdir(
            fd,
            buf.as_mut_ptr() as *mut libc::c_void,
            buf_len,
            cookie,
            buf_used,
        )
    }
}

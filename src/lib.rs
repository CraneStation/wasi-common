pub mod utils;
pub mod wasi;

use errno;
use libc;
use std::ffi::CString;

pub fn open_scratch_directory(path: &str) -> Result<libc::__wasi_fd_t, String> {
    // Open the scratch directory.
    let dir_fd: libc::__wasi_fd_t = unsafe {
        libc::open(
            CString::new(path.as_bytes()).unwrap().as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY,
        )
    } as libc::__wasi_fd_t;

    if (dir_fd as libc::c_int) < 0 {
        Err(format!(
            "error opening scratch directory '{}': {}",
            path,
            errno::errno()
        ))
    } else {
        Ok(dir_fd)
    }
}

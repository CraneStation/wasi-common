use libc;
use misc_tests::open_scratch_directory;
use misc_tests::utils::{cleanup_file, close_fd};
use misc_tests::wasi::{wasi_fd_fdstat_get, wasi_fd_fdstat_set_rights, wasi_path_open};
use std::{env, mem, process};

fn test_truncation_rights(dir_fd: libc::__wasi_fd_t) {
    // Create a file in the scratch directory.
    let mut file_fd = libc::__wasi_fd_t::max_value() - 1;
    let mut status = wasi_path_open(
        dir_fd,
        0,
        "file",
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

    // Get the rights for the scratch directory.
    let mut dir_fdstat: libc::__wasi_fdstat_t = unsafe { mem::zeroed() };
    status = wasi_fd_fdstat_get(dir_fd, &mut dir_fdstat);
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
    assert!(
        (dir_fdstat.fs_rights_base & libc::__WASI_RIGHT_FD_FILESTAT_SET_SIZE) == 0,
        "directories shouldn't have the fd_filestat_set_size right",
    );

    // If we have the right to set sizes from paths, test that it works.
    if (dir_fdstat.fs_rights_base & libc::__WASI_RIGHT_PATH_FILESTAT_SET_SIZE) == 0 {
        eprintln!("implementation doesn't support setting file sizes, skipping");
    } else {
        // Test that we can truncate the file.
        status = wasi_path_open(
            dir_fd,
            0,
            "file",
            libc::__WASI_O_TRUNC,
            0,
            0,
            0,
            &mut file_fd,
        );
        assert_eq!(status, libc::__WASI_ESUCCESS, "truncating a file");
        close_fd(file_fd);

        let mut rights_base: libc::__wasi_rights_t = dir_fdstat.fs_rights_base;
        let mut rights_inheriting: libc::__wasi_rights_t = dir_fdstat.fs_rights_inheriting;

        if (rights_inheriting & libc::__WASI_RIGHT_FD_FILESTAT_SET_SIZE) == 0 {
            eprintln!("implementation doesn't support setting file sizes through file descriptors, skipping");
        } else {
            rights_inheriting &= !libc::__WASI_RIGHT_FD_FILESTAT_SET_SIZE;
            status = wasi_fd_fdstat_set_rights(dir_fd, rights_base, rights_inheriting);
            assert_eq!(
                status,
                libc::__WASI_ESUCCESS,
                "droping fd_filestat_set_size inheriting right on a directory",
            );
        }

        // Test that we can truncate the file without the
        // __WASI_RIGHT_FD_FILESTAT_SET_SIZE right.
        status = wasi_path_open(
            dir_fd,
            0,
            "file",
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
        close_fd(file_fd);

        rights_base &= !libc::__WASI_RIGHT_PATH_FILESTAT_SET_SIZE;
        status = wasi_fd_fdstat_set_rights(dir_fd, rights_base, rights_inheriting);
        assert_eq!(
            status,
            libc::__WASI_ESUCCESS,
            "droping path_filestat_set_size base right on a directory",
        );

        // Test that clearing __WASI_RIGHT_PATH_FILESTAT_SET_SIZE actually
        // took effect.
        status = wasi_fd_fdstat_get(dir_fd, &mut dir_fdstat);
        assert_eq!(
            status,
            libc::__WASI_ESUCCESS,
            "reading the fdstat from a directory",
        );
        assert_eq!(
            (dir_fdstat.fs_rights_base & libc::__WASI_RIGHT_PATH_FILESTAT_SET_SIZE),
            0,
            "reading the fdstat from a directory",
        );

        // Test that we can't truncate the file without the
        // __WASI_RIGHT_PATH_FILESTAT_SET_SIZE right.
        status = wasi_path_open(
            dir_fd,
            0,
            "file",
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

    cleanup_file(dir_fd, "file");
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
    test_truncation_rights(dir_fd)
}

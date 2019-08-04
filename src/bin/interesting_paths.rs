use libc;
use misc_tests::open_scratch_directory;
use misc_tests::utils::{close_fd, create_dir};
use misc_tests::wasi::{wasi_path_open, wasi_path_remove_directory, wasi_path_unlink_file};
use std::{env, process};

fn test_interesting_paths(dir_fd: libc::__wasi_fd_t, arg: &str) {
    // Create a directory in the scratch directory.
    create_dir(dir_fd, "dir");

    // Create a directory in the directory we just created.
    create_dir(dir_fd, "dir/nested");

    // Create a file in the nested directory.
    let mut file_fd: libc::__wasi_fd_t = libc::__wasi_fd_t::max_value() - 1;
    let mut status = wasi_path_open(
        dir_fd,
        0,
        "dir/nested/file",
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

    // Now open it with an absolute path.
    status = wasi_path_open(dir_fd, 0, "/dir/nested/file", 0, 0, 0, 0, &mut file_fd);
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
    status = wasi_path_open(
        dir_fd,
        0,
        "dir/.//nested/../../dir/nested/../nested///./file",
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
    close_fd(file_fd);

    // Now open it with a trailing NUL.
    status = wasi_path_open(dir_fd, 0, "dir/nested/file\0", 0, 0, 0, 0, &mut file_fd);
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
    status = wasi_path_open(dir_fd, 0, "dir/nested/file/", 0, 0, 0, 0, &mut file_fd);
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

    // Now open it with trailing slashes.
    status = wasi_path_open(dir_fd, 0, "dir/nested/file///", 0, 0, 0, 0, &mut file_fd);
    assert_eq!(
        status,
        libc::__WASI_ENOTDIR,
        "opening a file with trailing slashes"
    );
    assert_eq!(
        file_fd,
        libc::__wasi_fd_t::max_value(),
        "failed open should set the file descriptor to -1",
    );

    // Now open the directory with a trailing slash.
    status = wasi_path_open(dir_fd, 0, "dir/nested/", 0, 0, 0, 0, &mut file_fd);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "opening a directory with a trailing slash"
    );
    assert!(
        file_fd > libc::STDERR_FILENO as libc::__wasi_fd_t,
        "file descriptor range check",
    );
    close_fd(file_fd);

    // Now open the directory with trailing slashes.
    status = wasi_path_open(dir_fd, 0, "dir/nested///", 0, 0, 0, 0, &mut file_fd);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "opening a directory with trailing slashes"
    );
    assert!(
        file_fd > libc::STDERR_FILENO as libc::__wasi_fd_t,
        "file descriptor range check",
    );
    close_fd(file_fd);

    // Now open it with a path containing too many ".."s.
    let bad_path = format!("dir/nested/../../../{}/dir/nested/file", arg);
    status = wasi_path_open(dir_fd, 0, &bad_path, 0, 0, 0, 0, &mut file_fd);
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
    status = wasi_path_unlink_file(dir_fd, "dir/nested/file");
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "unlink_file on a symlink should succeed"
    );
    status = wasi_path_remove_directory(dir_fd, "dir/nested");
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "remove_directory on a directory should succeed"
    );
    status = wasi_path_remove_directory(dir_fd, "dir");
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "remove_directory on a directory should succeed"
    );
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
    test_interesting_paths(dir_fd, &arg)
}

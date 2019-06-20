fn test_big_random_buf() {
    let mut buf = Vec::new();
    buf.resize(1024, 0);
    let status =
        unsafe { libc::__wasi_random_get(buf.as_mut_ptr() as *mut libc::c_void, buf.len()) };
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "calling get_random on a large buffer"
    );
    // Chances are pretty good that at least *one* byte will be non-zero in
    // any meaningful random function producing 1024 u8 values.
    assert!(buf.iter().any(|x| *x != 0), "random_get returned all zeros");
}

fn main() {
    // Run the tests.
    test_big_random_buf()
}

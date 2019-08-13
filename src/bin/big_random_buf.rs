use wasi::wasi_unstable;

fn test_big_random_buf() {
    let mut buf = Vec::new();
    buf.resize(1024, 0);
    let status = wasi_unstable::random_get(&mut buf);
    assert_eq!(
        status,
        wasi_unstable::ESUCCESS,
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

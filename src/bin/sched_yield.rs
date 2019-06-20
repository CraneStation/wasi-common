use misc_tests::wasi::wasi_sched_yield;

fn test_sched_yield() {
    let status = wasi_sched_yield();
    assert_eq!(status, libc::__WASI_ESUCCESS, "sched_yield");
}

fn main() {
    // Run tests
    test_sched_yield()
}

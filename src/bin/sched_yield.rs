use wasi::wasi_unstable;

fn test_sched_yield() {
    let status = wasi_unstable::sched_yield();
    assert_eq!(status, wasi_unstable::ESUCCESS, "sched_yield");
}

fn main() {
    // Run tests
    test_sched_yield()
}

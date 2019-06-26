use libc;
use misc_tests::wasi::wasi_clock_time_get;

fn test_clock_time_get() {
    // Test that clock_time_get succeeds. Even in environments where it's not
    // desirable to expose high-precision timers, it should still succeed.
    // clock_res_get is where information about precision can be provided.
    let mut time: libc::__wasi_timestamp_t = 0;
    let mut status = wasi_clock_time_get(libc::__WASI_CLOCK_MONOTONIC, 0, &mut time);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "clock_time_get with a precision of 0"
    );

    status = wasi_clock_time_get(libc::__WASI_CLOCK_MONOTONIC, 1, &mut time);
    assert_eq!(
        status,
        libc::__WASI_ESUCCESS,
        "clock_time_get with a precision of 1"
    );
}

fn main() {
    // Run the tests.
    test_clock_time_get()
}

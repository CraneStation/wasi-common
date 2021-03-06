//! Types and constants specific to 32-bit wasi. These are similar to the types
//! in the `host` module, but pointers and `usize` values are replaced with
//! `u32`-sized types.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use wig::witx_wasi32_types;

use crate::wasi::*;

pub type uintptr_t = u32;
pub type size_t = u32;

witx_wasi32_types!("unstable" "wasi_unstable_preview0");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bindgen_test_layout_wasi_ciovec_t() {
        assert_eq!(
            ::std::mem::size_of::<__wasi_ciovec_t>(),
            8usize,
            concat!("Size of: ", stringify!(__wasi_ciovec_t))
        );
        assert_eq!(
            ::std::mem::align_of::<__wasi_ciovec_t>(),
            4usize,
            concat!("Alignment of ", stringify!(__wasi_ciovec_t))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<__wasi_ciovec_t>())).buf as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__wasi_ciovec_t),
                "::",
                stringify!(buf)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<__wasi_ciovec_t>())).buf_len as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(__wasi_ciovec_t),
                "::",
                stringify!(buf_len)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_wasi_iovec_t() {
        assert_eq!(
            ::std::mem::size_of::<__wasi_iovec_t>(),
            8usize,
            concat!("Size of: ", stringify!(__wasi_iovec_t))
        );
        assert_eq!(
            ::std::mem::align_of::<__wasi_iovec_t>(),
            4usize,
            concat!("Alignment of ", stringify!(__wasi_iovec_t))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<__wasi_iovec_t>())).buf as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__wasi_iovec_t),
                "::",
                stringify!(buf)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<__wasi_iovec_t>())).buf_len as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(__wasi_iovec_t),
                "::",
                stringify!(buf_len)
            )
        );
    }

    #[test]
    fn bindgen_test_layout___wasi_prestat_t___wasi_prestat_u___wasi_prestat_u_dir_t() {
        assert_eq!(
            ::std::mem::size_of::<__wasi_prestat_dir>(),
            4usize,
            concat!("Size of: ", stringify!(__wasi_prestat_dir))
        );
        assert_eq!(
            ::std::mem::align_of::<__wasi_prestat_dir>(),
            4usize,
            concat!("Alignment of ", stringify!(__wasi_prestat_dir))
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<__wasi_prestat_dir>())).pr_name_len as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__wasi_prestat_dir),
                "::",
                stringify!(pr_name_len)
            )
        );
    }

    #[test]
    fn bindgen_test_layout___wasi_prestat_t___wasi_prestat_u() {
        assert_eq!(
            ::std::mem::size_of::<__wasi_prestat_u>(),
            4usize,
            concat!("Size of: ", stringify!(__wasi_prestat_u))
        );
        assert_eq!(
            ::std::mem::align_of::<__wasi_prestat_u>(),
            4usize,
            concat!("Alignment of ", stringify!(__wasi_prestat_u))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<__wasi_prestat_u>())).dir as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__wasi_prestat_u),
                "::",
                stringify!(dir)
            )
        );
    }

    #[test]
    fn bindgen_test_layout___wasi_prestat_t() {
        assert_eq!(
            ::std::mem::size_of::<__wasi_prestat_t>(),
            8usize,
            concat!("Size of: ", stringify!(__wasi_prestat_t))
        );
        assert_eq!(
            ::std::mem::align_of::<__wasi_prestat_t>(),
            4usize,
            concat!("Alignment of ", stringify!(__wasi_prestat_t))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<__wasi_prestat_t>())).pr_type as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__wasi_prestat_t),
                "::",
                stringify!(pr_type)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<__wasi_prestat_t>())).u as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(__wasi_prestat_t),
                "::",
                stringify!(u)
            )
        );
    }
}

# wasi-common
[![build-status]][actions] [![rustc-1.37]][rustc]

[build-status]: https://github.com/CraneStation/wasi-common/workflows/Continuous%20Integration/badge.svg
[actions]: https://github.com/CraneStation/wasi-common/actions
[rustc-1.37]: https://img.shields.io/badge/rustc-1.37+-lightgray.svg
[rustc]: https://blog.rust-lang.org/2019/08/15/Rust-1.37.0.html
[Wasmtime]: https://github.com/CraneStation/wasmtime
[Lucet]: https://github.com/fastly/lucet
[lucet-wasi]: https://github.com/fastly/lucet/tree/master/lucet-wasi
[lucet-wasi-tracker]: https://github.com/fastly/lucet/commit/40ae1df64536250a2b6ab67e7f167d22f4aa7f94
[WASI API]: https://github.com/CraneStation/wasmtime/blob/master/docs/WASI-api.md

**Warning: this repo is now deprecated and read-only.**
**The contents of this repo has now been moved into [Wasmtime] crate.**
**If you have an issue, you'd like to raise, please submit it there.**

This repo will ultimately serve as a library providing a common implementation of
WASI hostcalls for re-use in any WASI (and potentially non-WASI) runtimes
such as [Wasmtime] and [Lucet].

The library is an adaption of [lucet-wasi] crate from the [Lucet] project, and it is
currently based on [40ae1df][lucet-wasi-tracker] git revision.

Please note that the library requires Rust compiler version at least 1.37.0.

## Supported syscalls

### *nix
In our *nix implementation, we currently support the entire [WASI API]
with the exception of socket hostcalls:
- `sock_recv`
- `sock_send`
- `sock_shutdown`

We expect these to be implemented when network access is standardised.

We also currently do not support the `proc_raise` hostcall, as it is expected to
be dropped entirely from WASI.

### Windows
In our Windows implementation, we currently support the minimal subset of [WASI API]
which allows for running the very basic "Hello world!" style WASM apps. More coming shortly,
so stay tuned!

## Development hints
When testing the crate, you may want to enable and run full wasm32 integration testsuite. This
requires `wasm32-wasi` target installed which can be done as follows using [rustup]

```
rustup target add wasm32-wasi
```

[rustup]: https://rustup.rs

Next initiate submodules containing the integration testsuite

```
git submodule update --init
```

Now, you should be able to run the integration testsuite by enabling the `wasm_tests` feature

```
cargo test --features wasm_tests
```

## Third-Party Code
Significant parts of our hostcall implementations are derived from the C implementations in
`cloudabi-utils`. See [LICENSE.cloudabi-utils](LICENSE.cloudabi-utils) for license information.

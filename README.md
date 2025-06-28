[![Rust](https://github.com/nixberg/chacha8rand-rs/actions/workflows/rust.yaml/badge.svg)](
https://github.com/nixberg/chacha8rand-rs/actions/workflows/rust.yaml)

# chacha8rand-rs

ChaCha8Rand is a ChaCha8-based key-erasure CSPRNG with performance similar to non-cryptographic
random number generators.

Implemented using `core::simd` and `rand_core::block`.
Until `feature(portable_simd)` is stabilized, a nightly toolchain is required.

[Spec.](https://github.com/C2SP/C2SP/blob/main/chacha8rand.md)
[Go.](https://pkg.go.dev/math/rand/v2#ChaCha8)
[Swift.](https://github.com/nixberg/chacha8rand-swift)

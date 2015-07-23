Comparative benchmarking for Rust implementations of the [BLAKE2] hash functions.

[BLAKE2]: https://blake2.net/

Sample output for the following versions:

* blake2-rfc v0.2.3
* libb2-sys v0.0.2 (https://github.com/cesarb/libb2-sys.git#1494601d)
* rust-blake2 v0.0.0 (https://github.com/ebfe/rust-blake2.git#24bce9b1)
* rust-crypto v0.2.31

```
running 21 tests
test bench_blake2_rfc::blake2b_16     ... bench:         208 ns/iter (+/- 1) = 76 MB/s
test bench_blake2_rfc::blake2b_4096   ... bench:       5,986 ns/iter (+/- 34) = 684 MB/s
test bench_blake2_rfc::blake2b_65536  ... bench:      95,677 ns/iter (+/- 905) = 684 MB/s
test bench_blake2_rfc::blake2s_16     ... bench:         158 ns/iter (+/- 1) = 101 MB/s
test bench_blake2_rfc::blake2s_4096   ... bench:       8,863 ns/iter (+/- 60) = 462 MB/s
test bench_blake2_rfc::blake2s_65536  ... bench:     141,496 ns/iter (+/- 1,073) = 463 MB/s
test bench_libb2_sys::blake2b_16      ... bench:         214 ns/iter (+/- 6) = 74 MB/s
test bench_libb2_sys::blake2b_4096    ... bench:       5,292 ns/iter (+/- 50) = 773 MB/s
test bench_libb2_sys::blake2b_65536   ... bench:      84,093 ns/iter (+/- 363) = 779 MB/s
test bench_libb2_sys::blake2s_16      ... bench:         137 ns/iter (+/- 1) = 116 MB/s
test bench_libb2_sys::blake2s_4096    ... bench:       7,595 ns/iter (+/- 66) = 539 MB/s
test bench_libb2_sys::blake2s_65536   ... bench:     121,407 ns/iter (+/- 711) = 539 MB/s
test bench_rust_blake2::blake2b_16    ... bench:         369 ns/iter (+/- 3) = 43 MB/s
test bench_rust_blake2::blake2b_4096  ... bench:      11,096 ns/iter (+/- 47) = 369 MB/s
test bench_rust_blake2::blake2b_65536 ... bench:     176,905 ns/iter (+/- 1,906) = 370 MB/s
test bench_rust_blake2::blake2s_16    ... bench:         362 ns/iter (+/- 6) = 44 MB/s
test bench_rust_blake2::blake2s_4096  ... bench:      21,981 ns/iter (+/- 121) = 186 MB/s
test bench_rust_blake2::blake2s_65536 ... bench:     351,017 ns/iter (+/- 2,354) = 186 MB/s
test bench_rust_crypto::blake2b_16    ... bench:         326 ns/iter (+/- 1) = 49 MB/s
test bench_rust_crypto::blake2b_4096  ... bench:       9,745 ns/iter (+/- 56) = 420 MB/s
test bench_rust_crypto::blake2b_65536 ... bench:     156,024 ns/iter (+/- 4,543) = 420 MB/s

test result: ok. 0 passed; 0 failed; 0 ignored; 21 measured
```

Comparative benchmarking for Rust implementations of the [BLAKE2] hash functions.

[BLAKE2]: https://blake2.net/

Sample output for the following versions:

* blake2-rfc v0.2.2
* libb2-sys v0.0.2 (https://github.com/cesarb/libb2-sys.git#1494601d)
* rust-blake2 v0.0.0 (https://github.com/ebfe/rust-blake2.git#24bce9b1)
* rust-crypto v0.2.31

```
running 21 tests
test bench_blake2_rfc::blake2b_16     ... bench:         207 ns/iter (+/- 1) = 77 MB/s
test bench_blake2_rfc::blake2b_4096   ... bench:       5,826 ns/iter (+/- 98) = 703 MB/s
test bench_blake2_rfc::blake2b_65536  ... bench:      92,517 ns/iter (+/- 2,681) = 708 MB/s
test bench_blake2_rfc::blake2s_16     ... bench:         197 ns/iter (+/- 1) = 81 MB/s
test bench_blake2_rfc::blake2s_4096   ... bench:      11,387 ns/iter (+/- 529) = 359 MB/s
test bench_blake2_rfc::blake2s_65536  ... bench:     181,892 ns/iter (+/- 8,607) = 360 MB/s
test bench_libb2_sys::blake2b_16      ... bench:         213 ns/iter (+/- 2) = 75 MB/s
test bench_libb2_sys::blake2b_4096    ... bench:       5,297 ns/iter (+/- 26) = 773 MB/s
test bench_libb2_sys::blake2b_65536   ... bench:      84,016 ns/iter (+/- 842) = 780 MB/s
test bench_libb2_sys::blake2s_16      ... bench:         137 ns/iter (+/- 4) = 116 MB/s
test bench_libb2_sys::blake2s_4096    ... bench:       7,596 ns/iter (+/- 78) = 539 MB/s
test bench_libb2_sys::blake2s_65536   ... bench:     121,411 ns/iter (+/- 698) = 539 MB/s
test bench_rust_blake2::blake2b_16    ... bench:         369 ns/iter (+/- 2) = 43 MB/s
test bench_rust_blake2::blake2b_4096  ... bench:      11,098 ns/iter (+/- 97) = 369 MB/s
test bench_rust_blake2::blake2b_65536 ... bench:     176,873 ns/iter (+/- 846) = 370 MB/s
test bench_rust_blake2::blake2s_16    ... bench:         362 ns/iter (+/- 4) = 44 MB/s
test bench_rust_blake2::blake2s_4096  ... bench:      21,974 ns/iter (+/- 218) = 186 MB/s
test bench_rust_blake2::blake2s_65536 ... bench:     350,962 ns/iter (+/- 4,034) = 186 MB/s
test bench_rust_crypto::blake2b_16    ... bench:         329 ns/iter (+/- 2) = 48 MB/s
test bench_rust_crypto::blake2b_4096  ... bench:       9,734 ns/iter (+/- 53) = 420 MB/s
test bench_rust_crypto::blake2b_65536 ... bench:     155,640 ns/iter (+/- 988) = 421 MB/s

test result: ok. 0 passed; 0 failed; 0 ignored; 21 measured
```

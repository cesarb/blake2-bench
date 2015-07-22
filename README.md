Comparative benchmarking for pure Rust implementations of the [BLAKE2] hash functions.

[BLAKE2]: https://blake2.net/

Sample output for the following versions:

* blake2-rfc v0.2.0
* rust-blake2 v0.0.0 (https://github.com/ebfe/rust-blake2.git#24bce9b1)
* rust-crypto v0.2.31

```
running 15 tests
test bench_blake2_rfc::blake2b_16     ... bench:         238 ns/iter (+/- 95) = 67 MB/s
test bench_blake2_rfc::blake2b_4096   ... bench:       6,657 ns/iter (+/- 31) = 615 MB/s
test bench_blake2_rfc::blake2b_65536  ... bench:     106,207 ns/iter (+/- 1,057) = 617 MB/s
test bench_blake2_rfc::blake2s_16     ... bench:         192 ns/iter (+/- 6) = 83 MB/s
test bench_blake2_rfc::blake2s_4096   ... bench:      11,015 ns/iter (+/- 58) = 371 MB/s
test bench_blake2_rfc::blake2s_65536  ... bench:     175,834 ns/iter (+/- 1,049) = 372 MB/s
test bench_rust_blake2::blake2b_16    ... bench:         368 ns/iter (+/- 25) = 43 MB/s
test bench_rust_blake2::blake2b_4096  ... bench:      11,190 ns/iter (+/- 429) = 366 MB/s
test bench_rust_blake2::blake2b_65536 ... bench:     177,932 ns/iter (+/- 1,136) = 368 MB/s
test bench_rust_blake2::blake2s_16    ... bench:         368 ns/iter (+/- 8) = 43 MB/s
test bench_rust_blake2::blake2s_4096  ... bench:      22,005 ns/iter (+/- 148) = 186 MB/s
test bench_rust_blake2::blake2s_65536 ... bench:     351,270 ns/iter (+/- 2,193) = 186 MB/s
test bench_rust_crypto::blake2b_16    ... bench:         324 ns/iter (+/- 1) = 49 MB/s
test bench_rust_crypto::blake2b_4096  ... bench:       9,769 ns/iter (+/- 105) = 419 MB/s
test bench_rust_crypto::blake2b_65536 ... bench:     156,086 ns/iter (+/- 964) = 419 MB/s

test result: ok. 0 passed; 0 failed; 0 ignored; 15 measured
```

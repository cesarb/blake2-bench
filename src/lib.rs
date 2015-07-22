#![feature(test)]

extern crate blake2_rfc;
extern crate blake2 as rust_blake2;
extern crate crypto as rust_crypto;
extern crate test;

#[cfg(test)]
mod bench_blake2_rfc {
    use test::Bencher;

    fn bench_blake2b(data: &[u8], b: &mut Bencher) {
        use blake2_rfc::blake2b::Blake2b;

        b.bytes = data.len() as u64;
        b.iter(|| {
            let mut state = Blake2b::new(64);
            state.update(data);
            state.finalize()
        })
    }

    #[bench] fn blake2b_16(b: &mut Bencher) { bench_blake2b(&[0; 16], b) }
    #[bench] fn blake2b_4096(b: &mut Bencher) { bench_blake2b(&[0; 4096], b) }
    #[bench] fn blake2b_65536(b: &mut Bencher) { bench_blake2b(&[0; 65536], b) }

    fn bench_blake2s(data: &[u8], b: &mut Bencher) {
        use blake2_rfc::blake2s::Blake2s;

        b.bytes = data.len() as u64;
        b.iter(|| {
            let mut state = Blake2s::new(32);
            state.update(data);
            state.finalize()
        })
    }

    #[bench] fn blake2s_16(b: &mut Bencher) { bench_blake2s(&[0; 16], b) }
    #[bench] fn blake2s_4096(b: &mut Bencher) { bench_blake2s(&[0; 4096], b) }
    #[bench] fn blake2s_65536(b: &mut Bencher) { bench_blake2s(&[0; 65536], b) }
}

#[cfg(test)]
mod bench_rust_blake2 {
    use test::Bencher;

    fn bench_blake2b(data: &[u8], b: &mut Bencher) {
        use rust_blake2::Blake2b;

        b.bytes = data.len() as u64;
        b.iter(|| {
            let mut state = Blake2b::new(64);
            state.update(data);

            let mut result = [0; 64];
            state.finalize(&mut result);
            result
        })
    }

    #[bench] fn blake2b_16(b: &mut Bencher) { bench_blake2b(&[0; 16], b) }
    #[bench] fn blake2b_4096(b: &mut Bencher) { bench_blake2b(&[0; 4096], b) }
    #[bench] fn blake2b_65536(b: &mut Bencher) { bench_blake2b(&[0; 65536], b) }

    fn bench_blake2s(data: &[u8], b: &mut Bencher) {
        use rust_blake2::Blake2s;

        b.bytes = data.len() as u64;
        b.iter(|| {
            let mut state = Blake2s::new(32);
            state.update(data);

            let mut result = [0; 32];
            state.finalize(&mut result);
            result
        })
    }

    #[bench] fn blake2s_16(b: &mut Bencher) { bench_blake2s(&[0; 16], b) }
    #[bench] fn blake2s_4096(b: &mut Bencher) { bench_blake2s(&[0; 4096], b) }
    #[bench] fn blake2s_65536(b: &mut Bencher) { bench_blake2s(&[0; 65536], b) }
}

#[cfg(test)]
mod bench_rust_crypto {
    use test::Bencher;

    fn bench_blake2b(data: &[u8], b: &mut Bencher) {
        use rust_crypto::blake2b::Blake2b;
        use rust_crypto::digest::Digest;

        b.bytes = data.len() as u64;
        b.iter(|| {
            let mut state = Blake2b::new(64);
            state.input(data);

            let mut result = [0; 64];
            state.result(&mut result);
            result
        })
    }

    #[bench] fn blake2b_16(b: &mut Bencher) { bench_blake2b(&[0; 16], b) }
    #[bench] fn blake2b_4096(b: &mut Bencher) { bench_blake2b(&[0; 4096], b) }
    #[bench] fn blake2b_65536(b: &mut Bencher) { bench_blake2b(&[0; 65536], b) }
}

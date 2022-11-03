#![feature(scoped_threads)]
// Benchmark results:
// test bench_large_parallel   ... bench:     193,175 ns/iter (+/- 14,571)
// test bench_large_sequential ... bench:     351,673 ns/iter (+/- 4,694)
// test bench_small_parallel   ... bench:       8,268 ns/iter (+/- 384)
// test bench_small_sequential ... bench:      12,191 ns/iter (+/- 475)
// test bench_tiny_parallel    ... bench:          34 ns/iter (+/- 0)
// test bench_tiny_sequential  ... bench:          40 ns/iter (+/- 1)

use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        HashMap::new()
    } else if input.len() == 1 {
        count_freq(input)
    } else {
        let mut freq: HashMap<char, usize> = HashMap::new();

        let chunk_size = input.len() / worker_count;

        if chunk_size != 0 {
            let chunks = input.chunks(chunk_size);

            for chunk in chunks {
                let f = count_freq(chunk);

                for (c, s) in f {
                    (*freq.entry(c).or_insert(0)) += s;
                }
            }
        }

        freq
    }
}

fn count_freq(input: &[&str]) -> HashMap<char, usize> {
    let mut freq: HashMap<char, usize> = HashMap::new();

    for c in input.iter().flat_map(|i| {
        i.chars().filter_map(|mut s| {
            if s.is_alphabetic() {
                s.make_ascii_lowercase();
                Some(s)
            } else {
                None
            }
        })
    }) {
        (*freq.entry(c).or_insert(0)) += 1;
    }

    freq
}

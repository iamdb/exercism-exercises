#![feature(scoped_threads)]
// Benchmark results:
// test bench_large_parallel   ... bench:     144,595 ns/iter (+/- 24,168)
// test bench_large_sequential ... bench:     357,033 ns/iter (+/- 2,854)
// test bench_small_parallel   ... bench:       7,601 ns/iter (+/- 217)
// test bench_small_sequential ... bench:      12,306 ns/iter (+/- 545)
// test bench_tiny_parallel    ... bench:          35 ns/iter (+/- 1)
// test bench_tiny_sequential  ... bench:          41 ns/iter (+/- 0)

use std::{collections::HashMap, thread};

pub fn frequency(input: &[&'static str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        HashMap::new()
    } else if input.len() < 100 {
        count_freq(input)
    } else {
        let mut freq: HashMap<char, usize> = HashMap::new();

        let chunk_size = input.len() / worker_count;

        if chunk_size != 0 {
            let chunks = input.chunks(chunk_size);

            thread::scope(|s| {
                let mut threads = Vec::with_capacity(worker_count);

                for chunk in chunks {
                    threads.push(s.spawn(|| count_freq(chunk)));
                }

                for t in threads {
                    let f = t.join().unwrap();
                    for (c, s) in f {
                        (*freq.entry(c).or_insert(0)) += s;
                    }
                }
            });
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

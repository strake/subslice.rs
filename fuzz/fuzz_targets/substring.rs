#![no_main]
#[macro_use] extern crate libfuzzer_sys;

use std::cmp::{min, max};
extern crate subslice;

// This fuzzer tests that given a slice data, pick a subslice out of it
// and check that the find_bytes function can find the correct position
// of the subslice inside the whole data.
//
// The first 4 bytes are only used for picking the boundaries of the
// subslice (so nothing is random, it's driven by the contents of data).

fuzz_target!(|data: &[u8]| {
    if data.len() > 4 {
        let len = data.len() - 4;
        // use first 4 bytes as delimiters
        let (a, b) = (data[0] as usize, data[1] as usize);
        let first = ((a << 8) | b) % len;
        let (a, b) = (data[2] as usize, data[3] as usize);
        let second = ((a << 8) | b) % len;
        let (first, second) = (min(first, second), max(first, second));
        let (_, data) = data.split_at(4);
        let needle = &data[first..second];

        let find_result = subslice::SubsliceExt::find(data, needle);
        if let Some(i) = find_result {
            assert!(i <= first, "i={} must be leq first={}", i, first);
        } else {
            panic!("Expected match at first={}\ndata={:?}, needle={:?}", first, data, needle);
        }
    }
});

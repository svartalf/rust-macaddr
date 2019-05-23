#![feature(test)]

extern crate test;

use std::str::FromStr;

use macaddr::{MacAddr, MacAddr6, MacAddr8};

#[bench]
fn bench_v6_canonical_str_parse(b: &mut test::Bencher) {
    b.iter(|| MacAddr6::from_str("12-34-56-78-9A-BC"))
}

#[bench]
fn bench_v8_canonical_str_parse(b: &mut test::Bencher) {
    b.iter(|| MacAddr8::from_str("12-34-56-78-9A-BC-DE-F0"))
}

#[bench]
fn bench_addr_v6_canonical_str_parse(b: &mut test::Bencher) {
    b.iter(|| MacAddr::from_str("12-34-56-78-9A-BC"))
}

#[bench]
fn bench_addr_v8_canonical_str_parse(b: &mut test::Bencher) {
    b.iter(|| MacAddr::from_str("12-34-56-78-9A-BC-DE-F0"))
}

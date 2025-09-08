#![feature(test)]


extern crate test;

use test::Bencher;

use std::fs;


fn read_input(file: &str) -> Vec<u8> {

    let contents = fs::read_to_string(file).expect("Cannot read file");

    contents.lines().map(|l| u8::from_str_radix(l, 16).unwrap()).collect()

}


#[bench]

fn unsorted(b: &mut Bencher) {

    let data = read_input("2.input");

    b.iter(|| {

        let mut v = data.clone();

        v.sort();

    });

}

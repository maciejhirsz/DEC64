#![feature(test)]

extern crate test;
extern crate dec64;
extern crate dtoa;

use dec64::Dec64;
use test::Bencher;


#[bench]
fn f64_write_pi(b: &mut Bencher) {
    let mut target = Vec::new();

    let pi = 3.141592653589793;

    b.iter(|| {
        dtoa::write(&mut target, pi).unwrap();
    })
}

#[bench]
fn dec64_write_pi(b: &mut Bencher) {
    let mut target = Vec::new();

    let pi = Dec64::from_raw_parts(3141592653589793, -15);

    b.iter(|| {
        pi.write(&mut target).unwrap();
    })
}

extern crate dec64;

use dec64::Dec64;

#[test]
fn roundtrip_usize() {
    let dec = Dec64::from(255_usize);

    let num: usize = dec.into();

    assert_eq!(num, 255_usize);
}

#[test]
fn roundtrip_u8() {
    let dec = Dec64::from(255_u8);

    let num: u8 = dec.into();

    assert_eq!(num, 255_u8);
}

#[test]
fn roundtrip_u16() {
    let dec = Dec64::from(255_u16);

    let num: u16 = dec.into();

    assert_eq!(num, 255_u16);
}

#[test]
fn roundtrip_u32() {
    let dec = Dec64::from(255_u32);

    let num: u32 = dec.into();

    assert_eq!(num, 255_u32);
}

#[test]
fn roundtrip_u64() {
    let dec = Dec64::from(255_u64);

    let num: u64 = dec.into();

    assert_eq!(num, 255_u64);
}

#[test]
fn roundtrip_isize() {
    let dec = Dec64::from(-128_isize);

    let num: isize = dec.into();

    assert_eq!(num, -128_isize);
}

#[test]
fn roundtrip_i8() {
    let dec = Dec64::from(-128_i8);

    let num: i8 = dec.into();

    assert_eq!(num, -128_i8);
}

#[test]
fn roundtrip_i16() {
    let dec = Dec64::from(-128_i16);

    let num: i16 = dec.into();

    assert_eq!(num, -128_i16);
}

#[test]
fn roundtrip_i32() {
    let dec = Dec64::from(-128_i32);

    let num: i32 = dec.into();

    assert_eq!(num, -128_i32);
}

#[test]
fn roundtrip_i64() {
    let dec = Dec64::from(-128_i64);

    let num: i64 = dec.into();

    assert_eq!(num, -128_i64);
}

#[test]
fn compose_f64_pi() {
    let dec = Dec64::from_raw_parts(3141592653589793, -15);

    let num: f64 = dec.into();

    assert_eq!(num, 3.141592653589793);
}

#[test]
fn compose_f32_pi() {
    let dec = Dec64::from_raw_parts(3141592653589793, -15);

    let num: f32 = dec.into();

    assert_eq!(num, 3.141592653589793);
}

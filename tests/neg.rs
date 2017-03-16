extern crate dec64;

use dec64::Dec64;
use dec64::NAN;
use dec64::ZERO;
use dec64::consts::PI;
use dec64::more_consts::NAN_NAN;
use dec64::more_consts::NEG_PI;
use dec64::more_consts::NEG_TENTH;
use dec64::more_consts::TENTH;
use dec64::more_consts::ZIP;
use dec64::more_consts::normal::MAXINT;
use dec64::more_consts::normal::MININT;
use dec64::more_consts::normal::NEG_ONE;
use dec64::more_consts::normal::NEG_TEN;
use dec64::more_consts::normal::NEG_TWO;
use dec64::more_consts::normal::ONE;
use dec64::more_consts::normal::TEN;
use dec64::more_consts::normal::TWO;

#[test]
fn neg_zero() {
	assert_eq!(-ZERO, ZERO);
}

#[test]
fn neg_zip() {
	assert_eq!(-ZIP, ZERO);
}

#[test]
fn neg_trivial() {
	assert_eq!(-ONE, NEG_ONE);
	assert_eq!(-TWO, NEG_TWO);
	assert_eq!(-TEN, NEG_TEN);
	assert_eq!(-PI, NEG_PI);
}

#[test]
fn neg_trivial_neg() {
	assert_eq!(-NEG_ONE, ONE);
	assert_eq!(-NEG_TWO, TWO);
	assert_eq!(-NEG_TEN, TEN);
	assert_eq!(-NEG_PI, PI);
}

#[test]
fn neg_nan() {
	assert_eq!(-NAN,  NAN);
	assert_eq!(-NAN_NAN, NAN);
}

#[test]
fn neg_tenth() {
	assert_eq!(-TENTH, NEG_TENTH);
}

#[test]
fn neg_neg_tenth() {
	assert_eq!(-NEG_TENTH, TENTH);
}

#[test]
fn neg_max() {
	let expect = Dec64::from_parts(-dec64::MAX_COEFFICIENT, dec64::MAX_EXP as i8);
	assert_eq!(-dec64::MAX, expect);
}

#[test]
fn neg_min() {
	assert_eq!(-dec64::MIN, NAN);
}

#[test]
fn neg_maxint() {
	let expect = Dec64::from_parts(-dec64::MAX_COEFFICIENT, 0);
	assert_eq!(-MAXINT, expect);
}

#[test]
fn neg_minint() {
	// this will round up since last digit of MIN_COEFFICIENT is 6
	let expect = Dec64::from_parts(-dec64::MIN_COEFFICIENT/10 + 1, 1);
	assert_eq!(-MININT, expect);
}

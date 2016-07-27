extern crate dec64;

mod macros;

use dec64::Dec64;
use dec64::ZERO;
use dec64::NAN;
use dec64::more_consts::NAN_NAN;
use dec64::more_consts::NEG_FIFTH;
use dec64::more_consts::NEG_TENTH;
use dec64::more_consts::normal::ONE;
use dec64::more_consts::normal::TWO;
use dec64::more_consts::normal::NEG_ONE;
use dec64::more_consts::normal::NEG_TWO;
use dec64::more_consts::normal::NEG_FOUR;
use dec64::more_consts::normal::NEG_EIGHT;

#[test]
fn add_zero() {
	assert_eq!(ZERO + ZERO, ZERO);

	assert_eq!(ZERO + ONE, ONE);
	assert_eq!(ONE  + ZERO, ONE);

	assert_eq!(ZERO + dec64::MAX, dec64::MAX);
	assert_eq!(dec64::MAX + ZERO, dec64::MAX);

	assert_eq!(ZERO + dec64::MIN, dec64::MIN);
	assert_eq!(dec64::MIN + ZERO, dec64::MIN);
}

#[test]
fn add_trivial() {
	assert_eq!(ONE + ONE, TWO);
	assert_eq!(ONE + NEG_ONE, ZERO);
	assert_eq!(NEG_ONE + ONE, ZERO);
}

#[test]
fn add_same_neg() {
	assert_eq!(NEG_ONE + NEG_ONE, NEG_TWO);
	assert_eq!(NEG_TWO + NEG_TWO, NEG_FOUR);
	assert_eq!(NEG_FOUR + NEG_FOUR, NEG_EIGHT);
}

#[test]
fn add_nan() {
	assert_eq!(NAN_NAN + NAN_NAN, NAN);

	assert_eq!(NAN_NAN + ZERO, NAN);
	assert_eq!(ZERO + NAN_NAN, NAN);

	assert_eq!(NAN_NAN + ONE, NAN);
	assert_eq!(ONE + NAN_NAN, NAN);

	assert_eq!(NAN_NAN + dec64::MAX, NAN);
	assert_eq!(dec64::MAX + NAN_NAN, NAN);

	assert_eq!(NAN_NAN + dec64::MIN, NAN);
	assert_eq!(dec64::MIN + NAN_NAN, NAN);
}

#[test]
fn add_neg_tenth() {
	assert_eq!(NEG_TENTH + NEG_TENTH, NEG_FIFTH);
}

#[test]
fn add_range_overflow() {
	assert_eq!(dec64::MAX + dec64::MAX, NAN);
	assert_eq!(dec64::MIN + dec64::MIN, NAN);
}

#[test]
fn add_minmax() {
	let expect = Dec64::from_parts(-1, 127);

	assert_eq!(dec64::MAX + dec64::MIN, expect);
	assert_eq!(dec64::MIN + dec64::MAX, expect);
}

#[test]
fn add_positive_integer_overflow() {
	let ten = dec64::more_consts::normal::TEN;
	let value =  Dec64::from_parts(36028797018963960, 0);
	let expect = Dec64::from_parts(3602879701896397, 1);
	let result = value + ten;

	assert_eq!(result, expect);
}

#[test]
fn add_negative_integer_overflow() {
	let ten = dec64::more_consts::normal::NEG_TEN;
	let value =  Dec64::from_parts(-36028797018963960, 0);
	let expect = Dec64::from_parts(-3602879701896397, 1);
	let result = value + ten;

	assert_eq!(result, expect);
}

#[test]
fn add_with_zero_exponent() {
	let coefficient_a = 333;
	let coefficient_b = 222;
	let exponent      = 0;
	let a = Dec64::from_parts(coefficient_a, exponent);
	let b = Dec64::from_parts(coefficient_b, exponent);
	let expect = Dec64::from_parts(coefficient_a + coefficient_b, exponent);
	let result = a + b;

	assert_eq!(result.coefficient(), coefficient_a + coefficient_b);
	assert_eq!(result.exponent(), exponent);
	assert_eq!(result, expect);
}

#[test]
fn add_with_same_non_zero_exponent() {
	let coefficient_a = 333;
	let coefficient_b = 222;
	let exponent      = 11;
	let a = Dec64::from_parts(coefficient_a, exponent);
	let b = Dec64::from_parts(coefficient_b, exponent);
	let expect = Dec64::from_parts(coefficient_a + coefficient_b, exponent);
	let result = a + b;

	assert_eq!(result.coefficient(), coefficient_a + coefficient_b);
	assert_eq!(result.exponent(), exponent);
	assert_eq!(result, expect);
}

#[test]
fn add_with_reducable_exponent() {
	let coefficient_a = 333;
	let coefficient_b = 222;
	let exponent_a    = 20;
	let exponent_b    = 10;
	let a = Dec64::from_parts(coefficient_a, exponent_a);
	let b = Dec64::from_parts(coefficient_b, exponent_b);
	let expect = Dec64::from_parts(coefficient_a * 10_000_000_000 + coefficient_b, exponent_b);
	let result = a + b;

	assert_eq!(result, expect);
}

#[test]
fn add_with_significance_loss() {
	let coefficient_a = 333;
	let coefficient_b = 222;
	let exponent_a    = 26;
	let exponent_b    = 10;
	let a = Dec64::from_parts(coefficient_a, exponent_a);
	let b = Dec64::from_parts(coefficient_b, exponent_b);
	let expect = Dec64::from_parts(coefficient_a * 100_000_000_000_000 + coefficient_b / 100, exponent_b + 2);
	let result = a + b;

	assert_eq!(result, expect);
}

#[test]
fn add_double_max_coefficient() {
	let value  = Dec64::from_parts(dec64::MAX_COEFFICIENT, 64);
	let expect = Dec64::from_parts(dec64::MAX_COEFFICIENT * 2 / 10, 65);

	assert_eq!(value + value, expect);
}

#[test]
fn add_double_min_coefficient() {
	let value  = Dec64::from_parts(dec64::MIN_COEFFICIENT, 64);
	// min coefficient is rounded down when scaling exponent since -1.
	let expect = Dec64::from_parts(dec64::MIN_COEFFICIENT * 2 / 10 - 1, 65);

	assert_eq!(value + value, expect);
}

//! Not very useful constants. Mainly for tests.

use super::Dec64;

/// A non-normal `NaN`.
pub const NAN_NAN: Dec64 = dec64_raw!(0x8080);
/// Difference between `1.0` and the previous largest representable number = `1.0_e-16`.
pub const NEG_EPSILON: Dec64 = dec64_parts!(1, -16_i8);
/// A non normal `0`.
pub const ZIP: Dec64 = dec64_raw!(90);
/// `0.01`
pub const CENT: Dec64 = dec64_parts!(1, -2_i8);
/// `0.1`
pub const TENTH: Dec64 = dec64_parts!(1, -1_i8);
/// `0.5`
pub const HALF: Dec64 = dec64_parts!(5, -1_i8);
/// `0.9999999999999999`
pub const ALMOST_ONE: Dec64 = dec64_parts!(9999999999999999, -16_i8);
/// `-0.9999999999999999`
pub const ALMOST_NEG_ONE: Dec64 = dec64_parts!(-999999999999999, -16_i8);
/// `1.0 / normal::MAXINT`
pub const FRAC_1_MAXINT: Dec64 = dec64_parts!(27755575615628914, -33_i8);
/// Googol (`10^100`)
pub const GOOGOL: Dec64 = dec64_parts!(1, 100);
/// The smallest possible negative number.
pub const NEG_MINNUM: Dec64 = dec64_parts!(-1, super::MIN_EXP);
/// -PI
pub const NEG_PI: Dec64 = dec64_parts!(-31415926535897932, -16_i8);
/// `-0.1`
pub const NEG_TENTH: Dec64 = dec64_parts!(-1, -1_i8);
/// `-0.2`
pub const NEG_FIFTH: Dec64 = dec64_parts!(-2, -1_i8);

/// Set of numbers with zero-exponent.
pub mod normal {
    use super::super::Dec64;

    pub use super::super::ZERO;

    pub const ONE:   Dec64 = dec64_parts!( 1, 0);
    pub const TWO:   Dec64 = dec64_parts!( 2, 0);
    pub const THREE: Dec64 = dec64_parts!( 3, 0);
    pub const FOUR:  Dec64 = dec64_parts!( 4, 0);
    pub const FIVE:  Dec64 = dec64_parts!( 5, 0);
    pub const SIX:   Dec64 = dec64_parts!( 6, 0);
    pub const SEVEN: Dec64 = dec64_parts!( 7, 0);
    pub const EIGHT: Dec64 = dec64_parts!( 8, 0);
    pub const NINE:  Dec64 = dec64_parts!( 9, 0);
    pub const TEN:   Dec64 = dec64_parts!(10, 0);

    pub const NEG_ONE:   Dec64 = dec64_parts!( -1, 0);
    pub const NEG_TWO:   Dec64 = dec64_parts!( -2, 0);
    pub const NEG_THREE: Dec64 = dec64_parts!( -3, 0);
    pub const NEG_FOUR:  Dec64 = dec64_parts!( -4, 0);
    pub const NEG_FIVE:  Dec64 = dec64_parts!( -5, 0);
    pub const NEG_SIX:   Dec64 = dec64_parts!( -6, 0);
    pub const NEG_SEVEN: Dec64 = dec64_parts!( -7, 0);
    pub const NEG_EIGHT: Dec64 = dec64_parts!( -8, 0);
    pub const NEG_NINE:  Dec64 = dec64_parts!( -9, 0);
    pub const NEG_TEN:   Dec64 = dec64_parts!(-10, 0);

    /// Maximal normal integer.
    pub const MAXINT: Dec64 = dec64_parts!(super::super::MAX_COEFFICIENT, 0);
    /// Minimal normal integer.
    pub const MININT: Dec64 = dec64_parts!(super::super::MIN_COEFFICIENT, 0);
}

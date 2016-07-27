#[macro_use] mod macros;
mod diyfp;
mod fmt;
mod grisu2;
pub mod more_consts;
mod ops;
mod write;

/// Minimum value of DEC64 coefficient.
pub const MIN_COEFFICIENT: i64 = -36_028_797_018_963_968;
/// Maximum value of DEC64 coefficient.
pub const MAX_COEFFICIENT: i64 =  36_028_797_018_963_967;
/// Minimum value of DEC64 exponent.
pub const MIN_EXP: i32 = -127;
/// Maximum value of DEC64 exponent.
pub const MAX_EXP: i32 =  127;

/// Difference between `1.0` and the next largest representable number = `1.0_e-16`.
pub const EPSILON: Dec64 = dec64_parts!(1, -16_i8);
/// Standard Not a Number (NaN) value.
pub const NAN: Dec64 = dec64_raw!(0x80);
/// Standard ZERO value.
pub const ZERO: Dec64 = dec64_raw!(0);
/// Largest DEC64 value = `36_028_797_018_963_967_e127`.
pub const MAX: Dec64 = dec64_parts!(MAX_COEFFICIENT, MAX_EXP);
/// Smallest DEC64 value = `-36_028_797_018_963_968_e127`.
pub const MIN: Dec64 = dec64_parts!(MIN_COEFFICIENT, MAX_EXP);
/// Smallest positive DEC64 = `1.0_e-127`.
pub const MIN_POSITIVE: Dec64 = dec64_parts!(1, MIN_EXP);

/// Basic mathematical constants.
pub mod consts {
    use super::Dec64;

    /// Archimedes constant: π = `3.1415926535897932`.
    pub const PI: Dec64 = dec64_parts!(31_415_926_535_897_932, -16_i8);

    /// π/2.0 = `1.5707963267948966`.
    pub const FRAC_PI_2: Dec64 = dec64_parts!(15_707_963_267_948_966, -16_i8);

    /// π/3.0 = `1.0471975511965977`.
    pub const FRAC_PI_3: Dec64 = dec64_parts!(10_471_975_511_965_977, -16_i8);

    /// π/4.0 = `0.7853981633974483`.
    pub const FRAC_PI_4: Dec64 = dec64_parts!(7_853_981_633_974_483, -16_i8);

    /// π/6.0 = `0.5235987755982989`.
    pub const FRAC_PI_6: Dec64 = dec64_parts!(5_235_987_755_982_989, -16_i8);

    /// π/8.0 = `0.3926990816987242`.
    pub const FRAC_PI_8: Dec64 = dec64_parts!(3_926_990_816_987_242, -16_i8);

    /// 1.0/π = `0.31830988618379067`.
    pub const FRAC_1_PI: Dec64 = dec64_parts!(31_830_988_618_379_067, -17_i8);

    /// 2.0/π = `0.6366197723675813`.
    pub const FRAC_2_PI: Dec64 = dec64_parts!(6_366_197_723_675_813, -16_i8);

    /// 2.0/sqrt(π) = `1.1283791670955125`.
    pub const FRAC_2_SQRT_PI: Dec64 = dec64_parts!(11_283_791_670_955_125, -16_i8);

    /// sqrt(2.0) = `1.4142135623730950`.
    pub const SQRT_2: Dec64 = dec64_parts!(14_142_135_623_730_950, -16_i8);

    /// 1.0/sqrt(2.0) = `0.7071067811865475`.
    pub const FRAC_1_SQRT_2: Dec64 = dec64_parts!(7_071_067_811_865_475, -16_i8);

    /// sqrt(3.0) = `1.7320508075688773`.
    pub const SQRT_3: Dec64 = dec64_parts!(17_320_508_075_688_773, -16_i8);

    /// 1.0/sqrt(3.0) = `0.5773502691896258`.
    pub const FRAC_1_SQRT_3: Dec64 = dec64_parts!(5_773_502_691_896_258, -16_i8);

    /// Euler's number: e = `2.7182818284590452`.
    pub const E: Dec64 = dec64_parts!(27_182_818_284_590_452, -16_i8);

    /// log2(e) = `1.4426950408889634`.
    pub const LOG2_E: Dec64 = dec64_parts!(14_426_950_408_889_634, -16_i8);

    /// log10(e) = `0.4342944819032518`.
    pub const LOG10_E: Dec64 = dec64_parts!(4_342_944_819_032_518, -16_i8);

    /// ln(2.0) = `0.6931471805599453`.
    pub const LN_2: Dec64 = dec64_parts!(6_931_471_805_599_453, -16_i8);

    /// ln(10.0) = `2.3025850929940457`.
    pub const LN_10: Dec64 = dec64_parts!(23_025_850_929_940_457, -16_i8);
}

const EXPONENT_MASK: i64 = 0xff;
const COEFFICIENT_MASK: i64 = !EXPONENT_MASK;
const SIGN_MASK: i64 = 1 << 63;

/// The powers of 10.
const POWERS_10: [u64; 20] = [
                       1,    // 0
                      10,    // 1
                     100,    // 2
                    1000,    // 3
                   10000,    // 4
                  100000,    // 5
                 1000000,    // 6
                10000000,    // 7
               100000000,    // 8
              1000000000,    // 9
             10000000000,    // 10
            100000000000,    // 11
           1000000000000,    // 12
          10000000000000,    // 13
         100000000000000,    // 14
        1000000000000000,    // 15
       10000000000000000,    // 16
      100000000000000000,    // 17
     1000000000000000000,    // 18
    10000000000000000000,    // 19
];

/// Struct holding DEC64 value.
#[derive(Clone, Copy, Default)]
pub struct Dec64 {
    value: i64
}

impl Dec64 {
    /// Construct Dec64 from raw coefficient and exponent parts.
    ///
    /// This can produce any kind of DEC64, including all varieties
    /// of zeros and NaNs.
    #[inline]
    pub fn from_parts(coefficient: i64, exponent: i8) -> Self {
        dec64_parts!(coefficient, exponent)
    }

    /// Construct Dec64 from coefficient and exponent values.
    /// Currently this is equivalent to `pack()`.
    ///
    /// This will produce only standard (coefficient zeroed) kind of zero and NaN.
    #[inline]
    pub fn new(coefficient: i64, exponent: i16) -> Self {
        Self::pack(coefficient, exponent as i32)
    }

    /// The pack function will combine the coefficient and exponent into a dec64.
    ///
    /// Numbers that are too huge to be contained in this format become NaN.
    ///
    /// Numbers that are too tiny to be contained in this format become zero.
    pub fn pack(mut coefficient: i64, mut exponent: i32) -> Self {
        if coefficient == 0 {
            // If the coefficient is zero, also zero the exponent.
            return ZERO;
        }

        // Is the exponent within supported range?
        if MIN_EXP <= exponent && exponent <= MAX_EXP {
            // Is the coefficient within supported range?
            if MIN_COEFFICIENT <= coefficient && coefficient <= MAX_COEFFICIENT {
                // Coefficient and exponent are OK.
                return dec64_parts!(coefficient, exponent);
            } else {
                // The coefficient is too long.
                // Add one to the exponent and Divide the coefficient by 10.
                loop {
                    exponent += 1;
                    if exponent > MAX_EXP {
                        // We cannot fit this number.
                        return NAN;
                    }

                    let rem = coefficient % 10;
                    coefficient /= 10;
                    // Reminder of coefficient division for rounding decision.
                    // Does it fit now?
                    if MIN_COEFFICIENT <= coefficient && coefficient <= MAX_COEFFICIENT {
                        // Examine the remainder to determine if the coefficient should be rounded up
                        // or down. We will shift before adding in the rounding bit to get the cheap
                        // overflow check. If rounding does not cause overflow, pack up and get out.
                        let round_add = if rem <= -5 {
                            -1 << 8
                        } else if rem >= 5 {
                             1 << 8
                        } else {
                             0
                        };

                        // If rounding caused the coefficient to overflow, then go one more time
                        // through the loop. Otherwise return the dec64.
                        // This is extremely unlikely.
                        let (ret_value, overflow) =  (coefficient << 8).overflowing_add(round_add);
                        if !overflow {
                            if coefficient == 0 {
                                // If the coefficient is zero, also zero the exponent.
                                return ZERO;
                            }
                            return Dec64 { value: ret_value | ((exponent as u8) as i64) };
                        }
                    }
                }
            }
        } else if exponent > MAX_EXP {
            // The exponent is too big. We can attempt to reduce it by scaling back.
            // This can decrease it in a small set of cases.
            loop {
                // try multiplying the coefficient by 10
                let (coefficient_mul_10, overflow) = coefficient.overflowing_mul(10);
                if overflow || coefficient_mul_10 < MIN_COEFFICIENT || MAX_COEFFICIENT < coefficient_mul_10 {
                    // We failed to salvage.
                    return NAN;
                }
                coefficient = coefficient_mul_10;

                // decrement the exponent
                exponent -= 1;
                if exponent <= MAX_EXP {
                    return dec64_parts!(coefficient, exponent);
                }
            }
        } else if exponent < MIN_EXP {
            // The exponent is too small. We can attempt to increase it by scaling forward.
            // This can increase it in a small set of cases.
            loop {
                let (coefficient_div_10, overflow) = coefficient.overflowing_div(10);
                if overflow || coefficient_div_10 == 0 {
                    // Value is too small to salvage.
                    return ZERO;
                }
                coefficient = coefficient_div_10;

                // increment the exponent
                exponent += 1;
                if exponent >= MIN_EXP {
                    return dec64_parts!(coefficient, exponent);
                }
            }
        }

        // We should've accounted for all cases.
        // If we ever reach here then there's a BUG in the implementation.
        unreachable!("Dec64::pack(): BUG");
    }

    /// Returns the DEC64 coefficient.
    #[inline]
    pub fn coefficient(self) -> i64 {
        self.value >> 8
    }

    /// Returns the DEC64 exponent.
    #[inline]
    pub fn exponent(self) -> i8 {
        self.value as i8
    }

    /// Returns `true` if DEC64 is any Not a Number (NaN) and `false` otherwise.
    ///
    /// DEC64 NaN have exponent value of `-128`, and any coefficient.
    ///
    /// Note that NaNs are equal to each other only when their coefficient are
    /// equal too.
    #[inline]
    pub fn is_nan(self) -> bool {
        self.exponent() == -128
    }

    /// Returns `true` if DEC64 is zero and `false` otherwise.
    ///
    /// DEC64 zeros have coefficient value of 0 and any non-NaN exponent.
    ///
    /// Note that all zeros are equal to each other, regardless of
    /// exponent value.
    #[inline]
    pub fn is_zero(self) -> bool {
        self.coefficient() == 0 && !self.is_nan()
    }

    /// Returns `false` if the DEC64 contains a non-zero fractional part or if it is NaN,
    /// and `true` otherwise.
    #[inline]
    pub fn is_integer(self) -> bool {
        let zero_coefficient = self.coefficient() == 0;
        if self.is_nan() || (self.exponent() <= -17 && !zero_coefficient) {
            // Extreme negative or positive exponents can never be integer. (This incules NaN).
            return false;
        } else if self.exponent() >= 0 || zero_coefficient {
            return true;
        }

        // Divide coefficient by the power of ten. If the remainder is zero, then return true.
        if self.coefficient() % POWERS_10[-self.exponent() as usize] as i64 == 0 {
            return true;
        }

        false
    }
}

impl PartialEq<Dec64> for Dec64 {
    /// Compare two DEC64 numbers.
    /// Denormal zeroes are equal but denormal NaNs are not.
    fn eq(&self, other: &Dec64) -> bool {
        // If the numbers are trivally equal, then return true.
        if self.value == other.value {
            return true;
        }

        // Zeroes are equal.
        if self.is_zero() && other.is_zero() {
            return true;
        }

        // If coefficient signs are different, or exponents are equal at this point
        // (after previous values equality test) then te numbers are not equal.
        let vals_xor = self.value ^ other.value;
        if vals_xor & (SIGN_MASK | EXPONENT_MASK) != 0 {
            return false;
        }

        // Do it the hard way by subtracting. Is the difference zero?
        // TODO: implement Sub.
        //if (self - other).is_zero() {
        //    return true;
        //}

        false
    }
}

fn exponent_to_power_f64(e: i8) -> f64 {
    static POS_POWERS: [f64; 23] = [
          1.0,    1e1,    1e2,    1e3,    1e4,    1e5,    1e6,    1e7,
          1e8,    1e9,   1e10,   1e11,   1e12,   1e13,   1e14,   1e15,
         1e16,   1e17,   1e18,   1e19,   1e20,   1e21,   1e22
    ];

    static NEG_POWERS: [f64; 23] = [
          1.0,   1e-1,   1e-2,   1e-3,   1e-4,   1e-5,   1e-6,   1e-7,
         1e-8,   1e-9,  1e-10,  1e-11,  1e-12,  1e-13,  1e-14,  1e-15,
        1e-16,  1e-17,  1e-18,  1e-19,  1e-20,  1e-21,  1e-22
    ];

    let index = e.abs() as usize;

    if index < 23 {
        if e < 0 {
            NEG_POWERS[index]
        } else {
            POS_POWERS[index]
        }
    } else {
        // powf is more accurate
        10f64.powf(e as f64)
    }
}

fn exponent_to_power_f32(e: i8) -> f32 {
    static POS_POWERS: [f32; 16] = [
          1.0,    1e1,    1e2,    1e3,    1e4,    1e5,    1e6,    1e7,
          1e8,    1e9,   1e10,   1e11,   1e12,   1e13,   1e14,   1e15
    ];

    static NEG_POWERS: [f32; 16] = [
          1.0,   1e-1,   1e-2,   1e-3,   1e-4,   1e-5,   1e-6,   1e-7,
         1e-8,   1e-9,  1e-10,  1e-11,  1e-12,  1e-13,  1e-14,  1e-15
    ];

    let index = e.abs() as usize;

    if index < 16 {
        if e < 0 {
            NEG_POWERS[index]
        } else {
            POS_POWERS[index]
        }
    } else {
        // powf is more accurate
        10f32.powf(e as f32)
    }
}

impl From<Dec64> for f64 {
    fn from(dec: Dec64) -> f64 {
        (dec.coefficient() as f64) * exponent_to_power_f64(dec.exponent())
    }
}

impl From<Dec64> for f32 {
    fn from(dec: Dec64) -> f32 {
        (dec.coefficient() as f32) * exponent_to_power_f32(dec.exponent())
    }
}

impl From<f64> for Dec64 {
    fn from(float: f64) -> Dec64 {
        if float < 0.0 {
            let (coefficient, exponent) = grisu2::convert(-float);

            Dec64::from_parts(-(coefficient as i64), exponent as i8)
        } else {
            let (coefficient, exponent) = grisu2::convert(float);

            Dec64::from_parts(coefficient as i64, exponent as i8)
        }
    }
}

impl From<f32> for Dec64 {
    fn from(float: f32) -> Dec64 {
        if float < 0.0 {
            let (coefficient, exponent) = grisu2::convert(-float as f64);

            Dec64::from_parts(-(coefficient as i64), exponent as i8)
        } else {
            let (coefficient, exponent) = grisu2::convert(float as f64);

            Dec64::from_parts(coefficient as i64, exponent as i8)
        }
    }
}

macro_rules! impl_integer {
    ($( $t:ty ),*) => ($(
        impl From<$t> for Dec64 {
            fn from(num: $t) -> Dec64 {
                dec64_raw!((num as i64) << 8)
            }
        }

        impl From<Dec64> for $t {
            fn from(dec: Dec64) -> $t {
                let exponent = dec.exponent();

                if exponent <= 0 {
                    dec.coefficient() as $t
                } else {
                    // This may overflow, which is fine
                    (dec.coefficient() * 10i64.pow(exponent as u32)) as $t
                }
            }
        }
    )*)
}

impl_integer!(usize, u8, u16, u32, u64, isize, i8, i16, i32, i64);

//! Implementation of traits from `std::ops`.

use std::ops::{
    Add,
};

use super::{
    Dec64,
    COEFFICIENT_MASK,
    NAN,
};

impl Add for Dec64 {
    type Output = Dec64;

    fn add(self, other: Dec64) -> Dec64 {
        let _sum_overflown = if self.exponent() == 0 && other.exponent() == 0 {
            // If the two exponents are both zero (which is usually the case for integers)
            // we can take the fast path. Since the exponents are both zero, we can simply
            // add the numbers together and check for overflow.
            let (sum, overflow) = self.value.overflowing_add(other.value);
            if !overflow {
                return dec64_raw!(sum);
            }

            sum
        } else {
            // The slow path is taken if the two operands do not both have zero exponents.
            if self.is_nan() {
                // If the first operand is NaN return NaN.
                return NAN;
            } else if self.exponent() == other.exponent() {
                // The exponents match so we may add now. Zero out the exponents so there
                // will be no carry into the coefficients when the coefficients are added.
                // If the result is zero, then return the normal zero.
                let (sum, overflow) = (self.value & COEFFICIENT_MASK).overflowing_add(other.value & COEFFICIENT_MASK);
                if !overflow {
                    return dec64_parts!(sum >> 8, self.exponent());
                }

                sum
            } else {
                // The slower path is taken when neither operand is nan, and their
                // exponents are different.
                if other.is_nan() {
                    return NAN;
                }

                // Before addition can take place, the exponents
                // must be made to match.
                let (hi, lo) = if self.exponent() > other.exponent() {
                    (self, other)
                } else {
                    (other, self)
                };

                let mut lo_coefficient = lo.coefficient();
                // If lower value has zero coefficient return the higher.
                if lo_coefficient == 0 {
                    return hi;
                }
                let mut lo_exponent    = lo.exponent();
                let mut hi_coefficient = hi.coefficient();
                let mut hi_exponent    = hi.exponent();

                // First we will try to decrease the first exponent. When we decrease the exponent
                // by 1, we must also multiply the coefficient by 10. We can do this as long as
                // there is no overflow. We have 8 extra bits to work with, so we can do this
                // at least twice, possibly more.
                loop {
                    // Before decrementing the exponent, multiply.
                    let (hi_coefficient_mul_10, overflow) = hi_coefficient.overflowing_mul(10);
                    if overflow {
                        // We cannot decrease exponent any more.
                        break;
                    }

                    hi_exponent -= 1;
                    hi_coefficient = hi_coefficient_mul_10;

                    // Are the exponents equal yet?
                    if hi_exponent == lo_exponent {
                        // We can sum & then pack.
                        let sum = hi_coefficient + lo_coefficient;
                        return Self::pack(sum, hi_exponent as i32);
                    }
                }

                // If we're still not done yet and we cannot decrease the first exponent any more,
                // so we must instead try to increase the second exponent, which will result in
                // a loss of significance.
                // That is the heartbreak of floating point.
                loop {
                    lo_coefficient /= 10;
                    lo_exponent += 1;

                    // Are the exponents equal yet?
                    if hi_exponent == lo_exponent {
                        // We can sum & then pack.
                        let sum = hi_coefficient + lo_coefficient;
                        return Self::pack(sum, lo_exponent as i32);
                    }
                }
            }
        };

        // Sum had an overflow.
        // This path happens only when both exponents are the same.
        // Re-add shifted coefficients (this won't overflow) and pack.
        // In original implementation of this path is much more elegant,
        // But here we don't have access to the carry flag.
        let sum = self.coefficient() + other.coefficient();

        Self::pack(sum, self.exponent() as i32)
    }
}


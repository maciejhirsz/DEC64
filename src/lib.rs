mod diyfp;
mod grisu2;
mod write;

pub const MAX_COEFFICIENT: i64 = 36028797018963967;
pub const MIN_COEFFICIENT: i64 = -36028797018963968;

pub static NAN:  Dec64 = Dec64 {
    value: -128
};
pub static ZERO: Dec64 = Dec64 {
    value: 0
};
pub static MAX:  Dec64 = Dec64 {
    value: (MAX_COEFFICIENT << 8) | (127u8 as i64)
};
pub static MIN:  Dec64 = Dec64 {
    value: (MIN_COEFFICIENT << 8) | (129u8 as i64)
};

#[derive(Clone, Copy, Debug)]
pub struct Dec64 {
    value: i64
}

impl Dec64 {
    pub fn from_parts(coefficient: i64, exponent: i8) -> Self {
        Dec64 {
            // Double casting on exponent so we don't end up with bunch
            // of `1` bits on the left if the exponent is negative
            value: (coefficient << 8) | ((exponent as u8) as i64)
        }
    }

    #[inline]
    pub fn coefficient(self) -> i64 {
        self.value >> 8
    }

    #[inline]
    pub fn exponent(self) -> i8 {
        self.value as i8
    }

    #[inline]
    pub fn is_nan(self) -> bool {
        self.exponent() == -128
    }

    #[inline]
    pub fn is_zero(self) -> bool {
        self.coefficient() == 0
    }
}

impl PartialEq<Dec64> for Dec64 {
    fn eq(&self, other: &Dec64) -> bool {
        if self.value == other.value {
            return true;
        }

        if self.coefficient() | other.coefficient() == 0 {
            return true;
        }

        if self.exponent() == -128 && other.exponent() == -128 {
            return true;
        }

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
                Dec64 {
                    value: (num as i64) << 8
                }
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

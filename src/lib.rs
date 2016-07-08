mod diyfp;
mod grisu2;

use std::{io, mem, ptr, slice};

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

const DEC_DIGITS_LUT: &'static[u8] =
    b"0001020304050607080910111213141516171819\
      2021222324252627282930313233343536373839\
      4041424344454647484950515253545556575859\
      6061626364656667686970717273747576777879\
      8081828384858687888990919293949596979899";

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

    pub fn write<W: io::Write>(self, wr: &mut W) -> io::Result<()> {
        let coefficient = self.coefficient();
        let exponent    = self.exponent();

        if coefficient == 0 {
            return wr.write_all(b"0");
        } else if exponent == -128 {
            return wr.write_all(b"nan");
        }

        let is_nonnegative = coefficient >= 0;
        let mut n = if is_nonnegative {
            coefficient
        } else {
            try!(wr.write_all(b"-"));
            // convert the negative num to positive by summing 1 to it's 2 complement
            (!coefficient).wrapping_add(1)
        };
        let mut buf: [u8; 30] = unsafe { mem::uninitialized() };
        let mut curr = buf.len() as isize;
        let buf_ptr = buf.as_mut_ptr();
        let lut_ptr = DEC_DIGITS_LUT.as_ptr();

        unsafe {
            // eagerly decode 4 characters at a time
            while n >= 10000 {
                let rem = (n % 10000) as isize;
                n /= 10000;

                let d1 = (rem / 100) << 1;
                let d2 = (rem % 100) << 1;
                curr -= 4;
                ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
                ptr::copy_nonoverlapping(lut_ptr.offset(d2), buf_ptr.offset(curr + 2), 2);
            }

            // if we reach here numbers are <= 9999, so at most 4 chars long
            let mut n = n as isize; // possibly reduce 64bit math

            // decode 2 more chars, if > 2 chars
            if n >= 100 {
                let d1 = (n % 100) << 1;
                n /= 100;
                curr -= 2;
                ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
            }

            // decode last 1 or 2 chars
            if n < 10 {
                curr -= 1;
                *buf_ptr.offset(curr) = (n as u8) + 48;
            } else {
                let d1 = n << 1;
                curr -= 2;
                ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
            }

            if exponent < 0 {
                ptr::copy(buf_ptr.offset(1), buf_ptr, buf.len() - (-exponent as usize));
                *buf_ptr.offset((buf.len() as isize) + (exponent as isize) - 1) = b'.';
                curr -= 1;
            } //else if exponent > 0 { }
        }

        wr.write_all(unsafe {
            slice::from_raw_parts(buf_ptr.offset(curr), buf.len() - curr as usize)
        })
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

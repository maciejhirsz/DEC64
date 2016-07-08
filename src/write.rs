use std::{io, mem, ptr, slice};
use Dec64;

const DEC_DIGITS_LUT: &'static[u8] =
    b"0001020304050607080910111213141516171819\
      2021222324252627282930313233343536373839\
      4041424344454647484950515253545556575859\
      6061626364656667686970717273747576777879\
      8081828384858687888990919293949596979899";


const ZEROES: &'static[u8] = b"00000000000000000000000000000000";

impl Dec64 {
    pub fn write<W: io::Write>(self, wr: &mut W) -> io::Result<()> {
        let coefficient = self.coefficient();
        let exponent    = self.exponent();

        if coefficient == 0 {
            return wr.write_all(b"0");
        } else if exponent == -128 {
            return wr.write_all(b"nan");
        }

        let mut n = if coefficient >= 0 {
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
                let mut e = (!exponent).wrapping_add(1) as usize;

                if e > 30 {
                    // panic!("CURR {}", curr);
                    ptr::copy(buf_ptr.offset(1), buf_ptr, curr as usize);
                    *buf_ptr.offset(curr) = b'.';
                    curr -= 1;
                    e -= buf.len() - (curr as usize) - 2;

                    try!(wr.write_all(
                        slice::from_raw_parts(buf_ptr.offset(curr), buf.len() - curr as usize)
                    ));
                    try!(wr.write_all(b"e-"));

                    let e = e as isize;

                    return if e > 100 {
                        // 127 is highest exponent, so the first digit can be only `1`
                        try!(wr.write_all(b"1"));
                        wr.write_all(
                            slice::from_raw_parts(lut_ptr.offset((e % 100) << 1), 2)
                        )
                    } else if e > 10 {
                        wr.write_all(
                            slice::from_raw_parts(lut_ptr.offset(e << 1), 2)
                        )
                    } else {
                        wr.write_all(&[(e as u8) + b'0'])
                    };
                } else {
                    let written = (buf.len() - curr as usize) - 1;

                    if e > written {
                        let zeroprefill = e - written;
                        curr -= zeroprefill as isize;
                        ptr::copy_nonoverlapping(ZEROES.as_ptr(), buf_ptr.offset(curr), zeroprefill);
                    }

                    ptr::copy(buf_ptr.offset(1), buf_ptr, buf.len() - (e as usize));
                    *buf_ptr.offset((buf.len() as isize) + (exponent as isize) - 1) = b'.';
                    curr -= 1;
                }
            } else if exponent > 0 {
                try!(wr.write_all(
                    slice::from_raw_parts(buf_ptr.offset(curr), buf.len() - curr as usize)
                ));
                try!(wr.write_all(b"e"));

                let e = exponent as isize;

                return if e > 100 {
                    // 127 is highest exponent, so the first digit can be only `1`
                    try!(wr.write_all(b"1"));
                    wr.write_all(
                        slice::from_raw_parts(lut_ptr.offset((e % 100) << 1), 2)
                    )
                } else if e > 10 {
                    wr.write_all(
                        slice::from_raw_parts(lut_ptr.offset(e << 1), 2)
                    )
                } else {
                    wr.write_all(&[(e as u8) + b'0'])
                };
            }

            wr.write_all(
                slice::from_raw_parts(buf_ptr.offset(curr), buf.len() - curr as usize)
            )
        }
    }
}

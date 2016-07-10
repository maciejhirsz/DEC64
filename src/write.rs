use std::{io, mem, ptr, slice};
use Dec64;

const DEC_DIGITS_LUT: &'static[u8] =
    b"0001020304050607080910111213141516171819\
      2021222324252627282930313233343536373839\
      4041424344454647484950515253545556575859\
      6061626364656667686970717273747576777879\
      8081828384858687888990919293949596979899";

#[inline(always)]
unsafe fn write_num(n: &mut isize, curr: &mut isize, buf_ptr: *mut u8, lut_ptr: *const u8) {
    // eagerly decode 4 digits at a time
    while *n >= 10000 {
        let rem = *n % 10000;
        *n /= 10000;

        let d1 = (rem / 100) << 1;
        let d2 = (rem % 100) << 1;
        *curr -= 4;
        ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(*curr), 2);
        ptr::copy_nonoverlapping(lut_ptr.offset(d2), buf_ptr.offset(*curr + 2), 2);
    }

    // decode 2 more digits
    if *n >= 100 {
        let d1 = (*n % 100) << 1;
        *n /= 100;
        *curr -= 2;
        ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(*curr), 2);
    }

    // decode last 1 or 2 digits
    if *n < 10 {
        *curr -= 1;
        *buf_ptr.offset(*curr) = (*n as u8) + b'0';
    } else {
        let d1 = *n << 1;
        *curr -= 2;
        ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(*curr), 2);
    }
}

impl Dec64 {
    pub fn write<W: io::Write>(self, wr: &mut W) -> io::Result<()> {
        let mut n = self.coefficient() as isize;
        let e = self.exponent() as i16;

        if n == 0 {
            return wr.write_all(b"0");
        } else if e == -128 {
            return wr.write_all(b"nan");
        }

        if n < 0 {
            try!(wr.write_all(b"-"));
            // convert the negative num to positive by summing 1 to it's 2 complement
            n = -n;
        }

        let mut buf: [u8; 24] = unsafe { mem::uninitialized() };
        let mut curr = buf.len() as isize;
        let buf_ptr = buf.as_mut_ptr();
        let lut_ptr = DEC_DIGITS_LUT.as_ptr();

        unsafe {
            if e < 0 {
                let mut e = -e as u16;

                // Decimal number with a fraction that's fully printable
                if e < 18 {
                    // eagerly decode 4 digits at a time
                    for _ in 0 .. e >> 2 {
                        let rem = n % 10000;
                        n /= 10000;

                        let d1 = (rem / 100) << 1;
                        let d2 = (rem % 100) << 1;
                        curr -= 4;
                        ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
                        ptr::copy_nonoverlapping(lut_ptr.offset(d2), buf_ptr.offset(curr + 2), 2);
                    }

                    e &= 3;

                    // write the remaining 3, 2 or 1 digits
                    if e & 2 == 2 {
                        let d1 = (n % 100) << 1;
                        n /= 100;
                        curr -= 2;
                        ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
                    }

                    if e & 1 == 1 {
                        curr -= 1;
                        *buf_ptr.offset(curr) = ((n % 10) as u8) + b'0';
                        n /= 10;
                    }

                    curr -= 1;
                    *buf_ptr.offset(curr) = b'.';

                    write_num(&mut n, &mut curr, buf_ptr, lut_ptr);

                    return wr.write_all(
                        slice::from_raw_parts(buf_ptr.offset(curr), buf.len() - curr as usize)
                    );

                // Not easily printable, write down fraction, then full number, then exponent
                } else {
                    // Single digit, no fraction
                    if n < 10 {
                        curr -= 1;
                        *buf_ptr.offset(curr) = ((n % 10) as u8) + b'0';
                    } else {
                        // eagerly decode 4 digits at a time
                        while n >= 100000 {
                            let rem = n % 10000;
                            n /= 10000;

                            let d1 = (rem / 100) << 1;
                            let d2 = (rem % 100) << 1;
                            curr -= 4;
                            ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
                            ptr::copy_nonoverlapping(lut_ptr.offset(d2), buf_ptr.offset(curr + 2), 2);
                        }

                        // decode 2 more digits
                        if n >= 1000 {
                            let d1 = (n % 100) << 1;
                            n /= 100;
                            curr -= 2;
                            ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
                        }

                        // decode last 1 or 2 digits
                        if n < 100 {
                            curr -= 1;
                            *buf_ptr.offset(curr) = ((n % 10) as u8) + b'0';
                            n /= 10;
                        } else {
                            let d1 = (n % 100) << 1;
                            n /= 100;
                            curr -= 2;
                            ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
                        }

                        curr -= 1;
                        *buf_ptr.offset(curr) = b'.';
                    }
                }
            }

            write_num(&mut n, &mut curr, buf_ptr, lut_ptr);

            wr.write_all(
                slice::from_raw_parts(buf_ptr.offset(curr), buf.len() - curr as usize)
            )
        }
    }
}
// }
//

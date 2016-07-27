use std::fmt;

use super::Dec64;

impl fmt::Debug for Dec64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        static NO_TRAIT: &'static str = "-";
        let trait_zero = if self.is_zero() { "Z" } else { NO_TRAIT };
        let trait_nan = if self.is_nan() { "N" } else { NO_TRAIT };
        let trait_integer = if self.is_integer() { "I" } else { NO_TRAIT };
        write!(f, "Dec64 {{ [{}{}{}] coef: {} exp: {} raw: {:#x} }}",
               trait_nan,
               trait_zero,
               trait_integer,
               self.coefficient(),
               self.exponent(),
               self.value)
    }
}

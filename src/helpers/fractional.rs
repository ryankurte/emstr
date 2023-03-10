//! [Fractional] helper for displaying scaled integers as decimal values
//! 
//! ```
//! # use emstr::{EncodeStr, helpers::Fractional};
//! # let mut buff = [0u8; 32];
//! 
//! let f = Fractional::<i32>::new(1234056, 1_000);
//! let s = f.write_str(&mut buff).unwrap();
//! 
//! assert_eq!(s, "1234.056");
//! ```

use core::{
    fmt::{Display, Debug},
    ops::Div,
};

use num_traits::{PrimInt, Signed, FromPrimitive};

use crate::EncodeStr;

/// [Number] trait combines encoding / numeric methods for convenience
pub trait Number: EncodeStr + PrimInt + Signed + FromPrimitive + Div + Display + Debug + Sized {}

/// Automatic implementation over viable types
impl <T: EncodeStr + PrimInt + Signed + FromPrimitive + Div + Display + Debug + Sized> Number for T {}

/// Helper for encoding integers as decimals using a specified divisor
pub struct Fractional<N: Number> {
    /// Raw integer value
    pub value: N,
    /// Divisor to be applied for encoding
    pub divisor: N,
}

impl <N: Number> Fractional<N> {
    /// Create a new fractional wrapper with the provided value and divisor
    pub const fn new(value: N, divisor: N) -> Self {
        Self{
            value,
            divisor,
        }
    }
}

impl <N: Number> EncodeStr for Fractional<N> {
    fn len(&self) -> usize {
        let int_part = self.value / self.divisor;
        let dec_part = (self.value % self.divisor).abs();

        let mut n = int_part.len();
        
        // No decimal part, just display integer
        if dec_part.is_zero() {
            return n;
        }

        // Negative integer part, add -ve sign
        if int_part.is_zero() && self.value.is_negative() {
            n += 1;
        }

        // Decimal part, integer + (divisior - 1) + 1
        n += self.divisor.len();

        // Trim trailing zeroes
        let mut d = dec_part;
        while d % N::from_i8(10).unwrap() == N::zero() {
            d = d / N::from_i8(10).unwrap();
            n -= 1;
        }

        n
    }

    fn write(&self, buff: &mut [u8]) -> Result<usize, crate::Error> {
        let mut n = 0;

        // Split integer and decimal components
        let int_part = self.value / self.divisor;
        let dec_part = (self.value % self.divisor).abs();

        // Write -ve sign for -ve fractions
        if int_part.is_zero() && self.value.is_negative() {
            buff[n] = '-' as u8;
            n += 1;
        }

        // Write integer part
        n += int_part.write(&mut buff[n..])?;

        // Skip decimal portion for whole numbers
        if dec_part.is_zero() {
            return Ok(n)
        }

        n += '.'.write(&mut buff[n..])?;

        // Pad decimal portion with zeroes based on divisor (5 / 100 -> 0.05)
        if self.divisor.len() > dec_part.len()  {
            let padding = self.divisor.len() - dec_part.len() - 1;
            for _i in 0..padding {
                buff[n] = '0' as u8;
                n += 1;
            }
        }

        // Trim decimal part
        let mut d = dec_part;
        while d % N::from_i8(10).unwrap() == N::zero() {
            d = d / N::from_i8(10).unwrap();
        }
        
        // Write trimmed decimal part
        n += d.write(&mut buff[n..])?;

        Ok(n)
    }
}


#[cfg(test)]
mod test {
    use crate::EncodeStr;
    use super::{Fractional, Number};

    #[test]
    fn fractional_i16() {
        let tests = &[
            (10, 10, "1"),
            (10, 100, "0.1"),
            (-10, 100, "-0.1"),
            (15, 10, "1.5"),
            (105, 100, "1.05"),
            (-10, 10, "-1"),
            (-15, 10, "-1.5"),
            (-105, 100, "-1.05"),
            (23041, 1_000, "23.041"),
            (-23041, 1_000, "-23.041"),
        ];

        encode_frac::<i16>(tests);
    }

    #[test]
    fn fractional_i32() {
        let tests = &[
            (10, 10, "1"),
            (15, 10, "1.5"),
            (105, 100, "1.05"),
            (1050, 1000, "1.05"),
            (-10, 10, "-1"),
            (-15, 10, "-1.5"),
            (-105, 100, "-1.05"),
            (-1050, 1000, "-1.05"),
            (312214312, 1_000_000, "312.214312"),
        ];

        encode_frac::<i32>(tests);
    }
    #[test]
    fn fractional_i64() {
        let tests = &[
            (10, 10, "1"),
            (15, 10, "1.5"),
            (105, 100, "1.05"),
            (-10, 10, "-1"),
            (-15, 10, "-1.5"),
            (-105, 100, "-1.05"),
            (105, 1000, "0.105"),
            (-105, 1000, "-0.105"),
            (123473634214312, 1_000_000, "123473634.214312"),
            (40_000_001_000_000_000, 1_000_000_000_000, "40000.001"),
            (-40_000_123_000_000_000, 1_000_000_000_000, "-40000.123"),
        ];

        encode_frac::<i64>(tests);
    }

    fn encode_frac<N: Number>(tests: &[(N, N, &'static str)]) {
        for (v, d, s) in tests {
            println!("test v: {} d: {} s: {}", v, d, s);

            let d = Fractional::<N>::new(*v, *d);

            assert_eq!(d.len(), s.len(), "invalid length for value: {}", s);

            let mut buff = [0u8; 32];
            let v = d.write_str(&mut buff).unwrap();

            assert_eq!(&v, s, "encoding mismatch for value: {}", s);
        }
    }

}

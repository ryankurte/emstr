
use crate::Error;
use super::EncodeStr;

/// Character map for integer encoding
const CHAR_MAP: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

/// Helper macro for implementing unsigned integer string encoding
macro_rules! impl_uint_encode {
    ($t:ty) => {
        impl EncodeStr for $t {
            fn len(&self) -> usize {
                let mut v = *self;
                let mut n = 0;
        
                // Handle zero
                if v == 0 {
                    return 1;
                }

                while v > 0 {
                    v /= 10;
                    n += 1;
                }
        
                n
            }
        
            fn write(&self, buff: &mut [u8]) -> Result<usize, Error> {
                let n = self.len();
                let mut v = *self;
        
                if buff.len() < n {
                    return Err(Error::BufferLength);
                }
        
                for i in 0..n {
                    let r = (v % 10) as usize;
                    v /= 10;
        
                    buff[n - i - 1] = CHAR_MAP[r] as u8;
                }
        
                Ok(n)
            }
        }
    };
}

/// Helper macro for implementing signed integer string encoding
macro_rules! impl_sint_encode {
    ($t:ty) => {
        impl EncodeStr for $t {
            fn len(&self) -> usize {
                let mut v = *self;
                let mut n = 0;
        
                // Handle zero
                if v == 0 {
                    return 1;
                }

                // Handle negatives
                if v < 0 {
                    n += 1;
                    v = -v;
                }

                // Compute required characters
                while v > 0 {
                    v /= 10;
                    n += 1;
                }
        
                n
            }
        
            fn write(&self, buff: &mut [u8]) -> Result<usize, Error> {
                let n = self.len();
                let mut v = *self;
                
                // Check buffer length
                if buff.len() < n {
                    return Err(Error::BufferLength);
                }
        
                // Handle negatives
                let c = if v < 0 {
                    buff[0] = '-' as u8;
                    v = -v;
                    n - 1
                } else {
                    n
                };

                for i in 0..c {
                    let r = (v % 10) as usize;
                    v /= 10;
        
                    buff[n - i - 1] = CHAR_MAP[r] as u8;
                }
        
                Ok(n)
            }
        }
    };
}

impl_uint_encode!(u8);
impl_uint_encode!(u16);
impl_uint_encode!(u32);
impl_uint_encode!(u64);
impl_uint_encode!(usize);

impl_sint_encode!(i8);
impl_sint_encode!(i16);
impl_sint_encode!(i32);
impl_sint_encode!(i64);
impl_uint_encode!(isize);

#[cfg(test)]
mod test {
    extern crate alloc;

    use super::EncodeStr;

    #[test]
    fn encode_u8() {
        for i in 0..u8::MAX {
            let mut buff = [0u8; 32];

            let s = alloc::format!("{}", i);
            
            let e = i.write_str(&mut buff).unwrap();

            assert_eq!(e.len(), s.len());
            assert_eq!(e, &s);
        }
    }

    #[test]
    fn encode_i8() {
        for i in i8::MIN+1..i8::MAX {
            let mut buff = [0u8; 32];

            let s = alloc::format!("{}", i);
            
            let e = i.write_str(&mut buff).unwrap();

            assert_eq!(e.len(), s.len());
            assert_eq!(e, &s);
        }
    }

    #[test]
    fn encode_u16() {
        for i in 0..i16::MAX {
            let mut buff = [0u8; 32];

            let s = alloc::format!("{}", i);
            
            let e = i.write_str(&mut buff).unwrap();

            assert_eq!(e.len(), s.len());
            assert_eq!(e, &s);
        }
    }

    #[test]
    fn encode_i16() {
        for i in i16::MIN+1..i16::MAX {
            let mut buff = [0u8; 32];

            let s = alloc::format!("{}", i);
            
            let e = i.write_str(&mut buff).unwrap();

            assert_eq!(e.len(), s.len());
            assert_eq!(e, &s);
        }
    }
    
    
    #[test]
    fn encode_u64() {
        let tests: &[(u64, &str)] = &[
            (0, "0"),
            (1, "1"),
            (1243566, "1243566"),
            (u64::MAX, "18446744073709551615"),
        ];

        for (v, s) in tests {
            let mut buff = [0u8; 32];

            assert_eq!(v.len(), s.len(), "length mismatch for value: {}", v);

            let e = v.write_str(&mut buff).unwrap();

            assert_eq!(e, *s, "encode failed for value: {}", v);
        }
    }

    #[test]
    fn encode_i64() {
        let tests: &[(i64, &str)] = &[
            (0, "0"),
            (1, "1"),
            (-1, "-1"),
            (1243566, "1243566"),
            (-1243566, "-1243566"),
            (i64::MAX, "9223372036854775807"),
            (i64::MIN + 1, "-9223372036854775807"),
            // TODO: handle actual i64::MIN
        ];

        for (v, s) in tests {
            let mut buff = [0u8; 32];

            assert_eq!(v.len(), s.len(), "length mismatch for value: {}", v);

            let e = v.write_str(&mut buff).unwrap();

            assert_eq!(e, *s, "encode failed for value: {}", v);
        }
    }
}

//! Helpers for string concatenation / encoding / manipulation in an embedded no_std / alloc free environment
//! 
//! See [EncodeStr] for the base encode trait, and [write!] for constructing strings
//! 
//! ```
//! # use emstr::{EncodeStr, Error};
//! 
//! // context can be any types implementing [emstr::EncodeStr]
//! let name = "something";
//! let progress = 15u8;
//! 
//! // use [emstr::write!]` macro to concatentate encodable types into `buff`
//! let mut buff = [0u8; 32];
//! let n = emstr::write!(&mut buff[..], name, ' ', progress, '/', 100u8).unwrap();
//! 
//! // creating the expected output
//! assert_eq!(&buff[..n], b"something 15/100");
//! ```
//! 

#![cfg_attr(not(feature = "std"), no_std)]

mod error;
pub use error::Error;

mod types;

/// [EncodeStr] implemented for string writable types
pub trait EncodeStr {
    /// Fetch the encoded length of the object
    fn len(&self) -> usize;

    /// Encode to string using the provided buffer, returning the number
    /// of characters written to the buffer
    fn write(&self, buff: &mut [u8]) -> Result<usize, Error>;

    /// Helper to encode to a borrowed string
    fn write_str<'a>(&self, buff: &'a mut [u8]) -> Result<&'a str, Error> {
        // Encode to buffer
        let n = self.write(buff)?;
        // Attempt string conversion
        core::str::from_utf8(&buff[..n]).map_err(|_| Error::InvalidUtf8)
    }
}

/// Blanket impl for references implementing [EncodeStr]
impl <T: EncodeStr> EncodeStr for &T {
    fn len(&self) -> usize {
        <T as EncodeStr>::len(self)
    }

    fn write(&self, buff: &mut [u8]) -> Result<usize, Error> {
        <T as EncodeStr>::write(self, buff)
    }
}

/// Helper macro for joining [EncodeStr] types
#[macro_export]
macro_rules! write {
    ($b:expr, $($t:expr),+) => {
        |buff: &mut [u8]| -> Result<usize, Error>{
            let mut n = 0;
        
            $(
                n += EncodeStr::write(& $t, &mut buff[n..])?;
            )*

            Ok(n)
        }(&mut $b)
    }
}

#[cfg(test)]
mod test {
    use crate::{EncodeStr, Error};

    #[test]
    fn join_str() {
        let mut buff = [0u8; 32];

        let n = write!(buff, "a", ' ', "b", ' ', "c").unwrap();

        assert_eq!(n, 5);
        assert_eq!(&buff[..n], b"a b c");
    }

    #[test]
    fn join_ints() {
        let mut buff = [0u8; 32];

        let n = write!(buff, 12u8, '/', 100u8).unwrap();

        assert_eq!(n, 6);
        assert_eq!(&buff[..n], b"12/100");
    }
}

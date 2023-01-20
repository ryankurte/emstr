

//! [EncodeStr] implementations for primitive

use crate::{EncodeStr, Error};

mod int;

/// [EncodeStr] implementation for [str] references
impl EncodeStr for &str {
    fn len(&self) -> usize {
        self.as_bytes().len()
    }

    fn write(&self, buff: &mut [u8]) -> Result<usize, Error> {
        let n = self.len();
        if n > buff.len() {
            return Err(Error::BufferLength)
        }

        buff[..n].copy_from_slice(&self.as_bytes());

        Ok(n)
    }
}

/// [EncodeStr] implementation for [char]s
impl EncodeStr for char {
    fn len(&self) -> usize {
        1
    }

    fn write(&self, buff: &mut [u8]) -> Result<usize, Error> {
        if 1 > buff.len() {
            return Err(Error::BufferLength)
        }

        buff[0] = *self as u8;

        Ok(1)
    }
}

#[cfg(test)]
mod test {
    use crate::EncodeStr;

    #[test]
    fn write_str() {
        let v = "abc123";

        let mut buff = [0u8; 32];
        let n = v.write(&mut buff).unwrap();

        assert_eq!(n, 6);
        assert_eq!(&buff[..n], v.as_bytes());
    }

    #[test]
    fn encode_char() {
        let v = 'c';

        let mut buff = [0u8; 32];
        let n = v.write(&mut buff).unwrap();

        assert_eq!(n, 1);
        assert_eq!(&buff[..n], "c".as_bytes());
    }
}

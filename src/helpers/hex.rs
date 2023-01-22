//! [Hex] helper for displaying arrays as hex
//! 
//! ```
//! # use emstr::{EncodeStr, helpers::Hex};
//! # let mut buff = [0u8; 32];
//! 
//! let h = Hex(&[0x12, 0x34, 0xff]);
//! let s = h.write_str(&mut buff).unwrap();
//! 
//! assert_eq!(s, "1234ff");
//! ```
//! 

use crate::{EncodeStr, Error};

/// Wrapper type for encoding byte arrays as hex strings
pub struct Hex<B: AsRef<[u8]>>(pub B);

/// Value to character mapping
const HEX_MAP: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

/// [EncodeStr] implementation to write bytes as hex
impl <B: AsRef<[u8]>> EncodeStr for Hex<B> {
    fn len(&self) -> usize {
        let b = self.0.as_ref();
        b.len() * 2
    }

    fn write(&self, buff: &mut [u8]) -> Result<usize, Error> {
        let b = self.0.as_ref();
        // Check buffer length
        if buff.len() < b.len() * 2 {
            return Err(Error::BufferLength)
        }

        // Write out hex
        for i in 0..b.len() {
            let v = b[i] as usize;

            buff[i * 2] = HEX_MAP[(v >> 4) & 0x0F] as u8;
            buff[i * 2 + 1] = HEX_MAP[v & 0x0F] as u8;
        }

        Ok(b.len() * 2)
    }
}

#[cfg(test)]
mod test {
    use super::{Hex, EncodeStr, HEX_MAP};

    #[test]
    fn encode_hex() {
        let data = [0x00, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde];

        let mut buff = [0u8; 32];
        let v = Hex(data).write_str(&mut buff).unwrap();

        assert_eq!(v, "00123456789abcde");
    }

    #[test]
    fn hex_map() {
        for i in 0..HEX_MAP.len() {

            let a = HEX_MAP[i].to_string();
            let e = format!("{:x}", i);

            assert_eq!(&a, &e);
        }
    }
}

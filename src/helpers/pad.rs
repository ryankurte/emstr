use core::marker::PhantomData;

use crate::{EncodeStr, Error};

/// Helper for padding string encodable types
pub struct Pad<E: EncodeStr, M> {
    inner: E,
    width: usize,
    pad: char,
    mode: PhantomData<M>
}

/// Marker for left padding
pub struct Left;

/// Marker for right padding
pub struct Right;


/// Left padding, see [Pad]
pub type PadLeft<E> = Pad<E, Left>;

/// Right padding, see [Pad]
pub type PadRight<E> = Pad<E, Right>;


impl <E: EncodeStr, M> Pad<E, M> {
    /// Create a new pad wrapper with the provided inner encoder and width
    pub const fn new(inner: E, width: usize, pad: char) -> Self {
        Self{
            inner,
            width,
            pad,
            mode: PhantomData,
        }
    }
}

/// [EncodeStr] for [PadRight]
impl <E: EncodeStr> EncodeStr for PadRight<E> {
    fn len(&self) -> usize {
        self.width.max(self.inner.len())
    }

    fn write(&self, buff: &mut [u8]) -> Result<usize, Error> {
        let n = self.inner.len();
        let m = self.width.max(n);

        // Write inner value
        self.inner.write(buff)?;

        // Pad remaining space
        for i in n..m {
            buff[i] = self.pad as u8;
        }

        Ok(m)
    }
}

/// [EncodeStr] for [PadLeft]
impl <E: EncodeStr> EncodeStr for PadLeft<E> {
    fn len(&self) -> usize {
        self.width.max(self.inner.len())
    }

    fn write(&self, buff: &mut [u8]) -> Result<usize, Error> {
        let n = self.inner.len();
        let p = self.width.max(n) - n;

        // Write padding
        for i in 0..p {
            buff[i] = self.pad as u8;
        }

        // Write inner value
        self.inner.write(&mut buff[p..])?;

        Ok(n + p)
    }
}

#[cfg(test)]
mod test {
    use crate::write_str;

    use super::*;

    #[test]
    fn test_pad_right() {
        let mut buff = [0u8; 32];

        let tests = &[
            (PadRight::new("123", 6, ' '), "123   "),
            (PadRight::new("123", 2, ' '), "123"),
        ];

        for (p, s) in tests {
            let v = write_str!(&mut buff[..], p);
            assert_eq!(v, Ok(*s));
        }
    }

    #[test]
    fn test_pad_left() {
        let mut buff = [0u8; 32];

        let tests = &[
            (PadLeft::new("123", 6, ' '), "   123"),
            (PadLeft::new("123", 2, ' '), "123"),
        ];

        for (p, s) in tests {
            let v = write_str!(&mut buff[..], p);
            assert_eq!(v, Ok(*s));
        }
    }
}

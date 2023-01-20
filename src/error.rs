
/// Error type for string encoding
#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "thiserror", derive(thiserror::Error))]
pub enum Error {
    #[cfg_attr(feature = "thiserror", error("buffer length"))]
    BufferLength,
    #[cfg_attr(feature = "thiserror", error("invalid utf8"))]
    InvalidUtf8,
}

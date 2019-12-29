use core::fmt;

/// The error type for decoding a hex string into `Vec<u8>` or `[u8; N]`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FromHexError {
    /// An invalid character was found. Valid ones are: `0...9`, `a...f`
    /// or `A...F`.
    InvalidHexCharacter { c: char, index: usize },

    /// A hex string's length needs to be even, as two digits correspond to
    /// one byte.
    OddLength,

    /// If the hex string is decoded into a fixed sized container, such as an
    /// array, the hex string's length * 2 has to match the container's
    /// length.
    InvalidStringLength,
}

#[cfg(feature = "std")]
impl std::error::Error for FromHexError {}

impl fmt::Display for FromHexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::InvalidHexCharacter { c, index } => {
                write!(f, "Invalid character '{}' at position {}", c, index)
            }
            Self::OddLength => write!(f, "Odd number of digits"),
            Self::InvalidStringLength => write!(f, "Invalid string length"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_display() {
        assert_eq!(
            FromHexError::InvalidHexCharacter { c: '\n', index: 5 }.to_string(),
            "Invalid character '\n' at position 5"
        );

        assert_eq!(FromHexError::OddLength.to_string(), "Odd number of digits");
        assert_eq!(
            FromHexError::InvalidStringLength.to_string(),
            "Invalid string length"
        );
    }
}

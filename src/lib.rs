//!
//!
//! Popular alphabets for use with `nanoid::nanoid!()` macro
//!
//! ```
//! use nanoid_dictionary::NOLOOKALIKES;
//! use nanoid::nanoid;
//!
//! let id = nanoid!(21, NOLOOKALIKES);
//! assert_eq!(id.len(), 21);
//! println!("{}", id);
//!```

/// Standard NanoId alphabet (64 symbols)
/// English lowercase letters, uppercase letters, digits, and symbols '_' and '-'.
///
pub const ALPHABET_STD: &[char] = &[
    '_', '-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

/// Lowercase English letters: abcdefghijklmnopqrstuvwxyz
///
pub const LOWERCASE: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// Uppercase English letters: ABCDEFGHIJKLMNOPQRSTUVWXYZ
///
pub const UPPERCASE: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

/// Numbers from 0 to 9
///
pub const NUMBERS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

/// Lowercase English hexadecimal characters: 0123456789abcdef
///
pub const HEXADECIMAL_LOWERCASE: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

/// Uppercase English hexadecimal characters: 0123456789ABCDEF
///
pub const HEXADECIMAL_UPPERCASE: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

/// Combination of all the lowercase, uppercase characters and numbers from 0 to 9
/// Does not include any symbols or special characters
///
pub const ALPHANUMERIC: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

/// Combination of all the lowercase characters and numbers from 0 to 9
/// Does not include any symbols or special characters
///
pub const ALPHANUMERIC_LOWERCASE: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// Combination of all the uppercase characters and numbers from 0 to 9
/// Does not include any symbols or special characters
///
pub const ALPHANUMERIC_UPPERCASE: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

/// Numbers and english alphabet without lookalikes: 1, l, I, 0, O, o, u, v, 5, S, s, 2, Z.
/// Complete set: 346789ABCDEFGHJKLMNPQRTUVWXYabcdefghijkmnpqrtwxyz
///
pub const NOLOOKALIKES: &[char] = &[
    '3', '4', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N',
    'P', 'Q', 'R', 'T', 'U', 'V', 'W', 'X', 'Y', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'm', 'n', 'p', 'q', 'r', 't', 'w', 'x', 'y', 'z',
];

/// Same as noolookalikes but with removed vowels and following letters: 3, 4, x, X, V.
/// This list should protect you from accidentally getting obscene words in generated strings.
/// Complete set: 6789BCDFGHJKLMNPQRTWbcdfghjkmnpqrtwz
///
pub const NOLOOKALIKES_SAFE: &[char] = &[
    '6', '7', '8', '9', 'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'T',
    'W', 'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 't', 'w', 'z',
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_length() {
        assert_eq!(ALPHABET_STD.len(), 64);
    }

    #[test]
    fn lowercase_length() {
        assert_eq!(LOWERCASE.len(), 26);
    }

    #[test]
    fn uppercase_length() {
        assert_eq!(UPPERCASE.len(), 26);
    }

    #[test]
    fn alphanumeric_length() {
        assert_eq!(ALPHANUMERIC.len(), 62);
    }

    #[test]
    fn alphanumeric_lowercase_length() {
        assert_eq!(ALPHANUMERIC_LOWERCASE.len(), 36);
    }

    #[test]
    fn alphanumeric_uppercase_length() {
        assert_eq!(ALPHANUMERIC_UPPERCASE.len(), 36);
    }

    #[test]
    fn nolookalikes_length() {
        assert_eq!(NOLOOKALIKES.len(), 49);
    }

    #[test]
    fn nolookalikes_safe_length() {
        assert_eq!(NOLOOKALIKES_SAFE.len(), 36);
    }
}

pub const ALPHABET_STD: &[char] = &[
    '_', '-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub const LOWERCASE: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub const UPPERCASE: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub const NUMBERS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub const HEXADECIMAL_LOWERCASE: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

pub const HEXADECIMAL_UPPERCASE: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

pub const ALPHANUMERIC: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

pub const NOLOOKALIKES: &[char] = &[
    '3', '4', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N',
    'P', 'Q', 'R', 'T', 'U', 'V', 'W', 'X', 'Y', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'm', 'n', 'p', 'q', 'r', 't', 'w', 'x', 'y', 'z',
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
    fn nolookalikes_length() {
        assert_eq!(NOLOOKALIKES.len(), 49);
    }
}

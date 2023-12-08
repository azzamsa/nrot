/// rot mode
#[derive(Copy, Clone)]
pub enum Mode {
    /// shift each letter n places to the right
    Encrypt,
    /// shift each letter n places to the left
    Decrypt,
}

/// Get the value of 'a'
///
/// Whether it use 'A' or 'a' as initial value
/// to preserve the case.
///
fn get_first_alphabet_position(letter: u8) -> u8 {
    // preserve case
    if letter.is_ascii_uppercase() {
        b'A'
    } else {
        b'a'
    }
}

/// Get the index number from english alphabet order
///
fn get_letter_position(letter: u8) -> u8 {
    let a_position = get_first_alphabet_position(letter);
    letter - a_position
}

/// Transform single letter to ROT
///
/// The letter after rotated/shifted by n from its initial
/// value
///
/// ```
/// use nrot::{rot_letter, Mode};
/// let rotation = 13;
///
/// let input = b'a';
/// let result = b'n';
/// let encrypted = rot_letter(Mode::Encrypt, input, rotation);
/// assert_eq!(result, encrypted);

/// let input = b'n';
/// let result = b'a';
/// let encrypted = rot_letter(Mode::Decrypt, input, rotation);
/// assert_eq!(result, encrypted);
/// ```
pub fn rot_letter(mode: Mode, letter: u8, rotation: u8) -> u8 {
    let a_position = get_first_alphabet_position(letter);
    let letter_position = get_letter_position(letter);

    let shifted_position = match mode {
        Mode::Encrypt => (letter_position + rotation) % 26,
        Mode::Decrypt => {
            if letter_position < rotation {
                26 - (rotation - letter_position)
            } else {
                letter_position - rotation
            }
        }
    };

    a_position + shifted_position
}

/// Transform any input to rot
///
/// ```
/// use nrot::{rot, Mode};
/// let rotation = 13;
///
/// let input = b"Hello, World!";
/// let result = b"Uryyb, Jbeyq!";
/// let encrypted = rot(Mode::Encrypt, input, rotation);
/// assert_eq!(result.as_slice(), &encrypted);
///
/// let input = b"Uryyb, Jbeyq!";
/// let result = b"Hello, World!";
/// let encrypted = rot(Mode::Decrypt, input, rotation);
/// assert_eq!(result.as_slice(), &encrypted);
/// ```
pub fn rot(mode: Mode, inputs: &[u8], rotation: u8) -> Vec<u8> {
    inputs
        .iter()
        .map(|&input| {
            // only apply rot13 to ascii alphabetic characters
            if input.is_ascii_alphabetic() {
                rot_letter(mode, input, rotation)
            } else {
                input
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_alphabet_position() {
        assert_eq!(65, get_first_alphabet_position(b'A'));
        assert_eq!(b'A', get_first_alphabet_position(b'A'));

        assert_eq!(97, get_first_alphabet_position(b'a'));
        assert_eq!(b'a', get_first_alphabet_position(b'a'));
    }

    #[test]
    fn letter_position() {
        assert_eq!(0, get_letter_position(b'A'));
        assert_eq!(0, get_letter_position(b'a'));

        assert_eq!(25, get_letter_position(b'Z'));
        assert_eq!(25, get_letter_position(b'z'));
    }

    #[test]
    fn rot13_letter_encrypt() {
        let rotation = 13;

        let encrypted = rot(Mode::Encrypt, b"a", rotation);
        assert_eq!(b"n".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"A", rotation);
        assert_eq!(b"N".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"Z", rotation);
        assert_eq!(b"M".as_slice(), &encrypted);
    }

    #[test]
    fn rot13_letter_decrypt() {
        let rotation = 13;

        let encrypted = rot(Mode::Decrypt, b"n", rotation);
        assert_eq!(b"a".as_slice(), &encrypted);

        let encrypted = rot(Mode::Decrypt, b"N", rotation);
        assert_eq!(b"A".as_slice(), &encrypted);

        let encrypted = rot(Mode::Decrypt, b"M", rotation);
        assert_eq!(b"Z".as_slice(), &encrypted);
    }

    #[test]
    fn rot13_encrypt() {
        let rotation = 13;

        let encrypted = rot(Mode::Encrypt, b"rust", rotation);
        assert_eq!(b"ehfg".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"Hello, World!", rotation);
        assert_eq!(b"Uryyb, Jbeyq!".as_slice(), &encrypted);
    }

    #[test]
    fn rot13_dencrypt() {
        let rotation = 13;

        let encrypted = rot(Mode::Decrypt, b"ehfg", rotation);
        assert_eq!(b"rust".as_slice(), &encrypted);

        let encrypted = rot(Mode::Decrypt, b"Uryyb, Jbeyq!", rotation);
        assert_eq!(b"Hello, World!".as_slice(), &encrypted);
    }

    #[test]
    fn all_rotations_encrypt() {
        let pairs = vec![(1, b"svtu"), (12, b"dgef"), (25, b"qtrs")];

        for (rotation, expected) in pairs {
            let encrypted = rot(Mode::Encrypt, b"rust", rotation);
            assert_eq!(expected.as_slice(), &encrypted);
        }
    }

    #[test]
    fn offsite() {
        let encrypted = rot(Mode::Encrypt, b"a", 0);
        assert_eq!(b"a".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"a", 26);
        assert_eq!(b"a".as_slice(), &encrypted);
    }
}

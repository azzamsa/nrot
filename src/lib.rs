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
/// Wheter it use 'A' or 'a' as initial value
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

        let input = b"a";
        let result = b"n";
        let encrypted = rot(Mode::Encrypt, input, rotation);
        assert_eq!(result.as_slice(), &encrypted);

        let input = b"A";
        let result = b"N";
        let encrypted = rot(Mode::Encrypt, input, rotation);
        assert_eq!(result.as_slice(), &encrypted);

        let input = b"Z";
        let result = b"M";
        let encrypted = rot(Mode::Encrypt, input, rotation);
        assert_eq!(result.as_slice(), &encrypted);
    }

    #[test]
    fn rot13_letter_decrypt() {
        let rotation = 13;

        let input = b"n";
        let result = b"a";
        let encrypted = rot(Mode::Decrypt, input, rotation);
        assert_eq!(result.as_slice(), &encrypted);

        let input = b"N";
        let result = b"A";
        let encrypted = rot(Mode::Decrypt, input, rotation);
        assert_eq!(result.as_slice(), &encrypted);

        let input = b"M";
        let result = b"Z";
        let encrypted = rot(Mode::Decrypt, input, rotation);
        assert_eq!(result.as_slice(), &encrypted);
    }

    #[test]
    fn rot13_encrypt() {
        let rotation = 13;

        let input = b"rust";
        let result = b"ehfg";
        let encrypted = rot(Mode::Encrypt, input, rotation);
        assert_eq!(result.as_slice(), &encrypted);

        let input = b"Hello, World!";
        let result = b"Uryyb, Jbeyq!";
        let encrypted = rot(Mode::Encrypt, input, rotation);
        assert_eq!(result.as_slice(), &encrypted);
    }

    #[test]
    fn rot13_dencrypt() {
        let rotation = 13;

        let input = b"ehfg";
        let result = b"rust";
        let encrypted = rot(Mode::Decrypt, input, rotation);
        assert_eq!(result.as_slice(), &encrypted);

        let input = b"Uryyb, Jbeyq!";
        let result = b"Hello, World!";
        let encrypted = rot(Mode::Decrypt, input, rotation);
        assert_eq!(result.as_slice(), &encrypted);
    }

    #[test]
    fn all_rotations_encrypt() {
        let encrypted = rot(Mode::Encrypt, b"rust", 1);
        assert_eq!(b"svtu".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 2);
        assert_eq!(b"twuv".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 3);
        assert_eq!(b"uxvw".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 4);
        assert_eq!(b"vywx".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 5);
        assert_eq!(b"wzxy".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 6);
        assert_eq!(b"xayz".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 7);
        assert_eq!(b"ybza".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 8);
        assert_eq!(b"zcab".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 9);
        assert_eq!(b"adbc".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 10);
        assert_eq!(b"becd".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 11);
        assert_eq!(b"cfde".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 12);
        assert_eq!(b"dgef".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 13);
        assert_eq!(b"ehfg".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 14);
        assert_eq!(b"figh".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 15);
        assert_eq!(b"gjhi".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 16);
        assert_eq!(b"hkij".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 17);
        assert_eq!(b"iljk".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 18);
        assert_eq!(b"jmkl".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 19);
        assert_eq!(b"knlm".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 20);
        assert_eq!(b"lomn".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 21);
        assert_eq!(b"mpno".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 22);
        assert_eq!(b"nqop".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 23);
        assert_eq!(b"orpq".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 24);
        assert_eq!(b"psqr".as_slice(), &encrypted);

        let encrypted = rot(Mode::Encrypt, b"rust", 25);
        assert_eq!(b"qtrs".as_slice(), &encrypted);
    }
}

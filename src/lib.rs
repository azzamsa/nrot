/// rot13 mode
#[derive(Copy, Clone)]
pub enum Mode {
    /// shift each letter 13 places to the right
    Encrypt,
    /// shift each letter 13 places to the left
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

/// Transform single letter to ROT13
///
/// The letter after rotated/shifted by 13 from its initial
/// value
///
/// ```
/// use nrot::{rot13_letter, Mode};
///
/// let input = b'a';
/// let result = b'n';
/// let encrypted = rot13_letter(Mode::Encrypt, input);
/// assert_eq!(result, encrypted);

/// let input = b'n';
/// let result = b'a';
/// let encrypted = rot13_letter(Mode::Decrypt, input);
/// assert_eq!(result, encrypted);
/// ```
pub fn rot13_letter(mode: Mode, letter: u8) -> u8 {
    let a_position = get_first_alphabet_position(letter);
    let letter_position = get_letter_position(letter);

    let shifted_position = match mode {
        Mode::Encrypt => (letter_position + 13) % 26,
        Mode::Decrypt => {
            if letter_position < 13 {
                26 - (13 - letter_position)
            } else {
                letter_position - 13
            }
        }
    };

    a_position + shifted_position
}

/// Transform any input to rot13
///
/// ```
/// use nrot::{rot13, Mode};
///
/// let input = b"Hello, World!";
/// let result = b"Uryyb, Jbeyq!";
/// let encrypted = rot13(Mode::Encrypt, input);
/// assert_eq!(result.as_slice(), &encrypted);
///
/// let input = b"Uryyb, Jbeyq!";
/// let result = b"Hello, World!";
/// let encrypted = rot13(Mode::Decrypt, input);
/// assert_eq!(result.as_slice(), &encrypted);
/// ```
pub fn rot13(mode: Mode, inputs: &[u8]) -> Vec<u8> {
    inputs
        .iter()
        .map(|&input| {
            // only apply rot13 to ascii alphabetic characters
            if input.is_ascii_alphabetic() {
                rot13_letter(mode, input)
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
        let input = b"a";
        let result = b"n";
        let encrypted = rot13(Mode::Encrypt, input);
        assert_eq!(result.as_slice(), &encrypted);

        let input = b"A";
        let result = b"N";
        let encrypted = rot13(Mode::Encrypt, input);
        assert_eq!(result.as_slice(), &encrypted);

        let input = b"Z";
        let result = b"M";
        let encrypted = rot13(Mode::Encrypt, input);
        assert_eq!(result.as_slice(), &encrypted);
    }

    #[test]
    fn rot13_letter_decrypt() {
        let input = b"n";
        let result = b"a";
        let encrypted = rot13(Mode::Decrypt, input);
        assert_eq!(result.as_slice(), &encrypted);

        let input = b"N";
        let result = b"A";
        let encrypted = rot13(Mode::Decrypt, input);
        assert_eq!(result.as_slice(), &encrypted);

        let input = b"M";
        let result = b"Z";
        let encrypted = rot13(Mode::Decrypt, input);
        assert_eq!(result.as_slice(), &encrypted);
    }

    #[test]
    fn rot13_encrypt() {
        let input = b"rust";
        let result = b"ehfg";
        let encrypted = rot13(Mode::Encrypt, input);
        assert_eq!(result.as_slice(), &encrypted);

        let input = b"Hello, World!";
        let result = b"Uryyb, Jbeyq!";
        let encrypted = rot13(Mode::Encrypt, input);
        assert_eq!(result.as_slice(), &encrypted);
    }

    #[test]
    fn rot13_dencrypt() {
        let input = b"ehfg";
        let result = b"rust";
        let encrypted = rot13(Mode::Decrypt, input);
        assert_eq!(result.as_slice(), &encrypted);

        let input = b"Uryyb, Jbeyq!";
        let result = b"Hello, World!";
        let encrypted = rot13(Mode::Decrypt, input);
        assert_eq!(result.as_slice(), &encrypted);
    }
}

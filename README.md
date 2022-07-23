<div align="center">
  <h1>nROT</h1>

  <img src='docs/padlock.svg' width=80px />
  
  Simple letter substitution cipher 🔐️

  <a href="https://github.com/azzamsa/nrot/workflows/ci.yml">
    <img src="https://github.com/azzamsa/nrot/workflows/ci/badge.svg" alt="Build status" />
  </a>

  <a href="https://crates.io/crates/nrot">
    <img src="https://img.shields.io/crates/v/nrot.svg">
  </a>

  <a href="https://docs.rs/nrot/">
    <img src="https://docs.rs/nrot/badge.svg">
  </a>

  <a href="https://azzamsa.com/support/">
    <img alt="Sponsor me" src="https://img.shields.io/badge/Sponsor%20Me-%F0%9F%92%96-ff69b4">
  </a>

</div>

---

## Features

- ROT encryption & decryption
- Exhaustive testing

## Usage


``` rust
use nrot::{rot, rot_letter, Mode};

fn encrypt(input: String) {
    let rotation = 13;

    let input_length = input.len();
    let input_bytes = input.as_bytes();

    if input_length == 1 {
        let byte_result = rot_letter(Mode::Encrypt, input_bytes[0], rotation);
        println!("{}", String::from_utf8_lossy(&[byte_result]))
    } else {
        let bytes_result = rot(Mode::Encrypt, input_bytes, rotation);
        println!("{}", String::from_utf8_lossy(&bytes_result))
    };
}

fn decrypt(input: String) {
    let rotation = 13;

    let input_length = input.len();
    let input_bytes = input.as_bytes();

    if input_length == 1 {
        let byte_result = rot_letter(Mode::Decrypt, input_bytes[0], rotation);
        println!("{}", String::from_utf8_lossy(&[byte_result]))
    } else {
        let bytes_result = rot(Mode::Decrypt, input_bytes, rotation);
        println!("{}", String::from_utf8_lossy(&bytes_result))
    };
}

fn main() {
    let input = "Hello, world!".to_string();
    encrypt(input);

    let input = "Uryyb, jbeyq!".to_string();
    decrypt(input);
}
```

To learn more, see other [examples](examples/).

## Credits

- ROT13 implementation is inspired by [Cameron Phillips's ROT13](https://github.com/cameronp98/rot13) 
- [Noto Emoji](https://github.com/googlefonts/noto-emoji) 

use nrot::{rot13, rot13_letter, Mode};

fn encrypt(input: String) {
    let input_length = input.len();
    let input_bytes = input.as_bytes();

    if input_length == 1 {
        let byte_result = rot13_letter(Mode::Encrypt, input_bytes[0]);
        println!("{}", String::from_utf8_lossy(&[byte_result]))
    } else {
        let bytes_result = rot13(Mode::Encrypt, input_bytes);
        println!("{}", String::from_utf8_lossy(&bytes_result))
    };
}

fn decrypt(input: String) {
    let input_length = input.len();
    let input_bytes = input.as_bytes();

    if input_length == 1 {
        let byte_result = rot13_letter(Mode::Decrypt, input_bytes[0]);
        println!("{}", String::from_utf8_lossy(&[byte_result]))
    } else {
        let bytes_result = rot13(Mode::Decrypt, input_bytes);
        println!("{}", String::from_utf8_lossy(&bytes_result))
    };
}

fn main() {
    let input = "Hello, world!".to_string();
    encrypt(input);

    let input = "Uryyb, jbeyq!".to_string();
    decrypt(input);
}

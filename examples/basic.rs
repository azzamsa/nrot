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

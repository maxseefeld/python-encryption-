use crypto::symmetriccipher::{SynchronousStreamCipher, Encryptor};
use crypto::aessafe::{AesSafe128Encryptor, AesSafe128Decryptor};
use std::fs;

fn main() {
    let input_filename = "example.py";
    let plaintext = fs::read(input_filename).unwrap();

    let key = b"0123456789abcdef";

    let encryptor = AesSafe128Encryptor::new(key);

    let mut ciphertext = plaintext.to_vec();
    let mut encryptor_ctr = Encryptor::new(encryptor, &mut ciphertext, &mut [0; 16], false).unwrap();
    encryptor_ctr.process(&plaintext).unwrap();

    let output_filename = "example_encrypted.py";
    fs::write(output_filename, &ciphertext).unwrap();

    println!("Encryption successful!");
}

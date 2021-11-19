use std::{
    env,
    io::{Read, Write},
};

use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes128Gcm, Key, Nonce};

use rand::Rng;

fn generate_key_iv() -> ([u8; 16], [u8; 12]) {
    let mut rng = rand::thread_rng();

    let key: [u8; 16] = rng.gen();
    let iv: [u8; 12] = rng.gen();

    (key, iv)
}

fn main() {
    // Extract the first argument from the command line.
    let arg = env::args().nth(1).expect("need a filename");

    // Open the file.
    let mut f = std::fs::File::open(&arg).expect("file not found");

    // Read the file into a buffer.
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).expect("failed to read file");

    // Compress the buf
    let buf = lz4_flex::compress_prepend_size(&buf);

    let (key, nonce) = generate_key_iv();
    let aes_key = Key::from_slice(&key);
    let aes_nonce = Nonce::from_slice(&nonce);

    let cipher = Aes128Gcm::new(aes_key);

    let cipher_text = cipher
        .encrypt(aes_nonce, buf.as_ref())
        .expect("failed to encrypt");

    write_to_file(&cipher_text, "stub_data.bin").expect("failed to write file");
    write_to_file(&key, "stub_data_key.bin").expect("failed to write file");
    write_to_file(&nonce, "stub_data_nonce.bin").expect("failed to write file");

    println!("Done!");
}

fn write_to_file(data: &[u8], file_name: &str) -> std::io::Result<()> {
    let mut f = std::fs::File::create(file_name)?;
    f.write_all(data)?;
    Ok(())
}

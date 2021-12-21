use chacha20poly1305::aead::{Aead, NewAead};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use crypto::hkdf;
use crypto::sha2::Sha256;
use std::{
    fs::self,
    path::Path,
};

fn create_keys(password: &str, salt: &str, filename: &str) -> (Vec<u8>, Vec<u8>) {
    let hash = Sha256::new();
    let mut prk: Vec<u8> = vec![0; 32];
    let mut okm: Vec<u8> = vec![0; 32];
    let mut nonce_root: Vec<u8> = vec![0; 12];
    hkdf::hkdf_extract(
        hash,
        &salt.as_bytes()[..],
        &password.as_bytes()[..],
        &mut prk,
    );
    hkdf::hkdf_expand(hash, &prk[..], &filename.as_bytes()[..], &mut okm);
    hkdf::hkdf_expand(hash, &okm[..], &filename.as_bytes()[..], &mut nonce_root);
    (okm, nonce_root)
}

fn encrypt_file<T: Clone + AsRef<Path>>(filepath: T, password: &str, salt: &str) {
    let (okm, nonce_root) = create_keys(
        password,
        salt,
        Path::new(filepath.as_ref()).file_stem().unwrap().to_str().unwrap(),
    );
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&okm));
    let nonce = Nonce::from_slice(nonce_root.as_ref());

    let output = filepath.clone();
    let data = fs::read(filepath).unwrap();
    let ciphertext = cipher
        .encrypt(nonce.into(), data.as_ref())
        .expect("encryption failure!");

    fs::write(&output, ciphertext);
}

fn decrypt_file<T: Clone + AsRef<Path>>(filepath: T, password: &str, salt: &str) {
    let (okm, nonce_root) = create_keys(
        password,
        salt,
        Path::new(filepath.as_ref()).file_stem().unwrap().to_str().unwrap(),
    );
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&okm));
    let nonce = Nonce::from_slice(nonce_root.as_ref());

    let output = filepath.clone();
    let data = fs::read(filepath).unwrap();
    let plaintext = cipher
        .decrypt(nonce.into(), data.as_ref())
        .expect("Decryption failure!");

    fs::write(&output, plaintext);
}

fn main() {
    let pass = "hello";
    let salt = "world";
    let file = "testfile.txt";

    encrypt_file(file, pass, salt);
    decrypt_file(file, pass, salt);
}

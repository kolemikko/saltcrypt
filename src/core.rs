use chacha20poly1305::aead::{self, Aead, NewAead};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use crypto::hkdf;
use crypto::sha2::Sha256;
use std::{fs, path::Path};

#[derive(Debug)]
pub enum CoreError {
    Io(std::io::Error),
    Aead(aead::Error),
}

impl From<std::io::Error> for CoreError {
    fn from(err: std::io::Error) -> CoreError {
        CoreError::Io(err)
    }
}

impl From<aead::Error> for CoreError {
    fn from(err: aead::Error) -> CoreError {
        CoreError::Aead(err)
    }
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CoreError::Io(ref err) => write!(f, "IO error: {}", err),
            CoreError::Aead(ref err) => write!(f, "{}", err),
        }
    }
}

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

pub fn encrypt_file<P: AsRef<Path>>(
    filepath: P,
    password: &str,
    salt: &str,
) -> Result<(), CoreError> {
    let (okm, nonce_root) = create_keys(
        password,
        salt,
        Path::new(filepath.as_ref())
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap(),
    );
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&okm));
    let nonce = Nonce::from_slice(nonce_root.as_ref());
    let data = fs::read(&filepath)?;
    let ciphertext = cipher.encrypt(nonce.into(), data.as_ref())?;

    fs::write(&filepath, ciphertext)?;
    Ok(())
}

pub fn decrypt_file<P: AsRef<Path>>(
    filepath: P,
    password: &str,
    salt: &str,
) -> Result<(), CoreError> {
    let (okm, nonce_root) = create_keys(
        password,
        salt,
        Path::new(filepath.as_ref())
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap(),
    );

    let cipher = ChaCha20Poly1305::new(Key::from_slice(&okm));
    let nonce = Nonce::from_slice(nonce_root.as_ref());

    let data = fs::read(&filepath)?;
    let plaintext = cipher.decrypt(nonce.into(), data.as_ref())?;

    fs::write(&filepath, plaintext)?;
    Ok(())
}

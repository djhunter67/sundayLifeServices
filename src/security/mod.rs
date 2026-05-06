use base64::{Engine as _, engine::general_purpose};
/// This module holds the encryption, salt, and peppering of passwords
use chacha20::{
    ChaCha20,
    cipher::{KeyIvInit, StreamCipher},
};
use tracing::{info, instrument, warn};

const PEPPER: [u8; 12] = *b"the_pepperer";

/// Encryption Logic
#[derive(Debug)]
pub struct PassWorder {
    pw: String,
}

impl PassWorder {
    /// Implementor for the password
    #[instrument(
        name = "Password Encryption",
        level = "info",
        target = "sundayLifeServices web app",
        skip(pw)
    )]
    pub fn new(pw: String) -> Self {
        PassWorder { pw }
    }

    #[instrument(
        name = "User registration attempted",
        level = "info",
        target = "sundayLifeServices web app"
    )]
    pub fn get(self) -> String {
        self.pw
    }

    /// Encrypt the password
    #[instrument(
        name = "Password Encryption",
        level = "info",
        target = "sundayLifeServices web app",
        skip(self)
    )]
    pub fn encrypt(self) -> Self {
        let key: [u8; 32] = *b"an example very very secret key!";
        let nonce: [u8; 12] = *b"unique nonce";

        let mut encryptor = ChaCha20::new(&key.into(), &nonce.into());

        let mut cipher_text = self.pw.into_bytes();

        encryptor.apply_keystream(&mut cipher_text);

        info!("Encrypting");

        // let mut pw = Self::new(hex::encode(&cipher_text));

        // pw.pw.insert(16, '$');

        // pw
        Self::new(hex::encode(&cipher_text))
    }

    #[instrument(
        name = "Password Salting",
        level = "info",
        target = "sundayLifeServices web app",
        skip(self)
    )]
    pub fn salt(mut self) -> Self {
        let _salted = String::from("The salted deal");
        info!("Salting");

        let random_salt: [u8; 16] = rand::random();

        // self.pw += &String::from_utf8_lossy(&random_salt);

        self.pw
            .insert_str(0, &format!("{}$", &hex::encode(random_salt)));

        info!("The generated Salt: {}", hex::encode(random_salt));

        Self::new(self.pw)
    }

    #[instrument(
        name = "Password Peppering",
        level = "info",
        target = "sundayLifeServices web app",
        skip(self)
    )]
    pub fn pepper(mut self) -> Self {
        // self.pw += "the_pepper";

        let base_64_pepper = general_purpose::STANDARD.encode(PEPPER);

        self.pw += &String::from_utf8_lossy(base_64_pepper.as_bytes());
        info!("The Peppered PW: {}", self.pw);
        Self::new(self.pw)
    }

    // #[instrument(
    //     name = "Create Hash",
    //     level = "info",
    //     target = "sundayLifeServices web app",
    //     skip(self)
    // )]
    // pub async fn create_hash(mut self) -> String {
    //     let new_hash = self.pw.clone()

    //     String::new()
    // }

    #[instrument(
        name = "Password deconstructor",
        level = "info",
        target = "sundayLifeServices web app",
        skip(self)
    )]
    pub fn deconstruct(&self) -> (String, String, String) {
        let (salt, hash) = self.pw.split_once('$').expect("No split delimeter found");

        match general_purpose::STANDARD.decode(
            self.pw
                .split_at(self.pw.len() - general_purpose::STANDARD.encode(PEPPER).len())
                .1,
        ) {
            Ok(pepper) => (
                String::from(salt),
                String::from(hash),
                String::from_utf8_lossy(&pepper).to_string(),
            ),
            Err(err) => {
                tracing::error!("Base64 decode failure: {err:?}");
                (
                    String::from(salt),
                    String::from(hash),
                    String::from(self.pw.split_at(self.pw.len() - PEPPER.len()).1),
                )
            }
        }
    }
}

// Tests for the PassWorder struct
#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_password_encryption() {
        let pw = PassWorder::new("my_secret_password".to_string());
        let encrypted_pw = pw.encrypt();
        assert_ne!(encrypted_pw.get(), "my_secret_password");
    }

    #[test]
    fn test_password_peppering() {
        let pw = PassWorder::new("my_secret_password".to_string());
        let peppered_pw = pw.pepper();
        assert!(
            peppered_pw
                .get()
                .ends_with(general_purpose::STANDARD.encode(PEPPER).as_str())
        );
    }

    #[test]
    fn test_password_deconstruction() {
        let pw = PassWorder::new("my_secret_password".to_string());
        let salted_peppered_pw = pw.salt().pepper();
        let (salt, hash, pepper) = salted_peppered_pw.deconstruct();
        assert!(!salt.is_empty());
        assert!(!hash.is_empty());
        assert_eq!(
            pepper,
            PEPPER.iter().map(|b| *b as char).collect::<String>()
        );
    }

    #[test]
    fn test_pw_encryption_length() {
        let pw = PassWorder::new("my_secret_password".to_string());
        let encrypted_pw = pw.encrypt();
        assert_eq!(encrypted_pw.get().len(), 36);
    }

    #[test]
    fn test_pw_salt_uniqueness() {
        let pw1 = PassWorder::new("my_secret_password".to_string());
        let salted_pw1 = pw1.salt();
        let pw2 = PassWorder::new("my_secret_password".to_string());
        let salted_pw2 = pw2.salt();
        assert_ne!(salted_pw1.get(), salted_pw2.get());
    }

    #[test]
    fn test_pw_pepper_consistency() {
        let pw1 = PassWorder::new("my_secret_password".to_string());
        let pw2 = PassWorder::new("my_secret_password".to_string());
        let peppered_pw1 = pw1.pepper();
        let peppered_pw2 = pw2.pepper();
        assert_eq!(peppered_pw1.get(), peppered_pw2.get());
    }

    #[test]
    fn test_pw_contains_dollar_sign() {
        let pw = PassWorder::new("my_secret_password".to_string());
        let salted_pw = pw.encrypt().salt().pepper();
        assert!(salted_pw.get().contains('$'));
    }

    #[test]
    fn test_random_is_lenght_before_and_after_conversion() {
        let random_salt: [u8; 16] = rand::random();
        let key: [u8; 32] = *b"an example very very secret key!";
        let nonce: [u8; 12] = *b"unique nonce";

        let mut encryptor = ChaCha20::new(&key.into(), &nonce.into());

        let mut cipher_text = random_salt;

        encryptor.apply_keystream(&mut cipher_text);

        assert_eq!(random_salt.len(), cipher_text.len());
    }
}

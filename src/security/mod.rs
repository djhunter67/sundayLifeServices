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
    pub async fn encrypt(self) -> Self {
        let key: [u8; 32] = *b"an example very very secret key!";
        let nonce: [u8; 12] = *b"unique nonce";

        let mut encryptor = ChaCha20::new(&key.into(), &nonce.into());

        let mut cipher_text = self.pw.into_bytes();

        encryptor.apply_keystream(&mut cipher_text);

        info!("Encrypting");

        // Encrypt the pw

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
        tracing::warn!(
            "The random salt: {:?}",
            hex::encode(&*String::from_utf8_lossy(&random_salt))
        );

        self.pw += &String::from_utf8_lossy(&random_salt);

        // self.pw = random_salt.concat(format!("${}", self.pw).as_bytes());
        // self.pw.insert(0, '$');
        self.pw
            .insert_str(0, &format!("{}$", &String::from_utf8_lossy(&random_salt)));

        info!("The Salted PW: {}", hex::encode(&self.pw));

        if self.pw.contains('$') {
            let idx = self.pw.rfind('$');
            tracing::warn!("The dollar sign is at idx: {idx:#?}");
        } else {
            tracing::error!("The dollar sign is not found");
        }

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
        self.pw += &String::from_utf8_lossy(&PEPPER);
        info!("The Peppered PW: {}", self.pw);
        Self::new(self.pw)
    }
}

// Tests for the PassWorder struct
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_password_encryption() {
        let pw = PassWorder::new("my_secret_password".to_string());
        let encrypted_pw = pw.encrypt().await;
        assert_ne!(encrypted_pw.get(), "my_secret_password");
    }

    #[test]
    fn test_password_salting() {
        let pw = PassWorder::new("my_secret_password".to_string());
        let salted_pw = pw.salt();
        assert!(salted_pw.get().contains('$'));
    }

    #[test]
    fn test_password_peppering() {
        let pw = PassWorder::new("my_secret_password".to_string());
        let peppered_pw = pw.pepper();
        assert!(
            peppered_pw
                .get()
                .ends_with(&*String::from_utf8_lossy(&PEPPER))
        );
    }
}

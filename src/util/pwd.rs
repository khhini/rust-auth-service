use std::error::Error;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, Params, PasswordHash, PasswordVerifier,
};

use crate::config::PwdConfig;

pub struct Pwd<'a> {
    argon2: Argon2<'a>,
}

impl<'a> Pwd<'a> {
    pub fn new(pwd_config: &'a PwdConfig) -> Self {
        Self {
            argon2: Argon2::new_with_secret(
                pwd_config.secret.as_bytes(),
                argon2::Algorithm::Argon2id,
                argon2::Version::V0x13,
                Params::default(),
            )
            .unwrap(),
        }
    }

    pub fn generate_password_hash(&self, password: &str) -> Result<String, Box<dyn Error>> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self
            .argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        Ok(password_hash)
    }

    pub fn verify_password_hash(
        &self,
        password: &str,
        password_hash: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let parsed_hash = PasswordHash::new(password_hash).unwrap();
        Ok(self
            .argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_generate_and_verify_password() {
        let pwd_config = PwdConfig::default();
        let pwd = Pwd::new(&pwd_config);
        let password = "SuperDuperSecretPassword";

        let result_hashed_password = pwd.generate_password_hash(password);
        assert!(result_hashed_password.is_ok());

        let hashed_password = result_hashed_password.unwrap();
        let result_verify_password = pwd.verify_password_hash(password, hashed_password.as_str());
        assert!(result_verify_password.is_ok());

        let verify_status = result_verify_password.ok().unwrap();
        assert!(verify_status);

        let wrong_password = "SuperDuperWrongPassword";
        let result_verify_password =
            pwd.verify_password_hash(wrong_password, hashed_password.as_str());
        assert!(result_verify_password.is_ok());

        let verify_status = result_verify_password.ok().unwrap();
        assert!(!verify_status);
    }
}

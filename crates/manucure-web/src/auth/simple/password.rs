use crate::auth::error::AuthError;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use secrecy::{ExposeSecret, Secret};

pub struct Credentials {
    email: String,
    password: Secret<String>,
}

fn verify_password_hash(
    expected_hash: Secret<String>,
    password_candidate: Secret<String>,
) -> Result<(), AuthError> {
    let expected_password_hash =
        PasswordHash::new(expected_hash.expose_secret()).map_err(AuthError::HashError)?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .map_err(|_| AuthError::InvalidCredentials)
}

impl Credentials {
    fn compute_password_hash(&self) -> Result<Secret<String>, anyhow::Error> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        )
        .hash_password(self.password.expose_secret().as_bytes(), &salt)
        .map_err(AuthError::HashError)?
        .to_string();

        Ok(Secret::new(password_hash))
    }
}

#[cfg(test)]
mod test {
    use crate::auth::simple::password::{verify_password_hash, Credentials};
    use secrecy::Secret;

    #[test]
    fn should_verify_password_ok() {
        let credentials = Credentials {
            email: "tom.bombadil@example.org".to_string(),
            password: Secret::new("hunter2".to_string()),
        };

        let expected_hash = credentials.compute_password_hash().unwrap();
        let candidate = Secret::new("hunter2".to_string());

        assert!(verify_password_hash(expected_hash, candidate).is_ok())
    }

    #[test]
    fn should_verify_password_err() {
        let credentials = Credentials {
            email: "tom.bombadil@example.org".to_string(),
            password: Secret::new("hunter2".to_string()),
        };

        let expected_hash = credentials.compute_password_hash().unwrap();
        let candidate = Secret::new("toto".to_string());

        assert!(verify_password_hash(expected_hash, candidate).is_err())
    }
}

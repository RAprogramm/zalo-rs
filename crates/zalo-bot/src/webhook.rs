use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::error::{BotResult, SignatureError};

type HmacSha256 = Hmac<Sha256>;

/// Verifies webhook signatures sent by the Zalo platform.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebhookVerifier {
    secret: Vec<u8>,
}

impl WebhookVerifier {
    /// Creates a new verifier using the provided shared secret.
    ///
    /// # Errors
    ///
    /// Returns [`SignatureError::InvalidSecretLength`] when the secret is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_bot::webhook::WebhookVerifier;
    ///
    /// let verifier = WebhookVerifier::new("top-secret")?;
    /// let signature = verifier.sign_payload(b"payload")?;
    /// verifier.verify(b"payload", Some(&signature))?;
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(secret: impl AsRef<[u8]>) -> Result<Self, SignatureError> {
        let secret_bytes = secret.as_ref();
        // Ensure the secret satisfies the requirements of the underlying HMAC
        // implementation.
        HmacSha256::new_from_slice(secret_bytes)?;

        Ok(Self {
            secret: secret_bytes.to_vec(),
        })
    }

    /// Computes the expected signature for a payload.
    pub fn sign_payload(&self, payload: &[u8]) -> Result<String, SignatureError> {
        let mut mac = HmacSha256::new_from_slice(&self.secret)?;
        mac.update(payload);
        let result = mac.finalize().into_bytes();
        Ok(hex::encode(result))
    }

    /// Validates the provided signature against the payload.
    ///
    /// # Errors
    ///
    /// Returns [`SignatureError::Missing`] when the signature header is absent
    /// and [`SignatureError::VerificationFailed`] when the signature does not
    /// match the payload.
    pub fn verify(&self, payload: &[u8], signature: Option<&str>) -> BotResult<()> {
        let signature = signature.ok_or(SignatureError::Missing)?;
        let signature_bytes =
            hex::decode(signature).map_err(|_| SignatureError::VerificationFailed)?;
        let mut mac = HmacSha256::new_from_slice(&self.secret).map_err(SignatureError::from)?;
        mac.update(payload);
        mac.verify_slice(&signature_bytes)
            .map_err(|_| SignatureError::VerificationFailed)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifies_valid_signature() {
        let verifier = WebhookVerifier::new("secret").expect("verifier");
        let payload = br#"{"event":"ping"}"#;
        let signature = verifier.sign_payload(payload).expect("signature");

        verifier
            .verify(payload, Some(&signature))
            .expect("signature should validate");
    }

    #[test]
    fn rejects_missing_signature() {
        let verifier = WebhookVerifier::new("secret").expect("verifier");
        let error = verifier.verify(b"payload", None).expect_err("missing");
        let app_error = zalo_types::AppError::from(error);

        assert!(matches!(
            app_error.kind,
            zalo_types::AppErrorKind::Unauthorized
        ));
    }

    #[test]
    fn rejects_invalid_signature() {
        let verifier = WebhookVerifier::new("secret").expect("verifier");
        let error = verifier
            .verify(b"payload", Some("deadbeef"))
            .expect_err("invalid signature");

        assert!(matches!(
            error,
            crate::error::BotError::Signature(SignatureError::VerificationFailed)
        ));
    }
}

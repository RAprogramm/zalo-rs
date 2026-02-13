// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::error::{BotResult, SignatureError};

type HmacSha256 = Hmac<Sha256>;

/// Verifies webhook signatures sent by the Zalo platform.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebhookVerifier {
    secret: Vec<u8>
}

impl WebhookVerifier {
    /// Creates a new verifier using the provided shared secret.
    ///
    /// # Errors
    ///
    /// Returns [`SignatureError::InvalidSecretLength`] when the secret is
    /// empty.
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
        HmacSha256::new_from_slice(secret_bytes)?;

        Ok(Self {
            secret: secret_bytes.to_vec()
        })
    }

    /// Computes the expected HMAC-SHA256 signature for a payload.
    ///
    /// The returned value is a lowercase hex-encoded string suitable for
    /// inclusion in a response header or comparison against an incoming value.
    ///
    /// # Errors
    ///
    /// Returns [`SignatureError::InvalidSecretLength`] when the stored secret
    /// is rejected by the HMAC implementation (should not occur in practice
    /// after successful construction).
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_bot::webhook::WebhookVerifier;
    ///
    /// let verifier = WebhookVerifier::new("secret")?;
    /// let sig = verifier.sign_payload(b"hello")?;
    /// assert!(!sig.is_empty());
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_bot::webhook::WebhookVerifier;
    ///
    /// let verifier = WebhookVerifier::new("secret")?;
    /// let sig = verifier.sign_payload(b"body")?;
    /// verifier.verify(b"body", Some(&sig))?;
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
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
    use zalo_types::AppErrorKind;

    use super::*;
    use crate::error::SignatureError;

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

        assert!(matches!(app_error.kind, AppErrorKind::Unauthorized));
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

    #[test]
    fn rejects_signature_for_wrong_payload() {
        let verifier = WebhookVerifier::new("secret").expect("verifier");
        let correct_sig = verifier.sign_payload(b"correct").expect("signature");
        let error = verifier
            .verify(b"wrong", Some(&correct_sig))
            .expect_err("wrong payload");

        assert!(matches!(
            error,
            crate::error::BotError::Signature(SignatureError::VerificationFailed)
        ));
    }

    #[test]
    fn sign_payload_is_deterministic() {
        let verifier = WebhookVerifier::new("secret").expect("verifier");
        let sig1 = verifier.sign_payload(b"same").expect("sig1");
        let sig2 = verifier.sign_payload(b"same").expect("sig2");

        assert_eq!(sig1, sig2);
    }

    #[test]
    fn different_payloads_produce_different_signatures() {
        let verifier = WebhookVerifier::new("secret").expect("verifier");
        let sig1 = verifier.sign_payload(b"payload-a").expect("sig1");
        let sig2 = verifier.sign_payload(b"payload-b").expect("sig2");

        assert_ne!(sig1, sig2);
    }

    #[test]
    fn rejects_non_hex_signature() {
        let verifier = WebhookVerifier::new("secret").expect("verifier");
        let error = verifier
            .verify(b"payload", Some("not-valid-hex!!"))
            .expect_err("non-hex signature");

        assert!(matches!(
            error,
            crate::error::BotError::Signature(SignatureError::VerificationFailed)
        ));
    }
}

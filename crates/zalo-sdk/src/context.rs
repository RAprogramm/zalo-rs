use serde::{Deserialize, Serialize};

use crate::error::{SdkError, SdkResult};

/// Immutable context capturing identifiers required by the mini app runtime.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MiniAppContext {
    app_id: String,
    oa_id: String,
}

impl MiniAppContext {
    /// Builds a new context after validating the identifiers.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::InvalidAppId`] or [`SdkError::InvalidOaId`] when the
    /// provided values are empty or contain only whitespace.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::MiniAppContext;
    ///
    /// let context = MiniAppContext::new("app", "oa")?;
    /// assert_eq!(context.app_id(), "app");
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(app_id: impl Into<String>, oa_id: impl Into<String>) -> SdkResult<Self> {
        let app_id = app_id.into();
        if app_id.trim().is_empty() {
            return Err(SdkError::InvalidAppId(app_id));
        }

        let oa_id = oa_id.into();
        if oa_id.trim().is_empty() {
            return Err(SdkError::InvalidOaId(oa_id));
        }

        Ok(Self { app_id, oa_id })
    }

    /// Returns the configured application identifier.
    #[must_use]
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    /// Returns the configured OA identifier.
    #[must_use]
    pub fn oa_id(&self) -> &str {
        &self.oa_id
    }

    /// Produces a handshake payload suitable for serialisation.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::json;
    /// use zalo_sdk::MiniAppContext;
    ///
    /// let payload = MiniAppContext::new("app", "oa")?.handshake_payload();
    /// assert_eq!(json!({ "app_id": "app", "oa_id": "oa" }), serde_json::to_value(&payload)?);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn handshake_payload(&self) -> HandshakePayload {
        HandshakePayload {
            app_id: self.app_id.clone(),
            oa_id: self.oa_id.clone(),
        }
    }
}

/// JSON-serialisable handshake payload shared with the host container.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HandshakePayload {
    app_id: String,
    oa_id: String,
}

impl HandshakePayload {
    /// Returns the application identifier included in the payload.
    #[must_use]
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    /// Returns the OA identifier included in the payload.
    #[must_use]
    pub fn oa_id(&self) -> &str {
        &self.oa_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_app_identifier() {
        let error = MiniAppContext::new("", "oa").expect_err("empty app id");

        assert!(matches!(error, SdkError::InvalidAppId(_)));
    }

    #[test]
    fn produces_handshake_payload() {
        let context = MiniAppContext::new("app", "oa").expect("context");
        let payload = context.handshake_payload();

        assert_eq!(payload.app_id(), "app");
        assert_eq!(payload.oa_id(), "oa");
        let json = serde_json::to_string(&payload).expect("serialise");
        assert!(json.contains("\"app_id\":\"app\""));
    }
}

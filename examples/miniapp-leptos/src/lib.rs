#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Minimal Leptos/Yew demo helpers built on top of `zalo-sdk`.

use zalo_sdk::{MiniAppContext, SdkResult};

/// Constructs a demo context suitable for bootstrapping the sample mini app.
///
/// The function showcases how application code can reuse [`MiniAppContext`] to
/// derive handshake payloads without touching lower-level primitives.
///
/// # Examples
///
/// ```
/// use miniapp_leptos::build_demo_context;
///
/// let context = build_demo_context()?;
/// assert_eq!(context.oa_id(), "demo-oa");
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
pub fn build_demo_context() -> SdkResult<MiniAppContext> {
    MiniAppContext::new("demo-app", "demo-oa")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_context_produces_handshake_payload() {
        let context = build_demo_context().expect("context");
        let payload = context.handshake_payload();

        assert_eq!(payload.app_id(), "demo-app");
        assert_eq!(payload.oa_id(), "demo-oa");
    }
}

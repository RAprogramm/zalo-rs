// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Zalo Mini App example using Leptos 0.8 with signals.
//!
//! This example demonstrates how to build a Mini App
//! that integrates with Zalo's JavaScript Bridge.

use leptos::{prelude::*, wasm_bindgen::JsCast};
use zalo_sdk::MiniAppContext;

/// Application configuration.
const APP_ID: &str = "your-app-id";
const OA_ID: &str = "your-oa-id";

/// User information structure.
#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, Default)]
struct UserInfo {
    user_id:      String,
    display_name: String,
    avatar:       String
}

/// Root component.
#[component]
fn App() -> impl IntoView {
    // Initialize Mini App context with error handling
    let ctx = MiniAppContext::new(APP_ID, OA_ID).ok();

    // Create signals using signal() as per Leptos 0.8 examples
    let (user_info, set_user_info) = signal(None::<UserInfo>);
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);
    let (debug_open, set_debug_open) = signal(false);

    // Get handshake payload
    let handshake = ctx.as_ref().map(|c| c.handshake_payload());

    // Authorize handler
    let on_authorize = move |_| {
        set_loading.set(true);
        set_error.set(None);

        // Simulate authorization (in real app, call Zalo SDK)
        set_timeout(
            move || {
                set_user_info.set(Some(UserInfo {
                    user_id:      "1234567890".to_string(),
                    display_name: "Nguyen Van A".to_string(),
                    avatar:       "https://avatar.zdn.vn/default.jpg".to_string()
                }));
                set_loading.set(false);
            },
            std::time::Duration::from_millis(500)
        );
    };

    // Get user info handler
    let on_get_user_info = move |_| {
        if user_info.get().is_none() {
            set_error.set(Some("Please authorize first".to_string()));
            return;
        }
        log::info!("Getting user info...");
    };

    // Get phone handler
    let on_get_phone = move |_| {
        if user_info.get().is_none() {
            set_error.set(Some("Please authorize first".to_string()));
            return;
        }
        log::info!("Getting phone number...");
    };

    // Share handler
    let on_share = move |_| {
        if user_info.get().is_none() {
            set_error.set(Some("Please authorize first".to_string()));
            return;
        }
        log::info!("Sharing...");
    };

    // Navigate handler
    let on_navigate = move |_| {
        if user_info.get().is_none() {
            set_error.set(Some("Please authorize first".to_string()));
            return;
        }
        log::info!("Navigating...");
    };

    // Storage signals
    let (storage_key, set_storage_key) = signal(String::new());
    let (storage_value, set_storage_value) = signal(String::new());
    let (storage_result, set_storage_result) = signal(None::<String>);

    let on_storage_save = move |_| {
        set_storage_result.set(Some(format!(
            "Stored: {} = {}",
            storage_key.get(),
            storage_value.get()
        )));
    };

    let on_storage_load = move |_| {
        set_storage_result.set(Some(format!("Retrieved: {} = ???", storage_key.get())));
    };

    view! {
        <div class="container">
            <header>
                <h1>"Zalo Mini App"</h1>
                <p class="subtitle">"Leptos 0.8 + Signals"</p>
            </header>

            <main>
                <section class="card">
                    <h2>"User Information"</h2>

                    <Show when=move || loading.get()>
                        <div class="loading">"Loading..."</div>
                    </Show>
                    <Show when=move || error.get().is_some()>
                        <div class="error">{move || error.get().unwrap_or_default()}</div>
                    </Show>
                    <Show when=move || user_info.get().is_some()>
                        <div class="user-profile">
                            <img src={move || user_info.get().unwrap_or_default().avatar} alt="Avatar" class="avatar" />
                            <div class="user-details">
                                <h3>{move || user_info.get().unwrap_or_default().display_name}</h3>
                                <p class="user-id">{"ID: "}{move || user_info.get().unwrap_or_default().user_id}</p>
                            </div>
                        </div>
                    </Show>
                    <Show when=move || !loading.get() && error.get().is_none() && user_info.get().is_none()>
                        <button
                            class="btn btn-primary"
                            on:click=on_authorize
                        >
                            "Authorize with Zalo"
                        </button>
                    </Show>
                </section>

                <section class="card">
                    <h2>"Actions"</h2>
                    <div class="button-grid">
                        <button
                            class="btn"
                            disabled=move || !user_info.get().is_some()
                            on:click=on_get_user_info
                        >
                            "👤 Get User Info"
                        </button>

                        <button
                            class="btn"
                            disabled=move || !user_info.get().is_some()
                            on:click=on_get_phone
                        >
                            "📱 Get Phone"
                        </button>

                        <button
                            class="btn"
                            disabled=move || !user_info.get().is_some()
                            on:click=on_share
                        >
                            "🔗 Share"
                        </button>

                        <button
                            class="btn"
                            disabled=move || !user_info.get().is_some()
                            on:click=on_navigate
                        >
                            "🧭 Navigate"
                        </button>
                    </div>
                </section>

                <section class="card">
                    <h2>"Storage"</h2>
                    <div class="form-group">
                        <input
                            type="text"
                            placeholder="Key"
                            prop:value=move || storage_key.get()
                            on:input=move |ev: web_sys::Event| {
                                let value = ev.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value();
                                set_storage_key.set(value);
                            }
                            class="input"
                        />
                        <input
                            type="text"
                            placeholder="Value"
                            prop:value=move || storage_value.get()
                            on:input=move |ev: web_sys::Event| {
                                let value = ev.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value();
                                set_storage_value.set(value);
                            }
                            class="input"
                        />
                    </div>
                    <div class="button-group">
                        <button
                            class="btn btn-secondary"
                            on:click=on_storage_save
                        >
                            "💾 Save"
                        </button>
                        <button
                            class="btn btn-secondary"
                            on:click=on_storage_load
                        >
                            "📖 Load"
                        </button>
                    </div>
                    <Show when=move || storage_result.get().is_some()>
                        <div class="result">
                            {move || storage_result.get().unwrap_or_default()}
                        </div>
                    </Show>
                </section>

                <section class="card debug">
                    <h2>
                        "Debug Info"
                        <button
                            class="btn-toggle"
                            on:click=move |_| set_debug_open.update(|v| *v = !*v)
                        >
                            {move || if debug_open.get() { "Hide" } else { "Show" }}
                        </button>
                    </h2>

                    <Show when=move || debug_open.get()>
                        <div class="debug-content">
                            <h3>"Zalo SDK Status"</h3>
                            <pre class="code">
                                {match &ctx {
                                    Some(_) => "SDK initialized successfully",
                                    None => "SDK not available (running outside Zalo)"
                                }}
                            </pre>

                            <h3>"Handshake Payload"</h3>
                            <pre class="code">
                                {match &handshake {
                                    Some(h) => serde_json::to_string_pretty(h)
                                        .unwrap_or_else(|_| "Error".to_string()),
                                    None => "Not available".to_string()
                                }}
                            </pre>

                            <h3>"User State"</h3>
                            <pre class="code">
                                {format!("{:#?}", user_info.get())}
                            </pre>
                        </div>
                    </Show>
                </section>
            </main>

            <footer>
                <p>"Powered by Zalo SDK"</p>
            </footer>
        </div>
    }
}

/// Main entry point.
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

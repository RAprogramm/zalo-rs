// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Zalo Mini App example using Leptos 0.8 with signals.
//!
//! This example demonstrates how to build a Mini App
//! that integrates with Zalo's JavaScript Bridge.

use leptos::prelude::*;
use leptos::event_target_value;
use zalo_sdk::MiniAppContext;

/// Application configuration.
const APP_ID: &str = "your-app-id";
const OA_ID: &str = "your-oa-id";

/// User information structure.
#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, Default)]
struct UserInfo {
    user_id: String,
    display_name: String,
    avatar: String,
}

/// Root component.
#[component]
fn App() -> impl IntoView {
    // Initialize Mini App context
    let ctx = MiniAppContext::new(APP_ID, OA_ID)
        .expect("Failed to create MiniApp context");

    // Create signals for reactive state
    let user_info = create_rw_signal(None::<UserInfo>);
    let loading = create_rw_signal(false);
    let error = create_rw_signal(None::<String>);
    let debug_open = create_rw_signal(false);

    // Get handshake payload
    let handshake = ctx.handshake_payload();

    // Authorize callback
    let on_authorize = Callback::new(move |_| {
        loading.set(true);
        error.set(None);
        
        // Simulate authorization (in real app, call Zalo SDK)
        set_timeout(
            move || {
                user_info.set(Some(UserInfo {
                    user_id: "1234567890".to_string(),
                    display_name: "Nguyen Van A".to_string(),
                    avatar: "https://avatar.zdn.vn/default.jpg".to_string(),
                }));
                loading.set(false);
            },
            std::time::Duration::from_millis(500),
        );
    });

    // Get user info callback
    let on_get_user_info = Callback::new(move |_| {
        if user_info.get().is_none() {
            error.set(Some("Please authorize first".to_string()));
            return;
        }
        // In real implementation, call zalo_sdk::user::get_user_info()
        log::info!("Getting user info...");
    });

    // Get phone callback
    let on_get_phone = Callback::new(move |_| {
        if user_info.get().is_none() {
            error.set(Some("Please authorize first".to_string()));
            return;
        }
        // In real implementation, call zalo_sdk::user::get_phone_number()
        log::info!("Getting phone number...");
    });

    // Share callback
    let on_share = Callback::new(move |_| {
        if user_info.get().is_none() {
            error.set(Some("Please authorize first".to_string()));
            return;
        }
        // In real implementation, call zalo_sdk::share::share()
        log::info!("Sharing...");
    });

    // Navigate callback
    let on_navigate = Callback::new(move |_| {
        if user_info.get().is_none() {
            error.set(Some("Please authorize first".to_string()));
            return;
        }
        // In real implementation, call zalo_sdk::navigation::open_webview()
        log::info!("Navigating...");
    });

    // Storage signals
    let storage_key = create_rw_signal(String::new());
    let storage_value = create_rw_signal(String::new());
    let storage_result = create_rw_signal(None::<String>);

    let on_storage_save = Callback::new(move |_| {
        // In real implementation, call zalo_sdk::storage::setItem()
        storage_result.set(Some(format!(
            "Stored: {} = {}",
            storage_key.get(),
            storage_value.get()
        )));
    });

    let on_storage_load = Callback::new(move |_| {
        // In real implementation, call zalo_sdk::storage::getItem()
        storage_result.set(Some(format!(
            "Retrieved: {} = ???",
            storage_key.get()
        )));
    });

    // Derived signals
    let is_authenticated = Signal::derive(move || user_info.get().is_some());
    let user_display = Signal::derive(move || {
        user_info
            .get()
            .map(|info| info.display_name.clone())
            .unwrap_or_else(|| "Guest".to_string())
    });

    view! {
        <div class="container">
            <header>
                <h1>"Zalo Mini App"</h1>
                <p class="subtitle">"Leptos 0.8 + Signals"</p>
            </header>

            <main>
                // User info section
                <section class="card">
                    <h2>"User Information"</h2>

                    {move || {
                        if loading.get() {
                            view! { <div class="loading">"Loading..."</div> }
                        } else if let Some(err) = error.get() {
                            view! { <div class="error">{err}</div> }
                        } else if let Some(info) = user_info.get() {
                            view! {
                                <div class="user-profile">
                                    <img src={info.avatar} alt="Avatar" class="avatar" />
                                    <div class="user-details">
                                        <h3>{info.display_name}</h3>
                                        <p class="user-id">{"ID: "}{info.user_id}</p>
                                    </div>
                                </div>
                            }
                        } else {
                            view! {
                                <button
                                    class="btn btn-primary"
                                    on:click=on_authorize
                                >
                                    "Authorize with Zalo"
                                </button>
                            }
                        }
                    }}
                </section>

                // Action buttons
                <section class="card">
                    <h2>"Actions"</h2>
                    <div class="button-grid">
                        <button
                            class="btn"
                            disabled=move || !is_authenticated.get()
                            on:click=on_get_user_info
                        >
                            "👤 Get User Info"
                        </button>

                        <button
                            class="btn"
                            disabled=move || !is_authenticated.get()
                            on:click=on_get_phone
                        >
                            "📱 Get Phone"
                        </button>

                        <button
                            class="btn"
                            disabled=move || !is_authenticated.get()
                            on:click=on_share
                        >
                            "🔗 Share"
                        </button>

                        <button
                            class="btn"
                            disabled=move || !is_authenticated.get()
                            on:click=on_navigate
                        >
                            "🧭 Navigate"
                        </button>
                    </div>
                </section>

                // Storage section
                <section class="card">
                    <h2>"Storage"</h2>
                    <div class="form-group">
                        <input
                            type="text"
                            placeholder="Key"
                            value=move || storage_key.get()
                            on:input=move |ev| storage_key.set(event_target_value(&ev))
                            class="input"
                        />
                        <input
                            type="text"
                            placeholder="Value"
                            value=move || storage_value.get()
                            on:input=move |ev| storage_value.set(event_target_value(&ev))
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
                    {move || {
                        storage_result
                            .get()
                            .map(|result| view! { <div class="result">{result}</div> })
                    }}
                </section>

                // Debug info
                <section class="card debug">
                    <h2>
                        "Debug Info"
                        <button
                            class="btn-toggle"
                            on:click=move |_| debug_open.update(|v| *v = !*v)
                        >
                            {move || if debug_open.get() { "Hide" } else { "Show" }}
                        </button>
                    </h2>

                    {move || {
                        if debug_open.get() {
                            view! {
                                <div class="debug-content">
                                    <h3>"Handshake Payload"</h3>
                                    <pre class="code">
                                        {serde_json::to_string_pretty(&handshake)
                                            .unwrap_or_else(|_| "Error".to_string())}
                                    </pre>

                                    <h3>"User State"</h3>
                                    <pre class="code">
                                        {format!("{:#?}", user_info.get())}
                                    </pre>

                                    <h3>"Authenticated"</h3>
                                    <pre class="code">{is_authenticated.get()}</pre>

                                    <h3>"Display Name"</h3>
                                    <pre class="code">{user_display.get()}</pre>
                                </div>
                            }
                        } else {
                            view! {}
                        }
                    }}
                </section>
            </main>

            <footer>
                <p>"Powered by Zalo SDK"</p>
            </footer>
        </div>
    }
}

/// Main entry point.
fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(|| view! { <App /> });
}

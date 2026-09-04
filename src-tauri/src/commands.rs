//! Commands exposed to the frontend.
//!
//! Every `#[tauri::command]` here must also be listed in the
//! `tauri::generate_handler!` macro in `lib.rs`, and mirrored by a typed
//! wrapper in `src/bindings.ts`.

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::AppState;

/// Errors that can cross the IPC boundary.
///
/// Commands returning `Result<T, Error>` reject the JS promise on `Err`, so the
/// frontend can `try`/`catch` it. `serde(tag = "kind", content = "message")`
/// keeps the shape discriminable in TypeScript.
#[derive(Debug, Serialize)]
#[serde(tag = "kind", content = "message")]
pub enum Error {
    /// The caller passed something we can't work with.
    InvalidInput(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

/// Simplest possible command: takes a string, returns a string.
#[tauri::command]
pub fn greet(name: &str) -> Result<String, Error> {
    let name = name.trim();
    if name.is_empty() {
        return Err(Error::InvalidInput("name must not be empty".into()));
    }
    Ok(format!("Hello, {name}! You've been greeted from Rust."))
}

/// A struct payload. `rename_all = "camelCase"` so the TS side reads naturally.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub tauri_version: String,
    pub os: String,
    pub arch: String,
}

/// Reading from the Tauri handle — package metadata, platform, etc.
#[tauri::command]
pub fn app_info(app: tauri::AppHandle) -> AppInfo {
    let package = app.package_info();
    AppInfo {
        name: package.name.clone(),
        version: package.version.to_string(),
        tauri_version: tauri::VERSION.to_string(),
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
    }
}

/// A struct argument. Tauri deserializes the JS object into this.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CounterArgs {
    pub by: i64,
}

/// Managed state, mutated behind a lock and returned to the caller.
#[tauri::command]
pub fn bump_counter(args: CounterArgs, state: State<'_, AppState>) -> i64 {
    let mut counter = state.counter.lock().expect("counter mutex poisoned");
    *counter += args.by;
    *counter
}

/// `async` commands run off the main thread and don't block the UI.
#[tauri::command]
pub async fn slow_task(millis: u64) -> Result<String, Error> {
    if millis > 10_000 {
        return Err(Error::InvalidInput("millis must be <= 10000".into()));
    }
    tokio::time::sleep(std::time::Duration::from_millis(millis)).await;
    Ok(format!("Finished after {millis}ms, without freezing the window."))
}

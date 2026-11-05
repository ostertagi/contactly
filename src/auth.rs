//! Fake authentication backend.
//!
//! It is `async` on purpose: it lets you see how an Iced `Task` runs work in
//! the background and sends the result back to `update()` as a message.

use crate::model::Session;
use std::time::Duration;

const USERS: &[(&str, &str, &str)] = &[
    ("demo", "demo", "Demo User"),
    ("admin", "admin123", "Administrator"),
];

pub async fn login(username: String, password: String) -> Result<Session, String> {
    // Pretend we are talking to a server.
    tokio::time::sleep(Duration::from_millis(700)).await;

    let username = username.trim().to_lowercase();
    USERS
        .iter()
        .find(|(u, p, _)| *u == username && *p == password)
        .map(|(u, _, name)| Session {
            username: u.to_string(),
            display_name: name.to_string(),
        })
        .ok_or_else(|| "Invalid username or password".to_string())
}

//! # yambot
//!
//! Библиотека для создания ботов в **Yandex Messenger**.

pub mod bot;
pub mod client;
pub mod config;
pub mod error;
pub mod events;      // ← уже должен быть
pub mod types;

pub use bot::Bot;
pub use config::Config;
pub use error::{Result, YambotError};

// Re-export типов
pub use types::*;

// Удобный публичный API событий
pub use events::{Dispatcher, Handler};

/// Создаёт нового бота с токеном
pub fn new(token: impl Into<String>) -> Bot {
    Bot::new(token)
}
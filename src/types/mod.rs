//! Все типы данных Yandex Messenger Bot API

pub mod bot_request;
pub mod button;
pub mod chat;
pub mod directive;
pub mod file;
pub mod image;
pub mod inline_suggest_button;
pub mod message;
pub mod sender;
pub mod suggest_buttons;
pub mod update;
pub mod user;
pub mod requests;

// Re-exports для удобства
pub use bot_request::BotRequest;
pub use button::Button;
pub use chat::Chat;
pub use directive::*;
pub use file::File;
pub use image::Image;
pub use inline_suggest_button::InlineSuggestButton;
pub use message::Message;
pub use sender::Sender;
pub use suggest_buttons::SuggestButtons;
pub use update::Update;
pub use user::User;
pub use requests::{GetUpdatesRequest, SendTextRequest};
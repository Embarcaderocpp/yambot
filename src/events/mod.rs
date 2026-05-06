//! Система обработки обновлений (handlers + dispatcher)

pub mod dispatcher;
pub mod handler;

pub use dispatcher::Dispatcher;
pub use handler::Handler;
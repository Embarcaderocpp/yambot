use thiserror::Error;

/// Основной тип ошибки библиотеки `yambot`
#[derive(Error, Debug)]
pub enum YambotError {
    /// Ошибка HTTP-запроса (reqwest)
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// Ошибка сериализации/десериализации JSON
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Ошибка API Yandex Messenger (ok: false)
    #[error("API error: {description} (code: {error_code:?})")]
    Api {
        description: String,
        error_code: Option<i64>,
    },

    /// Неизвестная ошибка
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Удобный алиас для `Result<T, YambotError>`
pub type Result<T> = std::result::Result<T, YambotError>;
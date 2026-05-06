const DEFAULT_BASE_URL: &str = "https://botapi.messenger.yandex.net/bot/v1/";

/// Конфигурация бота
#[derive(Debug, Clone)]
pub struct Config {
    /// OAuth-токен бота (обязательно)
    pub token: String,

    /// Базовый URL API (по умолчанию `https://botapi.messenger.yandex.net/bot/v1/`)
    pub base_url: String,

    /// Таймаут запросов в секундах (по умолчанию 30)
    pub timeout: u64,
}

impl Config {
    /// Создаёт конфигурацию только с токеном (использует все значения по умолчанию)
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout: 30,
        }
    }

    /// Builder-style конфигурация
    pub fn builder(token: impl Into<String>) -> ConfigBuilder {
        ConfigBuilder::new(token)
    }
}

/// Builder для `Config`
#[derive(Debug)]
pub struct ConfigBuilder {
    token: String,
    base_url: Option<String>,
    timeout: Option<u64>,
}

impl ConfigBuilder {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            base_url: None,
            timeout: None,
        }
    }

    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    pub fn timeout(mut self, seconds: u64) -> Self {
        self.timeout = Some(seconds);
        self
    }

    pub fn build(self) -> Config {
        Config {
            token: self.token,
            base_url: self.base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            timeout: self.timeout.unwrap_or(30),
        }
    }
}
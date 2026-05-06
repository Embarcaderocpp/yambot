use reqwest::{Client as ReqwestClient, header};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::{config::Config, error::YambotError, Result};

/// Низкоуровневый HTTP-клиент (внутренний)
#[derive(Debug, Clone)]
pub(crate) struct Client {
    http: ReqwestClient,
    config: Config,
}

#[derive(serde::Deserialize, Debug)]
struct ApiResponse<T> {
    ok: bool,
    #[serde(flatten)]
    data: serde_json::Value, // всё остальное (updates, message_id и т.д.)
    #[serde(default)]
    description: Option<String>,
}

impl Client {
    /// Создаёт новый клиент из конфигурации
    pub fn new(config: Config) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&format!("OAuth {}", config.token))
                .expect("Invalid token"),
        );

        let http = ReqwestClient::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(config.timeout))
            .build()
            .expect("Failed to create HTTP client");

        Self { http, config }
    }

    /// Основной метод: отправляет JSON-запрос и возвращает десериализованный результат
    /// `method` — путь без базового URL, например: "messages/getUpdates" или "messages/sendText"
    pub async fn post_json<T, B>(&self, method: &str, body: &B) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let url = format!(
            "{}{}",
            self.config.base_url.trim_end_matches('/'),
            if method.starts_with('/') { method } else { &format!("/{}", method) }
        );

        let response = self
            .http
            .post(&url)
            .json(body)
            .send()
            .await
            .map_err(YambotError::Http)?;

        self.handle_response(response).await
    }

    /// Вспомогательный метод обработки ответа API
    async fn handle_response<T: DeserializeOwned>(&self, response: reqwest::Response) -> Result<T> {
        let status = response.status();
        let text = response.text().await.map_err(YambotError::Http)?;

        // Если HTTP-статус не 2xx — сразу ошибка
        if !status.is_success() {
            return Err(YambotError::Unknown(format!(
                "HTTP error {}: {}",
                status, text
            )));
        }

        let api_resp: ApiResponse<serde_json::Value> =
            serde_json::from_str(&text).map_err(YambotError::Json)?;

        if api_resp.ok {
            // Десериализуем данные напрямую (у Yandex нет обёртки "result")
            serde_json::from_value(api_resp.data).map_err(YambotError::Json)
        } else {
            Err(YambotError::Api {
                description: api_resp.description.unwrap_or_else(|| "Unknown API error".to_string()),
                error_code: None,
            })
        }
    }
}

// Удобный From для Bot
impl From<Config> for Client {
    fn from(config: Config) -> Self {
        Self::new(config)
    }
}
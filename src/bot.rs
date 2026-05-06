use crate::{
    client::Client,
    config::Config,
    error::Result,
    types::{requests::*, SendTextRequest, Update},
};

/// Главный фасад библиотеки — `Yambot`
#[derive(Debug, Clone)]
pub struct Bot {
    client: Client,
}

impl Bot {
    /// Создать бота по токену (самый простой способ)
    pub fn new(token: impl Into<String>) -> Self {
        Self::from_config(Config::new(token))
    }

    /// Создать бота из полной конфигурации
    pub fn from_config(config: Config) -> Self {
        let client = Client::new(config);
        Self { client }
    }

    /// Получить обновления (long polling)
    ///
    /// Возвращает список `Update`.
    /// После обработки обновлений сохраняйте `last_update_id` и передавайте его как `offset`.
    pub async fn get_updates(
        &self,
        request: GetUpdatesRequest,
    ) -> Result<Vec<Update>> {
        // Обёртка нужна, потому что API возвращает { "updates": [...] }
        #[derive(serde::Deserialize)]
        struct UpdatesWrapper {
            updates: Vec<Update>,
        }

        let wrapper: UpdatesWrapper = self
            .client
            .post_json("messages/getUpdates", &request)
            .await?;

        Ok(wrapper.updates)
    }

    /// Отправить текстовое сообщение
    ///
    /// Можно указать либо `chat_id` (группа/канал), либо `login` (личное сообщение).
    pub async fn send_text(&self, request: SendTextRequest) -> Result<i64> {
        #[derive(serde::Deserialize)]
        struct SendTextResponse {
            message_id: i64,
        }

        let resp: SendTextResponse = self
            .client
            .post_json("messages/sendText", &request)
            .await?;

        Ok(resp.message_id)
    }

    /// Удобный shortcut без создания структуры
    pub async fn send_message(
        &self,
        chat_id: Option<&str>,
        login: Option<&str>,
        text: impl Into<String>,
    ) -> Result<i64> {
        self.send_text(SendTextRequest {
            chat_id: chat_id.map(|s| s.to_string()),
            login: login.map(|s| s.to_string()),
            text: text.into(),
            ..Default::default() // для остальных полей
        })
            .await
    }
}
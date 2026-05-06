use crate::{bot::Bot, error::Result, types::requests::GetUpdatesRequest, types::Update};
use super::Handler;
use std::sync::Arc;

/// Диспетчер — главный компонент для обработки обновлений
#[derive(Default)]
pub struct Dispatcher {
    bot: Option<Bot>,
    handlers: Vec<Arc<dyn Handler>>, // ← ИЗМЕНЕНИЕ: Arc для 'static + Clone в spawn
}

impl Dispatcher {
    /// Создать новый диспетчер с ботом
    pub fn new(bot: Bot) -> Self {
        Self {
            bot: Some(bot),
            handlers: vec![],
        }
    }

    /// Добавить обработчик (builder-style)
    pub fn register<H: Handler>(mut self, handler: H) -> Self {
        self.handlers.push(Arc::new(handler)); // ← ИЗМЕНЕНИЕ: Arc::new
        self
    }

    /// Удобный метод для обработчика сообщений (пример)
    pub fn on_message<F, Fut>(self, f: F) -> Self
    where
        F: Fn(Bot, Update) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + 'static, // ← добавлено для consistency
    {
        self.register(super::handler::handler(f))
    }

    /// Запустить long polling (бесконечный цикл)
    pub async fn run(self) -> Result<()> {
        let Dispatcher { bot, handlers } = self;
        let bot = bot.expect("Dispatcher должен быть создан через Dispatcher::new(bot)");
        let mut offset: Option<i64> = None;   // ← ИСПРАВЛЕНО: i64

        println!("🚀 Yambot Dispatcher запущен (long polling)...");

        loop {
            let request = GetUpdatesRequest {
                offset,                       // ← теперь просто offset (i64)
                limit: Some(100),
            };

            match bot.get_updates(request).await {
                Ok(updates) => {
                    if let Some(last_update) = updates.last() {
                        offset = Some(last_update.update_id);  // ← i64, без clone()
                    }

                    for update in updates {
                        for handler_arc in &handlers {
                            let handler = Arc::clone(handler_arc);
                            let bot_clone = bot.clone();
                            let update_clone = update.clone();

                            tokio::spawn(async move {
                                if let Err(e) = handler.handle(&bot_clone, update_clone).await {
                                    eprintln!("Handler error: {:?}", e);
                                }
                            });
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Ошибка получения обновлений: {:?}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
        }
    }
}
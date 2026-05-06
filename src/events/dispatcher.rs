use crate::{bot::Bot, error::Result, types::requests::GetUpdatesRequest, types::Update};

use super::Handler;

/// Диспетчер — главный компонент для обработки обновлений
#[derive(Default)]
pub struct Dispatcher {
    bot: Option<Bot>,
    handlers: Vec<Box<dyn Handler>>,
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
        self.handlers.push(Box::new(handler));
        self
    }

    /// Удобный метод для обработчика сообщений (пример)
    pub fn on_message<F, Fut>(self, f: F) -> Self
    where
        F: Fn(Bot, Update) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send,
    {
        self.register(super::handler::handler(f))
    }

    /// Запустить long polling (бесконечный цикл)
    pub async fn run(self) -> Result<()> {
        let bot = self.bot.expect("Dispatcher должен быть создан через Dispatcher::new(bot)");
        let mut offset: Option<String> = None;

        println!("🚀 Yambot Dispatcher запущен (long polling)...");

        loop {
            let request = GetUpdatesRequest {
                offset: offset.clone(),
                limit: Some(100),
            };

            match bot.get_updates(request).await {
                Ok(updates) => {
                    if let Some(last_update) = updates.last() {
                        offset = Some(last_update.update_id.clone());
                    }

                    // Запускаем все handlers для каждого обновления
                    for update in updates {
                        for handler in &self.handlers {
                            let bot_clone = bot.clone();
                            let update_clone = update.clone();
                            let handler = handler.as_ref(); // &dyn Handler

                            // Запускаем обработку в отдельной задаче (не блокируем polling)
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
                    // Небольшая пауза перед повторной попыткой
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }

            // Пауза между запросами (можно регулировать)
            tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
        }
    }
}
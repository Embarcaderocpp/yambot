use async_trait::async_trait;

use crate::{bot::Bot, error::Result, types::Update};

/// Основной трейт обработчика обновлений.
/// Все обработчики должны реализовывать этот трейт.
#[async_trait]
pub trait Handler: Send + Sync + 'static {
    /// Вызывается для каждого полученного `Update`
    async fn handle(&self, bot: &Bot, update: Update) -> Result<()>;
}

/// Удобный конструктор для closure-based обработчиков
pub fn handler<F, Fut>(f: F) -> impl Handler
where
    F: Fn(Bot, Update) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Result<()>> + Send,
{
    struct FnHandler<F, Fut>(F)
    where
        F: Fn(Bot, Update) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send;

    #[async_trait]
    impl<F, Fut> Handler for FnHandler<F, Fut>
    where
        F: Fn(Bot, Update) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send,
    {
        async fn handle(&self, bot: &Bot, update: Update) -> Result<()> {
            (self.0)(bot.clone(), update).await
        }
    }

    FnHandler(f)
}
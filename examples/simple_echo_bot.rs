use yambot::{new, Dispatcher};

#[tokio::main]
async fn main() -> yambot::Result<()> {
    let bot = new("y0__wgBEI6Ti-oIGJaREyCbu7KrF_jNFeHuiMq9n0Q8-Cmn_VJTOLvX");

    Dispatcher::new(bot)
        .on_message(|bot, update| async move {
            if let Some(text) = &update.text {
                // ← ИСПРАВЛЕНИЕ: выбираем ровно один параметр
                let chat_id = if update.chat.r#type != "private" {
                    update.chat.id.as_deref()
                } else {
                    None
                };
                let login = update.sender.login.as_deref();

                bot.send_message(chat_id, login, format!("Эхо: {}", text)).await?;
                println!("Ответили на сообщение от {}", update.sender.login.unwrap_or_default());
            }
            Ok(())
        })
        .run()
        .await
}
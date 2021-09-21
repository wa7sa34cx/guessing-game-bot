use dotenv::dotenv;
use teloxide::{prelude::*, utils::command::BotCommand};

#[tokio::main]
async fn main() {
    dotenv().ok();

    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message| async move {
        message.answer_dice().await?;
        respond(())
    })
    .await;
}

use anyhow::Result;
use dotenv::dotenv;
use teloxide::{prelude::*, utils::command::BotCommands};
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    dotenv().ok();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    connect_postgres().await?;

    Command::repl(bot, answer).await;
    Ok(())
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "help me!!")]
    Help,
    #[command(description = "who are you?")]
    Hi,
    #[command(description = "Push some info")]
    Rss,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Hi => bot.send_message(msg.chat.id, "你好! 我是张无忌").await?,
        Command::Rss => bot.send_message(msg.chat.id, "你好! 我是张无忌").await?,
    };

    Ok(())
}

async fn connect_postgres() -> Result<()> {
    // Connect to the database.
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=password connect_timeout=10",
        NoTls,
    )
    .await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client.query("SELECT $1::TEXT", &[&"hello world"]).await?;

    // And then check that we got back the same string we sent over.
    let value: &str = rows[0].get(0);
    println!("{}", value);
    Ok(())
}

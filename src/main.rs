use std::error::Error;

use anyhow::Result;
use atom_syndication::Feed;
use dotenv::dotenv;
use teloxide::{prelude::*, utils::command::BotCommands};
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    dotenv().ok();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    // connect_postgres().await?;

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
        Command::Rss => {
            if let Err(e) = update_youtube_channel_rss(&bot, &msg, "UCrqM0Ym_NbK1fqeQG2VIohg").await
            {
                log::error!("{:?}", e);
            }
            bot.send_message(msg.chat.id, "你好! 我是张无忌").await?
        }
    };

    Ok(())
}

async fn update_youtube_channel_rss(
    bot: &Bot,
    msg: &Message,
    channel_id: &str,
) -> Result<(), Box<dyn Error>> {
    let url = format!("https://www.youtube.com/feeds/videos.xml?channel_id={channel_id}");
    let content = reqwest::get(&url).await?.text().await?;
    log::info!("Fetched RSS feed from: {}", url);

    let feed = content.parse::<Feed>()?;
    log::info!("Parsed Atom feed successfully");

    let title = feed.title.value;
    let link = &feed.links[0].href;
    log::info!("Channel Title: {}", title);
    log::info!("Channel Link: {}", link);

    if let Some(entry) = feed.entries.first() {
        log::info!("\nVideo Title: {}", entry.title.value);
        log::info!("Video Link: {}", entry.links[0].href);
        log::info!("Published Date: {}", entry.published.unwrap_or_default());
        let message = format!("{}\n{}", title, entry.links[0].href);
        bot.send_message(msg.chat.id, message).await?;
    }

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

use rand::seq::SliceRandom;
use crate::commands::{Context, Error};

#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn whycrash(ctx: Context<'_>,) -> Result<(), Error> {
    let pick = &ctx.data().responses.choose(&mut rand::thread_rng()).cloned().unwrap_or_else(|| { "Idk lmao".to_string() });
    ctx.reply(format!("> {}", pick)).await?;
    Ok(())
}
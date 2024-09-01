use rand::{thread_rng, Rng};
use crate::commands::{Context, Error};

#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn whenupdate(
    ctx: Context<'_>,
    #[description = "Name of the mod"] name: String,
    #[description = "Minecraft version"] version: Option<String>
) -> Result<(), Error> {
    let days =  thread_rng().gen_range(7..=365);
    let message = if version.is_none() {
        format!("{} will be updated in {} days", name, days)
    } else {
        format!("{} will be updated to {} in {} days", name, version.unwrap(), days)
    };
    let _ = ctx.reply(message).await;
    Ok(())
}
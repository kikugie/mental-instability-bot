use crate::commands::{Context, Error};
use rand::Rng;

mod whycrash {
    use lazy_static::lazy_static;
    use rand::rngs::StdRng;
    use std::sync::Mutex;
    use rand::SeedableRng;

    lazy_static! {
        pub static ref RNG: Mutex<StdRng> = Mutex::new(StdRng::from_entropy());
    }
}

#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
pub async fn whycrash(ctx: Context<'_>,) -> Result<(), Error> {
    let message: String = {
        let mut rand = whycrash::RNG.lock().unwrap();
        let responses = &ctx.data().responses;
        if responses.is_empty() {
            "IDK lmao".to_string()
        } else {
            let index = rand.gen_range(0..responses.len());
            responses[index].clone()
        }
    };
    ctx.reply(format!("> {}", message)).await?;
    Ok(())
}
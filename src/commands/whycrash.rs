use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use futures::{Stream, StreamExt};
use futures::future::ready;
use futures::stream::iter;
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
pub async fn whycrash(
    ctx: Context<'_>,
    #[description = "Selected response"]
    #[autocomplete = "response"]
    response: Option<String>
) -> Result<(), Error> {
    let message: String = if response.is_some() {
        response.unwrap()
    } else {
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

async fn response<'a>(
    ctx: Context<'a>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    let matcher = SkimMatcherV2::default();
    let responses = &ctx.data().responses;
    iter(responses)
        .filter(move |name| ready(matcher.fuzzy_match(name, partial).unwrap_or(0) > 0))
        .map(|name| name.to_string())
}

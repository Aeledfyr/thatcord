use thatcord::{impl_event, events, Discord, Result};

struct ExampleReadyHandler {
    welcome_text: String
}

impl_event!(ExampleReadyHandler, ReadyEvent(this, client: (events::Client<'_>)) {
    let user = client.get_current_user();

    println!("{}", this.welcome_text);
    println!("user: {}#{}", user.username, user.discriminator);

    Ok(())
});

struct ExampleGuildCreateHandler {}
impl_event!(ExampleGuildCreateHandler, GuildCreateEvent(_this, _client: (events::Client<'_>), guild: (thatcord::Guild)) {
    println!(" - {}", guild.name.unwrap());
    Ok(())
});

#[tokio::main] // https://crates.io/crates/tokio
async fn main() -> Result<()> {
    env_logger::init();

    let token = std::env::var("TOKEN").expect("please set the environment variable TOKEN");
    let client = Discord::new();

    {
        let mut client_events = client.borrow_mut();
        client_events.register_event(events::READY, ExampleReadyHandler {
            welcome_text: "Welcome! Listing guilds!".to_owned()
        });

        client_events.register_event(events::GUILD_CREATE, ExampleGuildCreateHandler {});
    }

    Discord::connect(client, &token).await
}

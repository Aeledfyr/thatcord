use thatcord::{events, impl_event, Discord, Result};

struct ExampleHandler {
    welcome_text: String,
}

// https://github.com/dtolnay/async-trait/issues/46
// `this` can be anything but `self`  --v
impl_event!(ExampleHandler, ReadyEvent(this, client: (events::Client<'_>)) {
    let client = client;
    let user = client.get_current_user();

    println!("{}", this.welcome_text);
    println!("user: {}#{}", user.username, user.discriminator);

    Ok(())
});

#[tokio::main] // https://crates.io/crates/tokio
async fn main() -> Result<()> {
    env_logger::init();

    let token = std::env::var("TOKEN").expect("please set the environment variable TOKEN");
    let client = Discord::new();

    {
        let mut client_events = client.borrow_mut();
        client_events.register_event(
            events::READY,
            ExampleHandler {
                welcome_text: "Welcome! We are ready!".to_owned(),
            },
        );
    }

    Discord::connect(client, &token).await
}

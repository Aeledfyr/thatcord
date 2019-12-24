use crate::errors::*;
use crate::events::*;
use crate::gateway::{EventHandler as GatewayEventHandler, Gateway};
use crate::json;
use async_trait::async_trait;
use snafu::ResultExt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub(crate) const API_PATH: &str = "https://discordapp.com/api/v6";

/// The User-Agent of the discord bot that is used when interacting
/// with the discord apis.
///
/// https://discordapp.com/developers/docs/reference#user-agent
pub(crate) const USER_AGENT: &str = concat!(
    "DiscordBot (https://github.com/Admicos/thatcord, ",
    env!("CARGO_PKG_VERSION"),
    ")"
);

struct DefaultEventHandler {
    events: HashMap<String, Box<dyn EventHandler>>,
    client: Option<Weak<RefCell<Discord>>>,
}

impl DefaultEventHandler {
    pub(crate) fn new() -> Self {
        DefaultEventHandler {
            client: None,
            events: HashMap::new(),
        }
    }

    pub(crate) fn set_client(&mut self, client: Weak<RefCell<Discord>>) {
        self.client = Some(client);
    }

    pub(crate) fn register_event(&mut self, event: String, handler: Box<dyn EventHandler>) {
        self.events.insert(event, handler);
    }

    async fn pre_ready(&mut self, data: &serde_json::Value) -> InternalResult<()> {
        let c = self
            .client
            .as_mut()
            .unwrap()
            .upgrade()
            .expect("Cannot upgrade weak client ref pre ready");

        let c = c.borrow();

        (*c.user.borrow_mut()) =
            Some(serde_json::from_value(data["user"].clone()).context(JsonConversionError)?);

        (*c.guilds.borrow_mut()) =
            serde_json::from_value(data["guilds"].clone()).context(JsonConversionError)?;

        Ok(())
    }

    async fn pre_guild_create(&mut self, data: &serde_json::Value) -> InternalResult<()> {
        let c = self
            .client
            .as_mut()
            .unwrap()
            .upgrade()
            .expect("Cannot upgrade weak client ref pre guild create");

        let c = c.borrow();
        let new_guild: json::Guild =
            serde_json::from_value(data.clone()).context(JsonConversionError)?;

        // There might be a better method, not sure though.
        (*c.guilds.borrow_mut()).retain(|g| g.id != new_guild.id);
        (*c.guilds.borrow_mut()).push(new_guild);
        Ok(())
    }
}

#[async_trait(?Send)]
impl GatewayEventHandler for DefaultEventHandler {
    async fn handle(&mut self, event: String, data: serde_json::Value) -> InternalResult<()> {
        // Special case events
        match event.as_str() {
            "READY" => self.pre_ready(&data).await?,
            "GUILD_CREATE" => self.pre_guild_create(&data).await?,
            _ => {}
        }

        if let Some(handler) = self.events.get_mut(&event) {
            let c = self
                .client
                .as_mut()
                .unwrap()
                .upgrade()
                .expect("Cannot upgrade weak client ref on event handle");

            match handler.handle(c.borrow(), event, data).await {
                Ok(_) => {}
                Err(e) => log::error!("Event returned error: {}", e),
            }
        }

        Ok(())
    }
}

/// This is the "main" struct you'll need to connect to Discord.
pub struct Discord {
    events: Option<DefaultEventHandler>,

    user: RefCell<Option<json::User>>,
    guilds: RefCell<Vec<json::Guild>>,
}

impl Discord {
    pub fn new() -> Rc<RefCell<Self>> {
        let slf = Rc::new(RefCell::new(Self {
            events: Some(DefaultEventHandler::new()),

            user: RefCell::new(None),
            guilds: RefCell::new(Vec::default()),
        }));

        // I tried something. It didn't work. I forgot how I did it before.
        let cslf = slf.clone();
        let mut mslf = cslf.borrow_mut();
        mslf.events
            .as_mut()
            .expect("Event that I _just_ set is not set? What?")
            .set_client(Rc::downgrade(&slf));

        slf
    }

    /// Register an event handler struct. See `thatcord::events` for a list of event names.
    ///
    /// **Warning:** You **can not** use this after calling `connect`.
    /// **Another Warning:** You **can not** use the same handler for multiple events.
    pub fn register_event<T: EventHandler + 'static>(&mut self, event: &str, handler: T) {
        self.events.as_mut().expect("Events not initialized, are you sure you didn't call register_event() after calling connect()?").register_event(event.to_owned(), Box::new(handler))
    }

    /// Get information about the current user/bot.
    /// **Warning:** You **can not** use this until you receive a `thatcord::events::ReadyEvent`.
    pub fn get_user(&self) -> json::User {
        self.user
            .borrow()
            .as_ref()
            .expect("Cannot get user before connection ready")
            .clone()
    }

    /// This method will connect to Discord, and start everything.
    /// **Warning:** This method **will not return** until the connection closes.
    pub async fn connect(this: Rc<RefCell<Self>>, token: &str) -> Result<()> {
        let mut iself = this.borrow_mut();
        let gateway =
            json::gateway::get_bot_gateway(token)
                .await
                .map_err(|e| Errors::ApiHttpError {
                    source: DebugWrapper(e),
                })?;

        // Since this method is the last one that should be called in this struct,
        // we can safely move the event handler off of us, and into the gateway.
        //
        // https://stackoverflow.com/questions/31307680/how-to-move-one-field-out-of-a-struct-that-implements-drop-trait
        let take_ownership_of_events = std::mem::replace(&mut iself.events, None);
        let mut gateway = Gateway::new(
            &gateway.url,
            token,
            take_ownership_of_events.expect("Events not initialized? It should be..."),
        )
        .await?;

        std::mem::drop(iself);
        Ok(gateway.handle().await?)
    }
}

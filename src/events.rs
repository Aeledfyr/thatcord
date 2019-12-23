use crate::{Discord, Result};
use std::cell::Ref;

/// Convenience type alias for the client struct
pub type Client<'a> = Ref<'a, Discord>;

// Q: Why is every macro parameter a tt?
// A: https://github.com/dtolnay/async-trait/issues/46

#[macro_export]
macro_rules! impl_event {
    ($typ:tt, $evt:ident($_self:tt, $($param_name:tt: $param_type:tt),*) $inner:tt) => {
        paste::item! {
            $crate::[<_thatcord_internal_impl_ $evt>]!($typ);
            $crate::[<_thatcord_internal_custom_handler_ $evt>]!($typ, $inner, $_self, $($param_name: $param_type),*);
        }
    }
}

macro_rules! define_events {
    ($d:tt $($(#[$meta:meta])* event $name:tt, $gateway_name:tt, ($($param_name:tt: $param_type:tt),*) $map:tt < ($($use_param_name:tt: $use_param_type:tt),*))*) => {
        $(
            $(#[$meta])*
            #[async_trait::async_trait(?Send)]
            #[allow(unused_parens)]
            pub trait $name: EventHandler {
                async fn handle_real(&mut self $(, $param_name: $param_type)*) -> Result<()>;
            }

            pub const $gateway_name: &str = stringify!($gateway_name);

            paste::item! {
                #[macro_export]
                macro_rules! [<_thatcord_internal_impl_ $name>] {
                    ($typ:tt) => {
                        #[async_trait::async_trait(?Send)]
                        impl $crate::events::EventHandler for $typ {
                            async fn handle(&mut self, $($use_param_name: $use_param_type),*) -> $crate::Result<()> {
                                use $crate::events::$name;
                                $map
                            }
                        }
                    }
                }

                #[macro_export]
                macro_rules! [<_thatcord_internal_custom_handler_ $name>] {
                    ($typ:tt, $inner:tt, $_self:tt, $d($inner_param_name:tt: $inner_param_type:tt),*) => {
                        #[async_trait::async_trait(?Send)]
                        impl $crate::events::$name for $typ {
                            #[allow(unused_parens)]
                            async fn handle_real(&mut self, $d($inner_param_name: $inner_param_type),*) -> $crate::Result<()> {
                                let $_self = self;
                                $inner
                            }
                        }
                    }
                }
            }
        )*
    }
}

/// Base event handler trait. You probably don't need it.
#[async_trait::async_trait(?Send)]
pub trait EventHandler {
    async fn handle(
        &mut self,
        client: Client<'_>,
        event: String,
        data: serde_json::Value,
    ) -> Result<()>;
}

define_events!($
    /// This event will fire when you've been successfully connected to Discord.
    event ReadyEvent, READY, (
        client: (crate::events::Client<'_>)
    ) {
        self.handle_real(client).await
    } < (client: (crate::events::Client<'_>), event: String, data: (serde_json::Value))

    /// This event will fire when you receive a guild creation event.
    /// This will happen due to:
    ///  * Lazy loaded guilds on startup
    ///  * Added to a new guild
    ///  * Guild becomes available after an outage
    event GuildCreateEvent, GUILD_CREATE, (
        client: (crate::events::Client<'_>),
        guild: (crate::Guild)
    ) {
        self.handle_real(client, serde_json::from_value(data).unwrap()).await
    } < (client: (crate::events::Client<'_>), event: String, data: (serde_json::Value))
);

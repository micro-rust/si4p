//! Global `Message` dispatcher.



use crate::gui::msg::Message;

use iced_futures::futures::stream::{unfold, BoxStream};

use iced_native::subscription::{
    Recipe, Subscription,
};

use tokio::{
    sync::mpsc,
};

use tracing::{ warn };



pub fn subscription() -> Subscription<Message> {
    Subscription::from_recipe(MessageCenter {})
}




pub struct MessageCenter {}

impl<H: std::hash::Hasher, I> Recipe<H, I> for MessageCenter {
    type Output = Message;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
    }

    fn stream(self: Box<Self>, _: BoxStream<'static, I>) -> BoxStream<'static, Self::Output> {

        Box::pin( unfold(
            State::Uninitialized,
            move |state| async move {
                match state {
                    State::Uninitialized => {
                        warn!("Initializing the fucking dispatcher");

                        // Create the channel.
                        let (tx, rx) = mpsc::channel(1024);

                        Some((Message::DispatcherCreated(tx), State::WaitingDatabases(rx)))
                    },

                    State::WaitingDatabases(rx) => {
                        warn!("Initializing the fucking dispatcher databases");
                        Some((Message::DatabasesAvailable, State::Running(rx)))
                    },

                    State::Running(mut rx) => {
                        warn!("Running the fucking dispatcher");
                        match rx.recv().await {
                            Some(msg) => {
                                Some((msg, State::Running(rx)))
                            },
                            _ => {
                                let _: () = iced::futures::future::pending().await;
                                None
                            },
                        }
                    },
                }
            },
        ))
    }
}




enum State {
    /// Dispatcher has not initialized yet.
    Uninitialized,

    /// Dispatcher is waiting for the databases.
    WaitingDatabases(mpsc::Receiver<Message>),

    /// Dispatcher is running.
    Running(mpsc::Receiver<Message>),
}

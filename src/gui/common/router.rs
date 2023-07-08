//! A `Router` receives data from a `tokio` channel and maps it to `iced` messages.



use std::sync::Arc;

use tokio::sync::{
    mpsc::Receiver,
    
    Mutex,
};



/// Thread-safe container of a router.
/// Should be low overhead as there should be no contention on the mutex.
type Container<E, M> = Arc<Mutex<Router<E, M>>>;



pub struct Router<E, M> {
    /// Router mapping function.
    map: Arc<fn(E) -> M>,

    /// Message to return if the channel closes.
    close: M,

    /// Reception channel.
    rx: Receiver<E>,
}

impl<E, M: Clone> Router<E, M> {
    /// Creates a new `Router`.
    pub fn create(map: Arc<fn(E) -> M>, close: M, rx: Receiver<E>) -> Self {
        Self { map, close: close, rx, }
    }

    /// Listen to the channel and produce messages.
    pub async fn listen(container: Container<E, M>) -> (M, Container<E, M>) {
        // Get the next message.
        let message = {
            // Acquire the lock on the router.
            // This should be instantaneous, as there should be no contention.
            let mut router = container.lock().await;

            // Wait for a message or event.
            match router.rx.recv().await {
                // There is a received message.
                Some(msg) => {
                    //println!("New console message");
                    (router.map)(msg)
                },

                // Send the router closed signal.
                _ => router.close.clone(),
            }
        };

        (message, container)
    }
}

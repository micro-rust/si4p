//! Commands related to the SVD files.



use svd_parser::svd::MaybeArray;

use crate::{
    gui::console::{
        Entry, Source,
    },

    library::Library,

    target::Peripheral,
};

use std::{
    path::PathBuf,
    sync::Arc,
};

use super::Message;

use tokio::sync::mpsc::Sender;



/// Async function to read a SVD file.
pub async fn loadSVD(name: String, library: Arc<Library>) -> Message {
    use tokio::{
        fs::File,
        io::AsyncReadExt,
    };

    // Get the path to the SVD file.
    let path = match library.svd.read().await.file(&name) {
        None => return Entry::error(Source::Host, format!("Failed to load SVD file : No file for target {}", name)).into(),
        Some(path) => (*path).clone(),
    };

    // Create the file.
    let file = match File::open(&path.clone()).await {
        Err(_) => return Message::None,
        Ok(f) => f,
    };

    // Create the buffer.
    let mut data = Vec::new();

    // Create the reader.
    let mut reader = tokio::io::BufReader::new(file);

    // Read the file.
    match reader.read_to_end(&mut data).await {
        Err(e) => return Entry::error(Source::Host, format!("Failed to load SVD file {} : {}", path.display(), e)).into(),
        Ok(_) => (),
    }

    // Convert the contents to a string.
    let string = match String::from_utf8( data.clone() ) {
        Err(e) => return Entry::error(Source::Host, format!("Failed to load SVD file {} as string : {}", path.display(), e)).into(),
        Ok(string) => string,
    };

    // Parse the SVD data.
    let peripherals = parse( string ).await;

    Message::NewSVD( peripherals, path )
}



/// Parses the peripherals from an SVD file.
async fn parse(data: String) -> Vec<Arc<Peripheral>> {
    // Parse the SVD.
    let svd = match svd_parser::parse( &data ) {
        Err(_) => return Vec::new(),
        Ok(svd) => svd,
    };

    // Create the list of peripherals.
    let mut peripherals = Vec::new();

    for maybe in &svd.peripherals {
        // Get the peripheral.
        let peripheral = match maybe {
            MaybeArray::Array(p, _) => p,
            MaybeArray::Single(p) => p,
        };

        // Create the peripheral information.
        let information = Peripheral::create(&svd, peripheral);

        // Add the peripheral to the list.
        peripherals.push( Arc::new( information ) );
    }

    peripherals
}



/// Watches the svd folder and emits rebuild commands.
pub fn watch(path: PathBuf, channel: Sender<Message>) {
    use notify::{
        Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,

        event::{
            CreateKind, ModifyKind, RemoveKind,
        },
    };

    use std::time::Duration;



    // Internal struct to handle events.
    struct EventHandler {
        channel: Sender<Message>,
    }

    impl notify::EventHandler for EventHandler {
        fn handle_event(&mut self, event: notify::Result<Event>) {
            // Check if a message is emitted.
            let send = match event {
                Ok(change) => match change.kind {
                    // Only recreate on file creation or symlink creation.
                    EventKind::Create(kind) => match kind {
                        CreateKind::File | CreateKind::Other => true,
                        _ => false,
                    },

                    // Only recreate on data or name changes.
                    EventKind::Modify(kind) => match kind {
                        ModifyKind::Data(_) | ModifyKind::Name(_) => true,
                        _ => false,
                    },

                    // Recreate on all remove events.
                    EventKind::Remove(kind) => match kind {
                        RemoveKind::Any => true,
                        _ => false,
                    },

                    _ => false,
                },

                Err(e) => {
                    // Log the error.
                    let _ = self.channel.try_send( Entry::error(Source::Host, format!("Error in SVD watcher : {}", e)).into() );

                    return;
                },
            };

            if send {
                let _ = self.channel.try_send( Message::LibraryRebuildSVD );
            }
        }
    }

    // Create the watcher configuration.
    let config = Config::default()
        .with_poll_interval( Duration::from_secs(10) )
        .with_compare_contents(false);

    // Create the event handler.
    let event_handler = EventHandler { channel: channel.clone() };

    // Create the watcher.
    let mut watcher = match RecommendedWatcher::new(event_handler, config) {
        Err(e) => {
            // Log the error.
            let _ = channel.try_send( Entry::error( Source::Host, format!("Failed to create SVD filesystem watcher : {}", e) ).into() );

            return;
        },

        Ok(w) => w,
    };

    // Begin watching.
    match watcher.watch(&path, RecursiveMode::Recursive) {
        Err(e) => {
            // Log the error.
            let _ = channel.try_send( Entry::error( Source::Host, format!("Failed to create SVD filesystem watcher : {}", e) ).into() );

            return;
        },

        Ok(_) => (),
    }

    loop {
        use std::sync::atomic::Ordering;

        const TIMEOUT : Duration = Duration::from_secs(5);

        // Wait for a while.
        std::thread::sleep( TIMEOUT );

        // Check if quitting.
        if crate::QUIT.load(Ordering::Relaxed) { break; }
    }
}

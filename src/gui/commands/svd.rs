//! Commands related to the SVD files.



use svd_parser::svd::MaybeArray;

use crate::{
    gui::console::{
        Entry, Source,
    },
    library::Library,

    target::Peripheral,
};

use std::sync::Arc;

use super::Message;



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

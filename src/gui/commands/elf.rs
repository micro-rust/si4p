//! Commands related to the ELF files.



use std::path::PathBuf;

use super::Message;



/// Async function to select an ELF file.
pub async fn selectELF(maybe: Option<PathBuf>) -> Message {
    // Extract the path.
    let path = match maybe {
        Some(path) => path,
        _ => PathBuf::from("/"),
    };

    // Get the file.
    let maybe: Option<rfd::FileHandle> = rfd::AsyncFileDialog::new()
        .set_directory( path )
        .pick_file()
        .await;

    // Check if anything was chosen.
    let path = match maybe.as_ref() {
        None => return Message::None,
        Some(f) => f.path().clone(),
    };

    Message::LoadELF( PathBuf::from( path ) )
}



/// Async function to read an ELF file.
pub async fn loadELF(path: PathBuf) -> Message {
    use tokio::{
        fs::File,
        io::AsyncReadExt,
    };

    // Create the file.
    let file = match File::open(path.clone()).await {
        Err(_) => return Message::None,
        Ok(f) => f,
    };

    // Create the buffer.
    let mut data = Vec::new();

    // Create the reader.
    let mut reader = tokio::io::BufReader::new(file);

    // Read the file.
    match reader.read_to_end(&mut data).await {
        Err(_) => return Message::None,
        Ok(_) => (),
    }

    Message::NewELF( std::sync::Arc::from(data), path )
}

//! The library contains information on all external resources of the application
//! such as images, fonts, svd files, etc...




pub struct Libray {

}

impl Libray {
    /// Static initializer.
    pub const fn empty() -> Self {
        Self {}
    }

    /// Parse the library contents from the given folder.
    /// If the folder is empty, create the file structure in it.
    pub async fn create() -> Self {
        use tokio::fs::{
            read_dir, DirBuilder,
        };

        // Get the data folder.
        let data = dirs::data_dir().expect("Unable to access the data directory");
        let path = data.join("si4p");

        if !path.exists() {
            // Create the Si4+ dir.
            DirBuilder::new().create( path.clone() ).await.expect("Failed to create Si4+ directory");

            return Self::build( path ).await;
        }

        // Create a one time read directory to check if the dir is empty.
        let once = read_dir( path.clone() ).await.expect("Failed to read data directory entries");

        // Read the first entry to see if it is empty.
        let first = once.next_entry().await.expect("Failed to read entries in data directory");

        match first {
            Some(_) => Self::parse(path).await,
            _ => Self::build(path).await,
        }
    }

    /// Creates the library structure in an empty Si4+ folder.
    async fn build(path: std::path::PathBuf) -> Self {
        use tokio::fs::DirBuilder;

        // Create the font dir.
        DirBuilder::new().create( path.join("font") ).await.expect( "Failed to create the font directory" );

        // Create the img dir.
        DirBuilder::new().create( path.join("img") ).await.expect( "Failed to create the image directory" );

        // Create the svd dir.
        DirBuilder::new().create( path.join("svd") ).await.expect( "Failed to create the SVD directory" );

        Self {}
    }
}

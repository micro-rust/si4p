//! The library contains information on all external resources of the application
//! such as images, fonts, svd files, etc...



mod svd;



use std::path::PathBuf;

use tokio::sync::RwLock;



pub struct Library {
    /// SVD library with all the targets and SVD files.
    pub svd: RwLock<svd::SVDLibrary>,

    /// Path to the Si4+ library.
    path: PathBuf,
}

impl Library {
    /// Rebuilds the libraries.
    pub async fn rebuild(&self) {
        // Rebuild the SVD library.
        self.svd.write().await.rebuild().await;
    }

    /// Parse the library contents from the given folder.
    /// If the folder is empty, create the file structure in it.
    pub fn create() -> Self {
        // Get the data folder.
        let mut path = dirs::data_dir().expect("Unable to access the data directory");

        // Create the Si4+ dir.
        Self::createdir( path.join("si4p") );

        // Extend the path.
        path = path.join("si4p");

        // Create the font dir.
        Self::createdir( path.join("font") );

        // Create the img dir.
        Self::createdir( path.join("img") );

        // Create the svd dir.
        Self::createdir( path.join("svd") );

        // Create the theme dir.
        Self::createdir( path.join("theme") );

        Self {
            svd: RwLock::new( svd::SVDLibrary::new( path.clone() ) ),
            path: path.clone(),
        }
    }

    /// Creates a dir if it does not exist.
    fn createdir(path: PathBuf) {
        use std::fs::DirBuilder;

        // If the path exists, do not create it.
        if path.exists() { return; }

        // Create the new directory.
        DirBuilder::new().create( path ).expect("Failed to create a library directory");
    }
}

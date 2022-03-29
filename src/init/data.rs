//! Initialization of data folders and database preparations.



use crate::log::TimeReport;

use std::{
    path::PathBuf,
};

use tokio::{
    fs::File,
    io::AsyncReadExt,
};

use tracing::{
    error, info, warn,
};



/// Initializes the main datafolder
pub async fn init() {
    // Create a time report.
    let mut report = TimeReport::new(String::from("init-data"));
    let start = report.start();

    let appfolder = {
        #[cfg(unix)] {
            match std::env::var("HOME") {
                Ok(home) => {
                    // Build the path to the base folder.
                    let appfolder = PathBuf::from(home.clone()).join(".si4p");

                    // If the folder does not exist, use the embedded data.
                    // This means that the application has not been bootstrapped yet
                    if !appfolder.exists() {
                        // Get the path of the executable.
                        let exe = match std::env::current_exe() {
                            Err(e) => {
                                error!(origin="app", "Could not get path to executable: {}", e);
                                return;
                            },
                            Ok(f) => f,
                        };

                        // Navigate up to the resources folder.
                        let resources = match exe.parent() {
                            None => {
                                error!(origin="app", "Could not find navigate to release folder");
                                return;
                            },

                            Some(release) => match release.parent() {
                                None => {
                                    error!(origin="app", "Could not find navigate to target folder");
                                    return;
                                },

                                Some(target) => match target.parent() {
                                    None => {
                                        error!(origin="app", "Could not find navigate to base folder");
                                        return;
                                    },

                                    Some(base) => base.join("res"),
                                },
                            },
                        };

                        // Check the existence of the resources folder.
                        if !resources.exists() {
                            error!(origin="app", "Resources folder does not exist");
                            return;
                        }

                        // Set the path to the resource folder for the databases.
                        setfolder( appfolder.clone() );
                    }

                    appfolder
                },

                Err(e) => {
                    error!(origin="app", "Could not get environment variable $HOME: {}", e);
                    return;
                },
            }
        }

        #[cfg(not(unix))] {
            panic!("Unimplemented for non unix OS");
        }
    };

    // Check that the version matches.
    match checkversion(appfolder.clone()).await {
        Some(isgood) => if isgood {
            // Set the path to the resource folder for the databases.
            setfolder( appfolder.clone() );
        } else {
            warn!(origin="app", "Base folder does not have the correct version");
        },
        None => {
            error!(origin="app", "An error ocurred while checking resources version");
        },
    }

    report.end(start);
    info!(origin="app", "Base folder initialization time:\n{}", report);
}



/// Sets the basefolder.
fn setfolder(path: PathBuf) {
    match crate::database::BASEFOLDER.lock() {
        Ok(mut basefolder) => {
            info!(origin="app", "Base folder set to {}", path.display());
            *basefolder = Some(path);
        },
        Err(e) => error!(origin="database", db="root", "Base folder mutex is poisoned: {}", e),
    }
}


/// Checks that the resources version matches the one used in the app.
async fn checkversion(appfolder: PathBuf) -> Option<bool> {
    // Create the file path.
    let path = appfolder.join("version.txt");

    // Check if the file exists.
    if !path.exists() { return Some( false ); }

    // Attempt to open the file.
    let content = match File::open(path.clone()).await {
        Ok(mut f) => {
            let mut buffer = Vec::new();

            match f.read_to_end(&mut buffer).await {
                Err(e) => {
                    error!(origin="app", "Could not read version file {}: {}", path.display(), e);
                    return None;
                },
                _ => buffer,
            }
        },

        Err(e) => {
            error!(origin="app", "Could not open version file {}: {}", path.display(), e);
            return None;
        },
    };

    // Parse the version into an u32.
    match String::from_utf8(content) {
        Err(e) => {
            error!(origin="app", "Could not read version file as string {}: {}", path.display(), e);
            return None;
        },

        Ok(s) => match s.parse::<u32>() {
            Ok(1) => {
                info!(origin="app", "APP resources are up to date");
                Some( true )
            },
            Ok(v) => {
                warn!(origin="app", "APP resources are out of date: {:08X} - 0x00000001", v);
                Some( false )
            },
            Err(e) => {
                warn!(origin="app", "Could not parse the version: {}", e);
                Some( false )
            },
        },
    }
}

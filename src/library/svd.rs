//! SVD section of the library.



use std::{
    collections::HashMap,

    path::PathBuf,

    sync::Arc,
};



pub struct SVDLibrary {
    /// List of the names of all the targets.
    targetnames: Vec<String>,

    /// List of all known targets and their associated file.
    targets: HashMap<String, Arc<PathBuf>>,

    /// The path of the SVD library.
    path: PathBuf,

    /// The search engine of the SVD library.
    search: HashMap<String, Vec<usize>>,
}

impl SVDLibrary {
    pub fn new(path: PathBuf) -> Self {
        SVDLibrary {
            targetnames: Vec::new(),

            targets: HashMap::new(),

            path,

            search: HashMap::new(),
        }
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    /// Returns the file path for a given target name.
    pub fn file(&self, target: &String) -> Option<Arc<PathBuf>> {
        match self.targets.get( target ) {
            Some( path ) => Some( path.clone() ),
            _ => None,
        }
    }

    /// Rebuilds the list of targets and files.
    /// This is an expensive operation and should only be done once a file
    /// change occurs.
    pub async fn rebuild(&mut self) {
        // Maximum search depth.
        const DEPTH: usize = 5;

        // Check that the path exists.
        if !self.path.exists() {
            return;
        }

        // Read the entries in the database.
        // ********************************************************************

        // List of directories to parse.
        let mut dirs = Vec::new();
        dirs.push( self.path.clone() );

        // List of files to parse.
        let mut files = Vec::new();

        for i in 0..DEPTH {
            // Build a list of the new directories to search.
            let mut newdirs = Vec::new();

            for dir in dirs.iter() {
                // Get all entries in the directory.
                let mut entries = match tokio::fs::read_dir(dir).await {
                    Ok(entries) => entries,
                    _ => continue,
                };

                // Classify all entries into folders and files, ignore symlinks.
                while let Ok(Some(entry)) = entries.next_entry().await {
                    // Get the path of the entry.
                    let path = entry.path();

                    // If symlink, ignore.
                    if path.is_symlink() {
                        continue;
                    }

                    // Is the entry a file?
                    if path.is_file() {
                        files.push( path );
                        continue;
                    }

                    // Is the entry a dir?
                    if path.is_dir() {
                        newdirs.push( path );
                        continue;
                    }
                }
            }

            // If still within depth, update the dirs.
            if i < (DEPTH - 1) {
                dirs = newdirs;
            }
        }

        // Read all the files and targets.
        // ********************************************************************

        // List of targets with their associated files' names.
        let mut targets = HashMap::new();

        // List of file names with their associated file paths.
        let mut filepaths = HashMap::new();

        for file in files.iter() {
            // Check if the file is a 'contents.txt'.
            if self.contents(file, &mut targets).await {
                continue;
            }

            if self.svdfile(file, &mut filepaths).await {
                continue;
            }
        }

        // Build the new map of targets to file.
        // ********************************************************************

        // Create the hashmap.
        let mut new = HashMap::new();

        for (target, filename) in targets.iter() {
            match filepaths.get( filename ) {
                Some(filepath) => if let Some(_) = new.insert( target.clone(), filepath.clone() ) {
                    continue;
                },
                _ => continue,
            }
        }

        // Create the new list of target names.
        self.targetnames = new.keys()
            .map(|string| string.clone())
            .collect();

        // Sort the names alphabetically.
        self.targetnames.sort();

        // Store the new targets.
        self.targets = new;

        // Build the search engine.
        self.engine();

        // Estimate the final size of the search engine.
        let mut bytes = 0;

        for (key, list) in self.search.iter() {
            // Add the length of the key.
            bytes += key.len();

            // Add the usizes.
            bytes += list.len() * core::mem::size_of::<usize>();
        }

        // Add the sizes of the string pointers.
        bytes += core::mem::size_of::<String>() * self.search.len();

        println!("Search engine is {} kB ({} entries)", bytes as f64 / 1024.0, self.search.len());
    }

    /// Checks if the file is a 'contents.txt' file and parses it.
    async fn contents(&mut self, file: &PathBuf, map: &mut HashMap<String, String>) -> bool {
        use tokio::{
            fs::File,
            io::{
                BufReader, AsyncBufReadExt,
            },
        };

        // Check if the file's name is 'contents.txt'.
        // ********************************************************************

        match file.file_name() {
            Some(string) => match string.to_str() {
                Some("content.txt")  => (),
                _ => return false,
            },

            _ => return false,
        }

        // Open the file as lines.
        // ********************************************************************

        // Attempt to open the file.
        let file = match File::open(file).await {
            Ok(file) => file,
            _ => return false,
        };

        // Build the buffered reader.
        let reader = BufReader::new(file);

        // Read the buffer as lines.
        let mut lines = reader.lines();

        // Process the file.
        // ********************************************************************

        while let Ok(Some(line)) = lines.next_line().await {
            // If the line starts with '#' it's a comment.
            if line.starts_with("#") || line.is_empty() {
                continue;
            }

            // Split the line by the end colon.
            let split = line.split(':').collect::<Vec<&str>>();

            // Split the first split by commas.
            let targets = split[0].split(',').collect::<Vec<&str>>();

            // Get the file name.
            let filename = String::from(split[1]);

            for key in targets {
                map.insert( String::from( key ), filename.clone() );
            }
        }

        true
    }

    /// Checks if the file is a SVD file and adds it to the list.
    async fn svdfile(&mut self, file: &PathBuf, files: &mut HashMap<String, Arc<PathBuf>>) -> bool {
        // Check if the file's extension is 'svd'.
        // ********************************************************************

        match file.extension() {
            Some(string) => match string.to_str() {
                Some("svd") | Some("SVD") => (),
                _ => return false,
            },

            _ => return false,
        }

        // Process the file.
        // ********************************************************************

        // Get the file name.
        let name = match file.file_name() {
            Some(string) => match string.to_str() {
                Some(name) => String::from(name),

                _ => return true,
            },

            _ => return true,
        };

        // Insert it in the map.
        files.insert( name, Arc::new( file.clone() ) );

        true
    }

    /// Creates the search engine for partial name searches.
    fn engine(&mut self) {
        // Create the suffix hashmap.
        let mut suffix = HashMap::new();

        for (i, name) in self.targetnames.iter().enumerate() {
            for j in 0..name.len() {
                for k in (j+1)..name.len() {
                    // Create the substring.
                    let substring = String::from( &name[j..=k] ).to_ascii_lowercase();

                    // Check if the substring has been added already.
                    match suffix.get_mut(&substring) {
                        None => match suffix.insert(substring, vec![i]) {
                            Some(_) => panic!("SVD search engine hashmap overwrite"),
                            _ => (),
                        },

                        Some(list) => list.push(i),
                    }
                }
            }
        }

        // Save the search engine.
        self.search = suffix;
    }

    /// Returns `true` if the given name exists in the target database.
    pub fn exists(&self, name: &String) -> bool {
        self.targets.contains_key(name)
    }

    /// Returns the indices of the targets that match the given partial search.
    pub fn matches(&self, name: &String) -> Option<Vec<usize>> {
        match self.search.get(&name.to_ascii_lowercase()) {
            Some(list) => Some(list.clone()),
            _ => None,
        }
    }

    /// Returns a reference to the list of all strings.
    pub fn all(&self) -> &Vec<String> {
        &self.targetnames
    }

    /// Returns a reference to the given target.
    pub fn target(&self, index: usize) -> Option<&String> {
        self.targetnames.get( index )
    }
}

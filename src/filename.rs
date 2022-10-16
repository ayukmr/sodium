use walkdir::DirEntry;

// filename for entry
pub struct Filename {
    // filename
    filename: String,
}

impl Filename {
    // create filename
    pub fn new(entry: &DirEntry) -> Self {
        // entry filename as string
        let filename = entry
            .file_name()
            .to_str()
            .unwrap()
            .to_string();

        Self { filename }
    }

    // display filename
    pub fn display(&self) -> &str {
        &self.filename
    }
}

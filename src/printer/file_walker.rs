use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use file_format::{FileFormat, Kind};

pub struct FileWalker {}

impl FileWalker {
    pub fn new() -> Self {
        FileWalker {}
    }

    pub fn walk(&self, input_paths: &[String]) -> anyhow::Result<Vec<PathBuf>> {
        let mut paths = Vec::new();
        for input_path in input_paths {
            for entry in WalkDir::new(input_path) {
                let entry = entry?;
                if entry.file_type().is_file() {
                    // TODO: Add filtering logic (exclude/include) based on regex.
                    if !self.is_binary(&entry.path()) {
                        paths.push(entry.into_path());
                    }
                }
            }
        }
        Ok(paths)
    }

    fn is_binary(&self, path: &Path) -> bool {
        if let Ok(format) = FileFormat::from_file(path) {
            if format.kind() == Kind::Text || format == FileFormat::ArbitraryBinaryData {
                false // Treat as a non-binary file.
            } else {
                true // Treat all other file formats as binary.
            }
        } else {
            // If there's an error determining the file format, treat it as binary.
            true
        }
    }
}

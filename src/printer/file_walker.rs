use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use file_format::{FileFormat, Kind};
use glob::Pattern;
use std::collections::HashSet;

pub struct FileWalker {
    exclude_patterns: Vec<Pattern>,
}

impl FileWalker {
    pub fn new(exclude_patterns: Vec<String>) -> Self {
        let exclude_patterns = exclude_patterns
            .into_iter()
            .map(|p| Pattern::new(&p).expect("Failed to create glob pattern"))
            .collect();
        FileWalker { exclude_patterns }
    }

    pub fn walk(&self, input_paths: &[String]) -> anyhow::Result<Vec<PathBuf>> {
        let mut unique_paths = HashSet::new(); // To store unique paths
        let mut final_paths = Vec::new(); // Final list of paths to return

        for input_str in input_paths {
            let input_path = Path::new(input_str);

            // Skip if this path is excluded
            if self.is_excluded(input_path) {
                continue;
            }

            // Process directories and files differently
            if input_path.is_dir() {
                for entry in WalkDir::new(input_path) {
                    let entry = entry?;
                    let path = entry.path();

                    // Skip if the path is excluded or is a directory
                    if self.is_excluded(path) || entry.file_type().is_dir() {
                        continue;
                    }

                    // If the file is not binary and not already included, add it to the list
                    if !self.is_binary(path) && unique_paths.insert(path.to_path_buf()) {
                        final_paths.push(path.to_path_buf());
                    }
                }
            } else if input_path.is_file() {
                // If it's a single file, add it if it's not binary and not already included
                if !self.is_binary(input_path) && unique_paths.insert(input_path.to_path_buf()) {
                    final_paths.push(input_path.to_path_buf());
                }
            }
        }
        Ok(final_paths)
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

    fn is_excluded(&self, path: &Path) -> bool {
        self.exclude_patterns.iter().any(|pattern| pattern.matches_path(path))
    }
}

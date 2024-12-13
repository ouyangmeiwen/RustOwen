use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// FileUtils: A utility struct for common file operations
pub struct FileUtils;

impl FileUtils {
    /// Check if a file exists
    pub fn exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    /// Copy a file from `src` to `dest`
    pub fn copy(src: &str, dest: &str) -> io::Result<u64> {
        fs::copy(src, dest)
    }

    /// Remove a file
    pub fn remove_file(path: &str) -> io::Result<()> {
        fs::remove_file(path)
    }

    /// Remove a directory and its contents
    pub fn remove_dir_all(path: &str) -> io::Result<()> {
        fs::remove_dir_all(path)
    }

    /// Create a new directory
    pub fn create_dir(path: &str) -> io::Result<()> {
        fs::create_dir(path)
    }

    /// Create a new directory and all its parent components if they are missing
    pub fn create_dir_all(path: &str) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    /// Rename or move a file or directory
    pub fn rename(src: &str, dest: &str) -> io::Result<()> {
        fs::rename(src, dest)
    }

    /// Get the file name from a given path
    pub fn file_name(path: &str) -> Option<String> {
        Path::new(path)
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string())
    }

    /// Get the file stem (name without extension)
    pub fn file_stem(path: &str) -> Option<String> {
        Path::new(path)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(|s| s.to_string())
    }

    /// Get the file extension
    pub fn extension(path: &str) -> Option<String> {
        Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string())
    }

    /// Get the absolute path of a file
    pub fn absolute_path(path: &str) -> io::Result<PathBuf> {
        fs::canonicalize(path)
    }
}

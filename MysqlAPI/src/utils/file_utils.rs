use encoding_rs::*;
use std::fs;
use std::fs::File;
use std::io::{self, Read};
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
    /// Read the entire contents of a file into a String
    pub fn read_file(path: &str) -> io::Result<String> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
    /// 读取文件并根据指定编码进行解码
    pub fn read_file_with_encoding(path: &str, encoding: &str) -> io::Result<String> {
        // 读取文件内容到字节数组
        let mut file = File::open(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        // 根据编码名称选择相应的解码器
        let encoding = match encoding.to_lowercase().as_str() {
            "utf-8" => UTF_8,
            "utf-16be" => UTF_16BE,
            "utf-16le" => UTF_16LE,
            "iso-8859-1" => WINDOWS_1252, // ISO-8859-1 和 Windows-1252 是兼容的
            "windows-1252" => WINDOWS_1252,
            "gbk" => GBK,
            "shift-jis" => SHIFT_JIS,
            "euc-kr" => EUC_KR,
            // 添加更多编码
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Unsupported encoding",
                ))
            }
        };

        // 解码字节数据
        let (decoded, _, _) = encoding.decode(&contents);
        Ok(decoded.to_string())
    }
}

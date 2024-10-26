use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref CONFIG: Mutex<Config> = Mutex::new(Config::new());
    pub static ref DEBUG_MODE: Mutex<bool> = Mutex::new(false);
}

#[derive(Debug)]
pub enum PCHStdError {
    PermissionDenied,
    FileNotFound,
    FileAlreadyExists,
    InvalidData,
    InvalidInput,
    TimedOut,
    WriteZero,
    Interrupted,
    WrongPath,
    EmptyInput,
    NoExtension,
    ConversionFailed,
    FileNameExtractFailed,
    CompressionFailed,
    Other,
}

impl From<std::io::Error> for PCHStdError {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::PermissionDenied => Self::PermissionDenied,
            std::io::ErrorKind::NotFound => Self::FileNotFound,
            std::io::ErrorKind::AlreadyExists => Self::FileAlreadyExists,
            std::io::ErrorKind::InvalidData => Self::InvalidData,
            std::io::ErrorKind::InvalidInput => Self::InvalidInput,
            std::io::ErrorKind::TimedOut => Self::TimedOut,
            std::io::ErrorKind::WriteZero => Self::WriteZero,
            std::io::ErrorKind::Interrupted => Self::Interrupted,
            _ => Self::Other,
        }
    }
}

impl From<std::path::StripPrefixError> for PCHStdError {
    fn from(_: std::path::StripPrefixError) -> Self {
        Self::WrongPath
    }
}

impl From<serde_json::Error> for PCHStdError {
    fn from(_error: serde_json::Error) -> Self {
        Self::InvalidData
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub file_names: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        let mut file_names = Vec::new();
        file_names.push("".to_string());
        Self { file_names }
    }

    pub async fn from_file<T: AsRef<Path>>(filename: T) -> Result<Self, PCHStdError> {
        if filename.as_ref().exists() {
            let file = tokio::fs::read_to_string(filename).await?;
            let config: Config = serde_json::from_str(&file)?;
            Ok(config)
        } else {
            let config = serde_json::from_str("{}")?;
            tokio::fs::write(filename, serde_json::to_string_pretty(&config)?).await?;
            Ok(config)
        }
    }
}

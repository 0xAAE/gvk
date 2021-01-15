use crate::vk_provider::AuthResponse;
use std::env::vars_os;
use std::fmt;
use std::fs::{read_to_string, write};
use std::path::Path;
use tokio::fs::File as TokioFile;
use tokio::io::{AsyncReadExt, AsyncWriteExt}; // for read_to_end() / write_all()

pub struct Storage {
    cache_home: String,
}

pub enum StorageError {
    JsonSerialize,
    JsonDeserialize,
    JsonUtf8,
    CreateFile(String),
    OpenFile(String),
    ReadWriteFile(String),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StorageError::JsonSerialize => write!(f, "serializing to JSON failed"),
            StorageError::JsonDeserialize => write!(f, "deserializing from JSON failed"),
            StorageError::JsonUtf8 => write!(f, "non-utf8 content while deserializing from JSON"),
            StorageError::CreateFile(name) => write!(f, "failed creating file {}", name),
            StorageError::OpenFile(name) => write!(f, "failed opening file {}", name),
            StorageError::ReadWriteFile(err) => {
                write!(f, "failed reading or writing file: {}", err)
            }
        }
    }
}

impl Storage {
    pub fn new() -> Self {
        // see https://specifications.freedesktop.org/basedir-spec/latest/ar01s03.html
        let mut home_dir = ".".to_string();
        let mut cache_dir = ".cache".to_string();
        for (key, value) in vars_os() {
            if let Some(key) = key.to_str() {
                if let Some(value) = value.to_str() {
                    match key {
                        "HOME" => home_dir = value.to_string(),
                        "XDG_CACHE_HOME" => cache_dir = value.to_string(),
                        &_ => {}
                    }
                }
            }
        }
        Storage {
            cache_home: home_dir + "/" + &cache_dir + "/gvk",
        }
    }

    pub fn get_cache_dir(&self) -> &str {
        &self.cache_home
    }

    pub fn get_auth_file_name(&self) -> String {
        self.cache_home.clone() + "/auth.json"
    }

    pub async fn save_auth_async(&self, auth: &AuthResponse) -> Result<(), StorageError> {
        let v = serde_json::to_string(auth).map_err(|_| StorageError::JsonSerialize)?;

        let auth_file = self.get_auth_file_name();
        let mut file = TokioFile::create(&auth_file)
            .await
            .map_err(|_| StorageError::CreateFile(auth_file))?;

        file.write_all(v.as_str().as_bytes())
            .await
            .map_err(|e| StorageError::ReadWriteFile(format!("{}", e).into()))?;

        Ok(())
    }

    pub async fn load_auth_async(&self) -> Result<AuthResponse, StorageError> {
        let auth_file = self.get_auth_file_name();
        let mut file = TokioFile::open(&auth_file)
            .await
            .map_err(|_| StorageError::OpenFile(auth_file))?;

        let mut content = vec![];
        file.read_to_end(&mut content)
            .await
            .map_err(|e| StorageError::ReadWriteFile(format!("{}", e).into()))?;

        let auth: AuthResponse = serde_json::from_str(
            std::str::from_utf8(&content).map_err(|_| StorageError::JsonUtf8)?,
        )
        .map_err(|_| StorageError::JsonDeserialize)?;

        Ok(auth)
    }

    // alternative sync version
    #[allow(dead_code)]
    pub fn save_auth(&self, auth: &AuthResponse) -> Result<(), StorageError> {
        let v = serde_json::to_string(auth).map_err(|_| StorageError::JsonSerialize)?;
        let auth_file = self.get_auth_file_name();
        let path = Path::new(&auth_file);
        write(&path, v.as_bytes()).map_err(|_| StorageError::CreateFile(auth_file))?;
        Ok(())
    }

    // alternative sync version
    #[allow(dead_code)]
    pub fn load_auth(&self) -> Result<AuthResponse, StorageError> {
        let auth_file = self.get_auth_file_name();
        let path = Path::new(&auth_file);
        let s = read_to_string(&path).map_err(|_| StorageError::OpenFile(auth_file))?;

        let auth: AuthResponse =
            serde_json::from_str(&s).map_err(|_| StorageError::JsonDeserialize)?;
        Ok(auth)
    }
}

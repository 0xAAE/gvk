use crate::vk_provider::AuthResponse;
use std::collections::HashMap;
use std::env::vars_os;
use std::fmt;
use std::fs::{read_to_string, write};
use std::path::Path;
use std::sync::{Arc, RwLock};
use tokio::fs::File as TokioFile;
use tokio::io::{AsyncReadExt, AsyncWriteExt}; // for read_to_end() / write_all()

pub mod download;

pub type SharedStorage = Arc<Storage>;

pub struct Storage {
    // root path
    cache_home: String,
    // file storage
    cache_files: String,
    // URL --> file pathname
    files: RwLock<HashMap<String, String>>,
}

pub enum StorageError {
    JsonSerialize,
    JsonDeserialize,
    JsonUtf8,
    CreateFile(String),
    OpenFile(String),
    ReadWriteFile(String),
    DownloadFile(String),
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
            StorageError::DownloadFile(name) => write!(f, "failed downloading file {}", name),
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
        // root cache
        let cache_home = home_dir + "/" + &cache_dir + "/gvk";
        // files cache
        let mut cache_files = cache_home.clone() + "/files";
        if std::fs::create_dir_all(&Path::new(cache_files.as_str())).is_err() {
            println!("failed creating files cache in {}", cache_files.as_str());
            cache_files = cache_home.clone();
        }
        // tune-up RVK tracing
        let mut trace_dir = cache_home.clone() + "/failed";
        if std::fs::create_dir_all(&Path::new(trace_dir.as_str())).is_err() {
            trace_dir = cache_home.clone();
        }
        std::env::set_var("RVK_TRACE_DIR", trace_dir.as_str());
        Storage {
            cache_home,
            cache_files,
            files: RwLock::new(HashMap::new()),
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

    /// If file is already in cache returns its pathname,
    /// otherwise downloads file, then caches it and also return its pathname
    pub async fn get_file(&self, uri: &str) -> Result<String, StorageError> {
        if uri.is_empty() {
            Err(StorageError::DownloadFile("name not set".into()))
        } else {
            {
                if let Ok(files_read) = self.files.read() {
                    if let Some(exists) = files_read.get(uri) {
                        return Ok(exists.clone());
                    }
                }
            }
            download::file(uri, self.cache_files.as_str())
                .await
                .map(|s| {
                    if let Ok(mut write) = self.files.write() {
                        write.insert(uri.to_string(), s.clone());
                        s
                    } else {
                        println!("unrecoverable inner error: cannot access files cache");
                        String::new()
                    }
                })
                .map_err(|e| {
                    println!("download error: {}", e);
                    StorageError::DownloadFile(uri.to_string())
                })
        }
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

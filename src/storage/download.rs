use std::fmt;
use std::fs::File;
use std::io::copy;
use std::io::Cursor;

pub enum DownloadError {
    // Uri incorrect or empty
    Malformed,
    // requesting URI failed
    UriGet,
    // reading content from response failed
    Content,
    // creating local file failed
    CreateFile(String),
    // saving downloaded content to file failed
    SaveFile(String),
}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DownloadError::Malformed => write!(f, "download URI is malformed or empty"),
            DownloadError::UriGet => write!(f, "requesting URI failed"),
            DownloadError::Content => write!(f, "reading content from response failed"),
            DownloadError::CreateFile(name) => write!(f, "failed creating file {}", name),
            DownloadError::SaveFile(name) => write!(f, "failed writing file {}", name),
        }
    }
}

pub async fn file(uri: &str, local_dir: &str, name_prefix: &str) -> Result<String, DownloadError> {
    if uri.is_empty() {
        return Err(DownloadError::Malformed);
    }
    let response = reqwest::get(uri).await.map_err(|_| DownloadError::UriGet)?;
    let name = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin");
    let pathname = local_dir.to_string() + format!("/{}", name_prefix).as_str() + name;
    let mut dest = match File::create(&pathname) {
        Ok(f) => f,
        Err(_) => return Err(DownloadError::CreateFile(pathname)),
    };
    let content = response.bytes().await.map_err(|_| DownloadError::Content)?;
    let mut content = Cursor::new(content);
    match copy(&mut content, &mut dest) {
        Ok(_) => {
            log::debug!("{}", pathname);
            Ok(pathname)
        }
        Err(_) => Err(DownloadError::SaveFile(pathname)),
    }
}

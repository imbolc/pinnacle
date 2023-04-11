//! Pinnacle API client with the ability to cache requests in the file system.
//! This replicates the [PinnacleClient] API and is designed for easy client swapping during
//! development.
use crate::client::{PinnacleClient, PinnacleClientError};
use crate::traits::PinnacleApiClient;
use crate::util::{error_chain, parse_json};
use async_trait::async_trait;
use reqwest::{IntoUrl, Url};
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;
use std::fs;
use std::marker::Send;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// Pinnacle API client
#[derive(Debug)]
pub struct PinnacleCachingClient {
    client: PinnacleClient,
    cache_dir: PathBuf,
    cache_ttl: Duration,
}

impl PinnacleCachingClient {
    /// Creates a new client
    pub fn new(
        username: impl Into<String>,
        password: impl Into<String>,
        cache_dir: impl Into<PathBuf>,
        cache_ttl: Duration,
    ) -> Self {
        let client = PinnacleClient::new(username, password);
        let cache_dir = cache_dir.into();
        create_cache_dir(cache_dir.as_path());
        Self {
            client,
            cache_dir,
            cache_ttl,
        }
    }

    fn from_cache<T>(&self, url: &Url) -> Option<T>
    where
        T: DeserializeOwned + Serialize,
    {
        let filename = self.filename_by_url(url);
        file_modified_ago_if_exists(filename.as_path())
            .filter(|modified| modified < &self.cache_ttl)
            .and_then(|_| read_file(filename.as_path()))
            .and_then(|s| from_json(&s))
    }

    fn to_cache<T: Serialize>(&self, url: &Url, data: &T) {
        let Some(content) = to_json(data) else { return };
        let filename = self.filename_by_url(url);
        write_file(filename.as_path(), &content);
    }

    fn filename_by_url(&self, url: &Url) -> PathBuf {
        self.cache_dir
            .as_path()
            .join(url_to_filename(url))
            .with_extension("json")
    }
}

#[async_trait]
impl PinnacleApiClient for PinnacleCachingClient {
    type Error = PinnacleClientError;

    async fn get_by_url<U, T>(&self, url: U) -> Result<T, Self::Error>
    where
        U: IntoUrl + Send,
        T: DeserializeOwned + Serialize + Send,
    {
        let url = url.into_url()?;
        let data = if let Some(data) = self.from_cache(&url) {
            data
        } else {
            let data = self.client.get_by_url(url.clone()).await?;
            self.to_cache(&url, &data);
            data
        };
        Ok(data)
    }
}

fn to_json(data: &impl Serialize) -> Option<String> {
    match serde_json::to_string(data) {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("Can't serialize data <-- {}", error_chain(&e));
            None
        }
    }
}

fn from_json<T: DeserializeOwned>(s: &str) -> Option<T> {
    match parse_json(s) {
        Ok(data) => Some(data),
        Err(e) => {
            eprintln!("Can't deserialize data <-- {}", error_chain(&e));
            None
        }
    }
}

fn create_cache_dir(path: &Path) {
    if let Err(e) = fs::create_dir_all(path) {
        eprintln!("Can't create cache folder {path:?} <-- {}", error_chain(&e));
    }
}

fn write_file(path: &Path, content: &str) {
    if let Err(e) = fs::write(path, content) {
        eprintln!("Can't write into file {path:?} <-- {}", error_chain(&e));
    }
}

fn read_file(path: &Path) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("Can't read file {path:?} <-- {}", error_chain(&e));
            None
        }
    }
}

fn url_to_filename(url: &Url) -> String {
    let mut filename = String::new();

    if let Some(path_segments) = url.path_segments() {
        for (i, segment) in path_segments.enumerate() {
            if i > 0 {
                filename.push_str("_");
            }
            filename.push_str(&segment);
        }
    }

    if let Some(query) = url.query() {
        filename.push_str("_");
        filename.push_str(&query.replace("?", "").replace("&", "_").replace("=", "-"));
    }

    filename
}

/// Returns the time elapsed since the file modification, or None for a non-existent file.
fn file_modified_ago_if_exists(path: &Path) -> Option<Duration> {
    if !path.is_file() {
        return None;
    }
    match file_modified_ago(path) {
        Ok(d) => Some(d),
        Err(e) => {
            eprintln!(
                "Can't get file modification time {path:?} <-- {}",
                error_chain(&*e)
            );
            None
        }
    }
}

fn file_modified_ago(path: &Path) -> Result<Duration, Box<dyn Error>> {
    let modified = fs::metadata(path)?.modified()?;
    let now_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let modified_since_epoch = modified.duration_since(SystemTime::UNIX_EPOCH)?;
    now_since_epoch
        .checked_sub(modified_since_epoch)
        .ok_or("times missmatch".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_to_filename() {
        let url =
            Url::parse("https://example.com/path/to/file?param1=value1&param2=value2").unwrap();
        let filename = url_to_filename(&url);
        assert_eq!(filename, "path_to_file_param1-value1_param2-value2");
    }

    #[test]
    fn test_file_modified_ago() {
        assert!(file_modified_ago(&Path::new("Cargo.toml")).is_ok());
        assert!(file_modified_ago(&Path::new("not-existing-file")).is_err());
    }
}

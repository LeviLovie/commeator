use anyhow::{Context, Result};
use dioxus::prelude::debug;

#[cfg(target_arch = "wasm32")]
use gloo_storage::Storage as _;

pub struct Storage;

impl Storage {
    pub fn new() -> Self {
        Self
    }

    pub async fn save_string(&self, filename: String, data: String) -> Result<()> {
        #[cfg(target_arch = "wasm32")]
        {
            gloo_storage::LocalStorage::set(filename, data).context("Failed to write file")
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            std::fs::write(
                data_dir()
                    .context("Failed to get the data dir")?
                    .push(filename),
                data,
            )
            .context("Failed to write file")
        }
    }

    pub async fn load_string(&self, filename: String) -> Result<String> {
        #[cfg(target_arch = "wasm32")]
        {
            use gloo_storage::{LocalStorage, Storage};

            Ok(LocalStorage::get(&filename).unwrap_or_else(|_| {
                let default = String::new();
                LocalStorage::set(&filename, &default).ok();
                default
            }))
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut path: std::path::PathBuf = data_dir().context("Failed to get the data dir")?;
            std::fs::create_dir_all(&path).context("Failed to create data directory")?;

            path.push(filename);

            if path.exists() {
                return std::fs::read_to_string(&path).context("Failed to read existing file");
            }

            let default_content = String::new();
            std::fs::write(&path, &default_content).context("Failed to create new file")?;

            Ok(default_content)
        }
    }

    pub async fn save<T: serde::Serialize>(&self, filename: String, data: T) -> Result<()> {
        let serialized = ron::to_string(&data).context("Failed to serialize data")?;
        self.save_string(filename, serialized).await
    }

    pub async fn load<T: serde::de::DeserializeOwned>(&self, filename: String) -> Result<T> {
        let data = self.load_string(filename).await?;
        let deserialized = ron::from_str(&data).context("Failed to deserialize data")?;
        Ok(deserialized)
    }

    pub fn save_block<T: serde::Serialize>(&self, filename: &str, data: &T) -> Result<()> {
        futures::executor::block_on(self.save(filename.to_string(), data.clone()))
    }

    pub fn load_block<T: serde::de::DeserializeOwned>(&self, filename: &str) -> Result<T> {
        futures::executor::block_on(self.load(filename.to_string()))
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn data_dir() -> Result<std::path::PathBuf> {
    let mut dir = dirs::config_dir().context("Could not find config directory")?;
    #[cfg(not(debug_assertions))]
    dir.push("Commeator");
    #[cfg(debug_assertions)]
    dir.push("CommeatorDev");

    if !dir.exists() {
        std::fs::create_dir_all(&dir).context("Could not create config directory")?;
    }

    debug!("Using data directory: {:?}", dir);

    Ok(dir)
}

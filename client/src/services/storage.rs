use dioxus::prelude::debug;

pub struct StorageService;

impl StorageService {
    pub fn new() -> Self { Self }

    pub async fn save(&self, filename: String, data: String) -> Result<()> {
        #[cfg(target_arch = "wasm32")]
        { gloo_storage::LocalStorage::set(filename, data).context("Failed to write file") }

        #[cfg(not(target_arch = "wasm32"))]
        { std::fs::write(data_dir().push(filename), data).context("Failed to write file") }
    }

    pub async fn load(&self, filename: String) -> Result<String> {
        #[cfg(target_arch = "wasm32")]
        { gloo_storage::LocalStorage::get(filename).context("Failed to read file") }

        #[cfg(not(target_arch = "wasm32"))]
        { std::fs::read_to_string(data_dir().push(filename)).context("Failed to read file") }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn data_dir() -> Result<std::path::PathBuf> {
    let mut dir = dirs::config_dir().context("Could not find config directory")?;
    #[cfg(not(debug_assertions))]
    dir.push("Commeator");
    #[cfg(debug_assertions)]
    dir.push("CommeatorDev");

    if (!dir.exists()) {
        std::fs::create_dir_all(&dir).context("Could not create config directory")?;
    }

    debug!("Using data directory: {:?}", dir);

    Ok(dir)
}


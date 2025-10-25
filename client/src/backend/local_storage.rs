use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Default)]
struct LocalStorage {
    jwt: Option<String>,
}

fn storage_path() -> PathBuf {
    let mut path = config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("commeator");
    fs::create_dir_all(&path).unwrap();
    path.push("storage.json");
    path
}

#[allow(dead_code)]
pub fn save_jwt(token: &str) {
    let data = LocalStorage {
        jwt: Some(token.to_string()),
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::write(storage_path(), json).unwrap();
}

pub fn load_jwt() -> Option<String> {
    let path = storage_path();
    if !path.exists() {
        return None;
    }
    let data = fs::read_to_string(path).ok()?;
    let storage: LocalStorage = serde_json::from_str(&data).ok()?;
    storage.jwt
}

pub fn delete_jwt() {
    let path = storage_path();
    if path.exists() {
        fs::remove_file(path).unwrap();
    }
}

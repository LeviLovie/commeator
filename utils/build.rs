fn main() {
    if std::path::Path::new(".env").exists() {
        if let Err(e) = dotenvy::from_filename(".env") {
            panic!("Failed to read .env: {}", e);
        }
    } else {
        eprintln!("Warning .env not found, skipping dotenv loading");
    }

    let base_url_api = std::env::var("BASE_URL_API").unwrap_or_default();
    let base_url_auth = std::env::var("BASE_URL_AUTH").unwrap_or_default();
    let base_url_wss = std::env::var("BASE_URL_WSS").unwrap_or_default();
    let base_url_web = std::env::var("BASE_URL_WEB").unwrap_or_default();

    println!("cargo:rustc-env=BASE_URL_API={}", base_url_api);
    println!("cargo:rustc-env=BASE_URL_AUTH={}", base_url_auth);
    println!("cargo:rustc-env=BASE_URL_WSS={}", base_url_wss);
    println!("cargo:rustc-env=BASE_URL_WEB={}", base_url_web);
}

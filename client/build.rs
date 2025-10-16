fn main() {
    if let Err(e) = dotenvy::from_filename(".env") {
        panic!("Failed to read .env: {}", e);
    }

    let base_url_wss = std::env::var("BASE_URL_WSS").unwrap_or_default();

    println!("cargo:rustc-env=BASE_URL_WSS={}", base_url_wss);
}

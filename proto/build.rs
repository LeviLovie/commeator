use std::path::Path;

const INCLUDES: &[&str] = &["schema"];

fn main() {
    let protos = get_protos();

    prost_build::Config::new()
        .out_dir(std::env::var("OUT_DIR").unwrap())
        .compile_protos(&protos, INCLUDES)
        .expect("Failed to compile .proto files");

    for include in INCLUDES {
        println!("cargo:rerun-if-changed={}", include);
    }
}

fn get_protos() -> Vec<String> {
    let mut protos = Vec::new();

    for include in INCLUDES {
        let schema_dir = Path::new(include);

        for entry in std::fs::read_dir(schema_dir).expect("Failed to read schema directory") {
            let entry = entry.expect("Failed to read schema entry");
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("proto") {
                protos.push(path.to_str().unwrap().to_string());
            }
        }
    }

    protos
}

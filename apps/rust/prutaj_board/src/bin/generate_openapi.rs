use std::fs;
use std::path::Path;

use prutaj_board::ApiDoc;
use utoipa::OpenApi;

fn main() {
    let yaml = ApiDoc::openapi()
        .to_yaml()
        .expect("Failed to serialize OpenAPI specification to YAML");

    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("specification.yml");
    fs::write(&path, yaml).expect("Failed to write specification.yml");

    println!("Wrote {}", path.display());
}

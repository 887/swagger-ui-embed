#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]

use std::io::Write;

fn main() -> Result<(), ()> {
    let dist = "https://raw.githubusercontent.com/swagger-api/swagger-ui/refs/heads/master/dist/";

    update_file(dist, "oauth2-redirect.html");
    update_file(dist, "swagger-ui.css");
    update_file(dist, "swagger-ui-bundle.js");
    update_file(dist, "swagger-ui-bundle.js");
    update_file(dist, "favicon-32x32.png");
    update_file(dist, "favicon-16x16.png");

    Ok(())
}

fn update_file(dist: &str, file: &str) {
    if let Ok(rsp) = reqwest::blocking::get(dist.to_string() + file) {
        let mut file = std::fs::File::create("src/".to_string() + file).unwrap();
        file.write_all(&rsp.bytes().unwrap()).unwrap();
    }
    println!("cargo:rerun-if-changed={}", file);
}

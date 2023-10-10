// extract_version.rs
extern crate toml;

use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
struct Package {
    version: String,
}

#[derive(Deserialize)]
struct CargoToml {
    package: Package,
}

fn main() {
    let mut file = File::open("Cargo.toml").expect("Could not open Cargo.toml");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read Cargo.toml");

    let cargo_toml: CargoToml = toml::from_str(&contents).expect("Could not parse Cargo.toml");
    println!("{}", cargo_toml.package.version);
}

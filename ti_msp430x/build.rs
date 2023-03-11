fn main() {
    let local = 
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| "".into());
    let filename = std::path::Path::new(&local).join("../Processors/TI_MSP430");
    println!("cargo:rerun-if-changed={}", filename.to_str().unwrap());
}
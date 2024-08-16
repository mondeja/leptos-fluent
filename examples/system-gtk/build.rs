fn main() {
    // needed by system-deps
    println!("cargo::rerun-if-changed=Cargo.toml");
    system_deps::Config::new().probe().unwrap();
}

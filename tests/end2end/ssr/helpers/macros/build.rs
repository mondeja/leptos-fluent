fn main() {
    println!("cargo:rerun-if-changed=force_rebuild_never_exists.txt");
}

fn main() {
    // Rerun always
    println!("cargo:rerun-if-changed=force_rebuild_never_exists.txt");
}

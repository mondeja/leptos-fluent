use which::which;

fn main() {
    if which("chromedriver").is_err() {
        panic!("❌ chromedriver not found in PATH. Please install it to run the end-to-end SSR tests.");
    }
}

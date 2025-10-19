const CHROMEDRIVER_PORT: u16 = 9515;

pub fn start_chromedriver() -> std::process::Child {
    std::process::Command::new("chromedriver")
        .args([format!("--port={CHROMEDRIVER_PORT}")])
        .spawn()
        .expect("failed to start chromedriver")
}

pub fn wait_until_chromedriver_ready() {
    use reqwest::blocking::get;
    use std::{thread, time::Duration};

    let url = format!("http://localhost:{CHROMEDRIVER_PORT}/status");
    let timeout = Duration::from_secs(10);
    let start = std::time::Instant::now();
    while start.elapsed() < timeout {
        if let Ok(response) = get(&url) {
            if response.status().is_success() {
                return;
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
    panic!("chromedriver did not become ready within the timeout period");
}

pub fn wait_until_chromedriver_terminated(child: &mut std::process::Child) {
    use std::{thread, time::Duration};

    let timeout = Duration::from_secs(10);
    let start = std::time::Instant::now();
    while start.elapsed() < timeout {
        match child.try_wait() {
            Ok(Some(_)) => return, // Process has exited
            Ok(None) => thread::sleep(Duration::from_millis(10)), // Still running
            Err(e) => panic!("Error while checking chromedriver status: {e}"),
        }
    }
    panic!("chromedriver did not terminate within the timeout period");
}

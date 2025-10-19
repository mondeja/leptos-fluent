pub async fn init_server(fn_name: &'static str) -> u32 {
    let app_directory = std::path::Path::new("src/apps").join(fn_name);
    let absolute_app_directory = std::env::var("CARGO_MANIFEST_DIR")
        .map(std::path::PathBuf::from)
        .unwrap()
        .join(app_directory);
    #[allow(clippy::zombie_processes)]
    std::process::Command::new("cargo")
        .args(["leptos", "serve"])
        .current_dir(absolute_app_directory)
        .spawn()
        .expect("failed to start cargo leptos serve");
    wait_until_server_ready().await
}

pub async fn wait_until_server_ready() -> u32 {
    // use lsof to check if port 3000 is being used and return its PID
    use std::process::Command;
    use std::time::{Duration, Instant};
    use tokio::time::sleep;
    let port = 3000;
    let timeout_secs = 600;

    let timeout = Duration::from_secs(timeout_secs);
    let start = Instant::now();
    loop {
        let output = Command::new("lsof")
            .args(["-i", &format!(":{port}")])
            .output()
            .expect("failed to execute lsof");

        if !output.stdout.is_empty() {
            let stdout_str = String::from_utf8_lossy(&output.stdout);
            let line = stdout_str.lines().nth(1).unwrap_or("");
            let pid = line
                .split_whitespace()
                .nth(1)
                .and_then(|pid_str| pid_str.parse::<u32>().ok());
            if let Some(pid) = pid {
                return pid;
            }
        }

        if start.elapsed() > timeout {
            panic!(
                "Timeout reached while waiting for port {port} to be in use",
            );
        }

        sleep(Duration::from_millis(200)).await;
    }
}

pub async fn wait_until_server_terminated() {
    use std::process::Command;
    use std::time::{Duration, Instant};
    use tokio::time::sleep;
    let port = 3000;
    let timeout_secs = 5;

    let timeout = Duration::from_secs(timeout_secs);
    let start = Instant::now();

    loop {
        let output = Command::new("lsof")
            .args(["-i", &format!(":{port}")])
            .output()
            .expect("failed to execute lsof");

        if output.stdout.is_empty() {
            return;
        }

        if start.elapsed() > timeout {
            panic!("Timeout reached while waiting for port {port} to be freed",);
        }

        // Esperar un poco antes de volver a chequear
        sleep(Duration::from_millis(200)).await;
    }
}

pub async fn terminate_server(pid: u32) {
    use std::process::Command;

    Command::new("kill")
        .args(["-9", &format!("{pid}")])
        .status()
        .expect("failed to terminate server process");

    wait_until_server_terminated().await;
}

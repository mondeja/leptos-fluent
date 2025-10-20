pub struct World {
    server_pid: u32,
    host: &'static str,
}

impl World {
    #[must_use]
    pub fn server_pid(&self) -> u32 {
        self.server_pid
    }

    #[must_use]
    pub fn host(&self) -> &str {
        self.host
    }

    #[must_use]
    pub fn client(&self) -> reqwest::Client {
        reqwest::Client::new()
    }

    pub fn new(server_pid: u32) -> Self {
        Self {
            server_pid,
            host: "http://127.0.0.1:3000",
        }
    }

    pub async fn with_driver(self) -> WorldWithDriver {
        let driver = WorldWithDriver::init_driver().await;
        WorldWithDriver {
            inner: self,
            driver,
        }
    }
}

pub struct WorldWithDriver {
    inner: World,
    pub driver: thirtyfour::WebDriver,
}

impl WorldWithDriver {
    #[must_use]
    pub fn driver(&self) -> &thirtyfour::WebDriver {
        &self.driver
    }

    #[must_use]
    pub fn server_pid(&self) -> u32 {
        self.inner.server_pid()
    }

    #[must_use]
    pub fn host(&self) -> &str {
        self.inner.host()
    }

    #[must_use]
    pub fn client(&self) -> reqwest::Client {
        self.inner.client()
    }

    pub async fn goto(&self, url: &str) -> anyhow::Result<()> {
        self.driver.get(url).await?;
        Ok(())
    }

    pub async fn goto_root(&self) -> anyhow::Result<()> {
        self.driver.get(self.host()).await?;
        Ok(())
    }

    pub async fn goto_path(&self, path: &str) -> anyhow::Result<()> {
        let url = format!("{}{}", self.host(), path);
        self.driver.get(&url).await?;
        Ok(())
    }

    pub async fn init_driver() -> thirtyfour::WebDriver {
        use thirtyfour::BrowserCapabilitiesHelper;
        let mut caps = thirtyfour::DesiredCapabilities::chrome();
        let opts = vec!["--no-sandbox", "--headless"];
        caps.insert_browser_option("args", opts)
            .unwrap_or_else(|err| {
                panic!("Failed to set Chrome options: {err}");
            });
        let driver = thirtyfour::WebDriver::new("http://localhost:9515", caps)
            .await
            .expect("failed to create WebDriver");

        driver
    }
}

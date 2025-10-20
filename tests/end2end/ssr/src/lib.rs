use ctor::{ctor, dtor};
use end2end_ssr_helpers::{
    start_chromedriver, wait_until_chromedriver_ready,
    wait_until_chromedriver_terminated,
};

static mut DRIVER: Option<std::process::Child> = None;

#[ctor]
fn setup_tests() {
    let child = start_chromedriver();
    wait_until_chromedriver_ready();
    unsafe {
        DRIVER = Some(child);
    }
}

#[dtor]
fn teardown_tests() {
    #[allow(static_mut_refs)]
    let maybe_child = unsafe { DRIVER.take() };
    if let Some(mut child) = maybe_child {
        let _ = child.kill();
        wait_until_chromedriver_terminated(&mut child);
        #[allow(clippy::print_stdout)]
        {
            println!("✅ chromedriver process terminated.");
        }
    } else {
        #[allow(clippy::print_stdout)]
        {
            println!("⚠️ No chromedriver process found to terminate.");
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use end2end_ssr_helpers::{World, WorldWithDriver};
    use end2end_ssr_helpers_macros::e2e_test;

    #[e2e_test]
    async fn initial_language_from_accept_language_header_axum(world: World) {
        let client = world.client();
        let response = client
            .get(world.host())
            .header("Accept-Language", "es-ES,es;q=0.9")
            .send()
            .await?;
        let body = response.text().await?;
        assert!(body.contains("¡Bienvenido a Leptos!"), "{}", body);
        assert!(!body.contains("Welcome to Leptos!"), "{}", body);

        let response = client
            .get(world.host())
            .header("Accept-Language", "en-US,en;q=0.9")
            .send()
            .await?;
        let body = response.text().await?;
        assert!(body.contains("Welcome to Leptos!"), "{}", body);
        assert!(!body.contains("¡Bienvenido a Leptos!"), "{}", body);
    }

    #[e2e_test]
    async fn initial_language_from_accept_language_header_actix(world: World) {
        let client = world.client();
        let response = client
            .get(world.host())
            .header("Accept-Language", "es-ES,es;q=0.9")
            .send()
            .await?;
        let body = response.text().await?;
        assert!(body.contains("¡Bienvenido a Leptos!"), "{}", body);
        assert!(!body.contains("Welcome to Leptos!"), "{}", body);

        let response = client
            .get(world.host())
            .header("Accept-Language", "en-US,en;q=0.9")
            .send()
            .await?;
        let body = response.text().await?;
        assert!(body.contains("Welcome to Leptos!"), "{}", body);
        assert!(!body.contains("¡Bienvenido a Leptos!"), "{}", body);
    }
}

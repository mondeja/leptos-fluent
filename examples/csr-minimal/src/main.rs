pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(leptos_fluent_csr_minimal_example::App);
}

use leptos::*;
use leptos_fluent_csr_minimal_example::App;

pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

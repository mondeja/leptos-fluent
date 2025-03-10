use leptos_fluent::{move_tr, tr};

fn main() {
    let foo = "foo";
    _ = tr!(foo);

    let bar = 1;
    _ = move_tr!(bar);
}

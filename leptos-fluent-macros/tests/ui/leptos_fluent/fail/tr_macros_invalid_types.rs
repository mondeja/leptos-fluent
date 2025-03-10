use leptos_fluent::{move_tr, tr};

struct MyStruct(u32);

fn main() {
    _ = tr!(1);

    _ = move_tr!(MyStruct(1));

    _ = tr!(());
}

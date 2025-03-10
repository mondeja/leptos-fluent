use leptos_fluent::{move_tr, tr};

fn foo() {
    // id with `tr!`
    _ = tr!("select-a-language");
    // id with `move_tr!`
    _ = move_tr!("select-a-language");
    // id + args with `tr!`
    _ = tr!("html-tag-lang-is", {"lang" => "en"});
    // id + args with `move_tr!`
    _ = move_tr!("html-tag-lang-is", {"lang" => "en"});
}

fn main() {}

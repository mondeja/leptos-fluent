//! 001 if_elseif_else
#![cfg_attr(feature = "nightly", feature(stmt_expr_attributes))]

use fluent_bundle::FluentValue;
use leptos_fluent::{move_tr, tr};
use std::borrow::Cow;

struct FakeSignal;

impl FakeSignal {
    fn new() -> Self {
        FakeSignal
    }

    fn get(&self) -> bool {
        true
    }
}

struct FakeI18n;

impl FakeI18n {
    fn new() -> Self {
        FakeI18n
    }

    fn tr(&self, key: &str) -> String {
        key.to_string()
    }

    fn tr_with_args(
        &self,
        key: &str,
        _args: &std::collections::HashMap<Cow<'static, str>, FluentValue>,
    ) -> String {
        key.to_string()
    }
}

/*
fn fail() {
    let (foo, bar) = (false, true);

    // no else branch (fails)
    _ = tr!(if foo {
        "foo"
    } else if bar {
        "bar"
    });

    let fake_i18n = FakeI18n::new();
    _ = tr!(
        i18n,
        if foo {
            "foo"
        } else if bar {
            "bar"
        }
    );
    _ = tr!(if foo {
        "foo"
    } else if bar {
        "bar"
    },
    {
        "arg1" => "value1",
        "arg2" => "value2",
    });
    _ = move_tr!(if foo {
        "foo"
    } else if bar {
        "bar"
    },
    {
        "arg1" => "value1",
        "arg2" => "value2",
    });
    _ = tr!(i18n, if foo {
        "foo"
    } else if bar {
        "bar"
    },
    {
        "arg1" => "value1",
        "arg2" => "value2",
    });

    // invalid type in branch (fails)
    _ = tr!(if foo {
        "foo"
    } else if bar {
        42
    } else {
        "baz"
    });
    _ = move_tr!(if foo {
        800u32
    } else if bar {
        42
    } else {
        "baz"
    });
}
*/

fn pass() {
    let (foo, bar) = (false, false);

    // only text_id
    _ = tr!(if foo { "foo" } else { "bar" });
    _ = move_tr!(if foo { "foo" } else { "bar" });

    // with i18n
    let fake_i18n = FakeI18n::new();
    _ = tr!(fake_i18n, if foo { "foo" } else { "bar" });
    _ = move_tr!(fake_i18n, if foo { "foo" } else { "bar" });

    // with args
    _ = tr!(
        if foo { "foo" } else { "bar" },
        {
            "arg1" => "value1",
            "arg2" => "value2",
        },
    );
    _ = move_tr!(
        if foo { "foo" } else { "bar" },
        {
            "arg1" => "value1",
            "arg2" => "value2",
        },
    );

    // with i18n and args
    let fake_i18n = FakeI18n::new();
    _ = tr!(
        fake_i18n,
        if foo { "foo" } else { "bar" },
        {
            "arg1" => "value1",
            "arg2" => "value2",
        },
    );
    let fake_i18n = FakeI18n::new();
    _ = move_tr!(
        fake_i18n,
        if foo { "foo" } else { "bar" },
        {
            "arg1" => "value1",
            "arg2" => "value2",
        },
    );

    _ = tr!(if foo { "foo" } else { "bar" },);
    _ = move_tr!(if foo { "foo" } else { "bar" },);
    _ = tr!(if foo {
        "foo"
    } else if bar {
        "bar"
    } else {
        "baz"
    });
    _ = move_tr!(if foo {
        "foo"
    } else if bar {
        "bar"
    } else {
        "baz"
    });
}

// Nightly tests because require `#![cfg(feature(stmt_expr_attributes))]`
// which is only available in nightly
#[cfg(feature = "nightly")]
fn pass_nightly() {
    // the next branch needs braces around the `bar && foo` condition
    // or a compiler warning will be raised
    let (foo, bar, baz) = (false, false, false);
    let fake_signal = FakeSignal::new();
    let fake_fn = || true;

    _ = tr!(
        #[allow(unused_braces)]
        if { foo && bar } {
            "foo"
        } else if baz {
            "baz"
        } else {
            "bar"
        }
    );

    _ = move_tr!(
        #[allow(unused_parens)]
        if (foo && bar) {
            "foo"
        } else if baz {
            "baz"
        } else {
            "bar"
        }
    );

    _ = tr!(
        "foo",
        #[allow(unused_braces)]
        {
            "arg1" => {"value1"},
            "arg2" => "value2",
        }
    );

    _ = move_tr!(
        #[allow(unused_parens)]
        if ( foo && bar ) {
            "foo"
        } else if baz {
            "baz"
        } else {
            "bar"
        },
        #[allow(unused_braces)]
        {
            "arg1" => {"value1"},
            "arg2" => "value2",
        }
    );

    _ = move_tr!(
        #[allow(unused_braces)]
        if { foo && bar } {
            "foo"
        } else if baz {
            "baz"
        } else {
            "bar"
        }
    );
    // same for functions and signals
    _ = tr!(
        #[allow(unused_parens)]
        if (fake_fn()) { "foo" } else { "bar" }
    );
    _ = tr!(
        #[allow(unused_parens)]
        if (fake_signal.get()) { "foo" } else { "bar" }
    );
    _ = tr!(
        #[allow(unused_parens)]
        if foo {
            "foo"
        } else if (fake_fn() && fake_signal.get()) {
            "bar"
        } else {
            "baz"
        }
    );
    _ = move_tr!(
        #[allow(unused_parens)]
        if foo {
            "foo"
        } else if (fake_fn() && FakeSignal::new().get()) {
            "bar"
        } else {
            "baz"
        }
    );
}

fn main() {}

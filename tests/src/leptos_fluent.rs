static LEPTOS_FLUENT_LIB_RS_CONTENT: &str =
    include_str!("../../leptos-fluent/src/lib.rs");
static LEPTOS_FLUENT_CARGO_TOML_CONTENT: &str =
    include_str!("../../leptos-fluent/Cargo.toml");
static LEPTOS_FLUENT_MACROS_CARGO_TOML_CONTENT: &str =
    include_str!("../../leptos-fluent-macros/Cargo.toml");
static LEPTOS_FLUENT_MACROS_LOADER_RS_CONTENT: &str =
    include_str!("../../leptos-fluent-macros/src/loader.rs");

pub(crate) fn major_and_minor_version(version: &str) -> String {
    version.split('.').take(2).collect::<Vec<_>>().join(".")
}

fn get_cargo_toml_version(content: &str) -> String {
    let cargo_toml = toml::from_str::<toml::Value>(content)
        .expect("Cargo.toml is not a valid TOML file")
        .get("package")
        .expect("package key not found in Cargo.toml")
        .clone();
    cargo_toml
        .get("version")
        .expect("version key not found in Cargo.toml")
        .as_str()
        .expect("version is not a string")
        .to_string()
}

pub(crate) fn get_leptos_fluent_cargo_toml_version() -> String {
    get_cargo_toml_version(LEPTOS_FLUENT_CARGO_TOML_CONTENT)
}

pub(crate) fn get_leptos_fluent_macros_cargo_toml_version() -> String {
    get_cargo_toml_version(LEPTOS_FLUENT_MACROS_CARGO_TOML_CONTENT)
}

pub(crate) fn get_leptos_fluent_macros_loader_fields() -> Vec<String> {
    let mut loader_fields = Vec::new();
    let mut index = 0;
    let lines = LEPTOS_FLUENT_MACROS_LOADER_RS_CONTENT
        .lines()
        .collect::<Vec<_>>();
    while index < lines.len() {
        let line = lines[index];
        if line.contains("if k == \"") {
            let field = line.split('"').nth(1).unwrap();
            loader_fields.push(field.to_string());
        } else if line.contains("else if k") {
            let line = lines[index + 1];
            let field = line.split('"').nth(1).unwrap();
            loader_fields.push(field.to_string());
            index += 1;
        }
        index += 1;
    }
    loader_fields
}

#[test]
fn readme_leptos_fluent_version_is_updated() {
    let mut readme_version = None;
    for line in LEPTOS_FLUENT_LIB_RS_CONTENT.lines() {
        if line.starts_with("//! leptos-fluent = ") {
            readme_version = Some(
                line.split("leptos-fluent = \"")
                    .nth(1)
                    .unwrap()
                    .split('"')
                    .next()
                    .unwrap(),
            );
            break;
        }
    }

    let leptos_fluent_version = get_leptos_fluent_cargo_toml_version();
    let leptos_fluent_macros_version =
        get_leptos_fluent_macros_cargo_toml_version();

    assert_eq!(
        leptos_fluent_version, leptos_fluent_macros_version,
        concat!(
            "The version of leptos-fluent and leptos-fluent-macros in",
            " Cargo.toml files are not the same."
        ),
    );

    assert!(
        readme_version.is_some(),
        r#"leptos-fluent = "<version>" not found in leptos-fluent/src/lib.rs"#
    );

    assert_eq!(
        major_and_minor_version(readme_version.unwrap()),
        major_and_minor_version(&leptos_fluent_version),
        concat!(
            "The version of leptos-fluent shown in the README at",
            " 'Installation' section is not updated."
        ),
    );
}

// Check that `LeptosFluentMeta`'s fields are in sync with the `leptos_fluent!` macro
#[test]
fn leptos_fluent_meta_is_updated() {
    let fields_to_ignore = ["children".to_string(), "translations".to_string()];

    // Get fields from `LeptosFluentMeta` struct
    let mut struct_fields = Vec::from(fields_to_ignore);
    let mut inside_struct_ = false;
    for line in LEPTOS_FLUENT_LIB_RS_CONTENT.lines() {
        if line == "pub struct LeptosFluentMeta {" {
            inside_struct_ = true;
            continue;
        }
        if inside_struct_ {
            if line == "}" {
                break;
            }
            if line.starts_with("    pub ") {
                let field = line
                    .split("pub ")
                    .nth(1)
                    .unwrap()
                    .split(":")
                    .next()
                    .unwrap()
                    .trim();
                struct_fields.push(field.to_string());
            }
        }
    }

    // Get fields from `leptos_fluent_macros` loader.rs conditional
    let loader_fields = get_leptos_fluent_macros_loader_fields();

    // Check that fields are in sync
    for field in &loader_fields {
        assert!(
            struct_fields.contains(&field),
            concat!(
                "Field \"{}\" from `leptos-fluent-macros/src/loader.rs`",
                " conditional is missing in `LeptosFluentMeta` struct"
            ),
            field
        );
    }
    for field in struct_fields {
        assert!(
            loader_fields.contains(&field),
            concat!("Field \"{}\" from `LeptosFluentMeta` struct",
                    " is missing in `leptos-fluent-macros/src/loader.rs` conditional"),
            field
        );
    }
}

const CARGO_TOML: &str = include_str!("../Cargo.toml");

#[test]
fn features_table_has_an_empty_default_and_unicode() {
    let normalized = CARGO_TOML.replace("\r\n", "\n");

    assert!(
        normalized.contains("\n[features]\ndefault = []\nunicode = []\n"),
        "Cargo.toml must declare empty default and unicode features"
    );
}

#[test]
fn unicode_feature_has_no_dependencies() {
    let normalized = CARGO_TOML.replace("\r\n", "\n");
    let (_, dependencies) = normalized
        .split_once("\n[dependencies]\n")
        .expect("Cargo.toml must contain a dependencies table");

    assert!(
        dependencies.trim().is_empty(),
        "the dependencies table must remain empty"
    );
}

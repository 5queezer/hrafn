#[test]
fn cargo_declares_microkernel_seed_artifacts() {
    let cargo_toml = std::fs::read_to_string("Cargo.toml").expect("read root Cargo.toml");

    assert!(
        cargo_toml.contains("\"crates/hrafn-sdk\"")
            && cargo_toml.contains("\"crates/hrafn-kernel\""),
        "workspace should include the SDK and kernel crates used as the microkernel boundary"
    );
    assert!(
        std::path::Path::new("crates/hrafn-kernel/Cargo.toml").exists(),
        "workspace should expose a tiny kernel seed package separate from the full CLI"
    );
    assert!(
        cargo_toml.contains("kernel = [\"dep:hrafn-sdk\"]"),
        "root features should include a minimal kernel feature that does not imply desktop integrations"
    );
    assert!(
        cargo_toml.contains("full = ["),
        "root features should name the full distribution profile explicitly"
    );
}

#[test]
fn sdk_crate_stays_dependency_light() {
    let sdk_toml =
        std::fs::read_to_string("crates/hrafn-sdk/Cargo.toml").expect("read hrafn-sdk Cargo.toml");

    for forbidden in [
        "reqwest",
        "axum",
        "ratatui",
        "rusqlite",
        "matrix-sdk",
        "wa-rs",
    ] {
        assert!(
            !sdk_toml.contains(forbidden),
            "hrafn-sdk must not depend on heavyweight integration crate {forbidden}"
        );
    }
}

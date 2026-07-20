use std::path::PathBuf;

fn wasm_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target/wasm32-wasip1/release/vortex_mod_captcha_browser.wasm")
}

#[test]
fn wasm_exports_request_human_interaction_without_host_capabilities() {
    let path = wasm_path();
    assert!(
        path.is_file(),
        "release WASM must be built before smoke tests"
    );
    let manifest = extism::Manifest::new([extism::Wasm::file(path)]);
    let mut plugin =
        extism::Plugin::new(&manifest, Vec::<extism::Function>::new(), true).expect("load WASM");
    let input = r#"{"challenge_id":"captcha-1","challenge_type":"text_input","challenge_url":"https://example.test","image_data":null}"#;

    let supported: String = plugin.call("can_solve", input).expect("can_solve");
    let result: String = plugin.call("solve", input).expect("solve");

    assert_eq!(supported, "true");
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&result).unwrap(),
        serde_json::json!({ "status": "interaction_required" })
    );
}

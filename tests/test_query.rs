use cmake_file_api::objects;

#[test]
fn query_writer_write_stateless_creates_files() {
    let tmp_dir = tempdir::TempDir::new("test_cmake").unwrap();
    let build_dir = tmp_dir.path();

    cmake_file_api::query::Writer::default()
        .request_object::<objects::CodeModelV2>()
        .request_object::<objects::ConfigureLogV1>()
        .request_object::<objects::CacheV2>()
        .request_object::<objects::ToolchainsV1>()
        .request_object::<objects::CMakeFilesV1>()
        .write_stateless(build_dir)
        .unwrap();

    assert!(
        cmake_file_api::query::dir(build_dir)
            .join("codemodel-v2")
            .exists(),
        "codeModel-v2 should exist"
    );
    assert!(
        cmake_file_api::query::dir(build_dir)
            .join("configureLog-v1")
            .exists(),
        "configureLog-v1 should exist"
    );
    assert!(
        cmake_file_api::query::dir(build_dir)
            .join("cache-v2")
            .exists(),
        "cache-v2 should exist"
    );
    assert!(
        cmake_file_api::query::dir(build_dir)
            .join("toolchains-v1")
            .exists(),
        "toolchains-v1 should exist"
    );
    assert!(
        cmake_file_api::query::dir(build_dir)
            .join("cmakeFiles-v1")
            .exists(),
        "cmakeFiles-v1 should exist"
    );
}

#[test]
fn query_writer_write_statefull_creates_files() {
    let tmp_dir = tempdir::TempDir::new("test_cmake").unwrap();
    let build_dir = tmp_dir.path();

    cmake_file_api::query::Writer::default()
        .set_client("test_client", serde_json::json!({"my_key": "my_value"}))
        .request_object::<objects::CodeModelV2>()
        .request_object::<objects::ConfigureLogV1>()
        .request_object::<objects::CacheV2>()
        .request_object::<objects::ToolchainsV1>()
        .request_object::<objects::CMakeFilesV1>()
        .write_stateful(build_dir)
        .unwrap();

    let client_dir = cmake_file_api::query::dir(build_dir).join("test_client");
    let query_file = client_dir.join("query.json");
    assert!(query_file.exists(), "query file should exist");

    let query = std::fs::read_to_string(&query_file).expect("query file should be readable");
    let query_json: serde_json::Value = serde_json::from_str(&query).expect("query should be json");

    // should contain client data
    assert_eq!(
        query_json["client"],
        serde_json::json!({"my_key": "my_value"}),
        "client data should be written"
    );

    // should contain requested objects
    assert_eq!(
        query_json["requests"],
        serde_json::json!([
            {"kind": "codemodel", "version": {"major": 2}},
            {"kind": "configureLog", "version": {"major": 1}},
            {"kind": "cache", "version": {"major": 2}},
            {"kind": "toolchains", "version": {"major": 1}},
            {"kind": "cmakeFiles", "version": {"major": 1}},
        ]),
        "requests should be written for each object"
    );
}

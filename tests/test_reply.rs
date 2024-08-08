use cmake_file_api::{objects, reply};

#[test]
fn test_missing_api() {
    let tmp_dir = tempdir::TempDir::new("test_cmake").unwrap();
    let empty_dir = tmp_dir.path();

    // Test that the API is not available when the directory is empty
    assert_eq!(reply::is_available(&empty_dir), false);

    // Test that the index_file function returns None when the directory is empty
    assert!(reply::index_file(&empty_dir).is_none());

    // Test for cmake_file_api::CMakeFileApiError::FileApiNotGenerated
    assert!(matches!(
        reply::Reader::from_build_dir(&empty_dir),
        Err(reply::ReaderError::FileApiNotGenerated)
    ));
}

#[test]
fn test_json_parser_error() {
    let tmp_dir = tempdir::TempDir::new("test_cmake").unwrap();
    let build_dir = tmp_dir.path();

    // create empty reply dir
    std::fs::create_dir_all(&reply::dir(&build_dir)).unwrap();

    // create broken index file
    let broken_index_file = reply::dir(&build_dir).join("index-broken.json");
    std::fs::write(&broken_index_file, "broken").unwrap();

    // Test that the API is available when the reply directory exists
    assert_eq!(reply::is_available(&build_dir), true);

    // Test that the index_file function returns None when the index file is missing
    assert_eq!(
        reply::index_file(&build_dir),
        Some(broken_index_file.clone())
    );

    // Test ReaderError::Parse
    assert!(matches!(
        reply::Reader::from_build_dir(&build_dir),
        Err(reply::ReaderError::Parse(_))
    ));
}

///
#[test]
fn test_valid_api() {
    let tmp_dir = tempdir::TempDir::new("test_cmake").unwrap();
    let project_dir = tmp_dir.path();

    // create minimal main.cpp
    {
        let main_cpp = project_dir.join("main.cpp");
        std::fs::write(&main_cpp, "int main() { return 0; }").expect("Failed to write main.cpp");
    }

    // create libfoo library
    {
        let libfoo_dir = project_dir.join("libfoo");
        std::fs::create_dir(&libfoo_dir).unwrap();

        let libfoo_cpp = libfoo_dir.join("libfoo.cpp");
        std::fs::write(
            libfoo_cpp,
            r#"
        extern int foo();
        "#,
        )
        .expect("Failed to write libfoo.cpp");

        let libfoo_h = libfoo_dir.join("libfoo.h");
        std::fs::write(
            libfoo_h,
            r#"
        #include <libfoo.cpp>
        int foo() { return 42; }
        "#,
        )
        .expect("Failed to write libfoo.h");

        let cmake_lists = libfoo_dir.join("CMakeLists.txt");
        std::fs::write(
            cmake_lists,
            r#"
        add_library(foo libfoo.cpp libfoo.h)
        target_include_directories(foo PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
        "#,
        )
        .expect("Failed to write CMakeLists.txt");
    }

    // create minimal CMakeLists.txt
    {
        let cmake_lists = project_dir.join("CMakeLists.txt");
        std::fs::write(
            cmake_lists,
            r#"
            cmake_minimum_required(VERSION 3.10)
            project(test_cmake)
            
            add_subdirectory(libfoo)
            
            add_executable(test_cmake main.cpp)
            target_link_libraries(test_cmake foo)
        "#,
        )
        .expect("Failed to write CMakeLists.txt");
    }

    let build_dir = tmp_dir.path().join("build");

    // make query
    cmake_file_api::query::Writer::default()
        .request_all_objects()
        .write_stateless(&build_dir)
        .expect("Failed to write query");

    // run cmake
    assert!(std::process::Command::new("cmake")
        .arg("-S")
        .arg(&project_dir)
        .arg("-B")
        .arg(&build_dir)
        .status()
        .expect("Failed to run cmake")
        .success());

    // Test that the API is available
    assert!(reply::is_available(&build_dir));

    // Test that the index_file function returns the index file
    assert!(reply::index_file(&build_dir).is_some());

    // Test that the CMakeFileApi::from_build_dir function returns the CMakeFileApi object
    let reader =
        reply::Reader::from_build_dir(&build_dir).expect("Reply reader should be available");

    // Test that the CMakeFileApi object can be used to get the CodeModel
    assert!(reader.has_object::<cmake_file_api::objects::CodeModelV2>());

    // Test that the CMakeFileApi object can be used to get the ConfigureLog
    assert!(reader.has_object::<cmake_file_api::objects::ConfigureLogV1>());

    // Test that the CMakeFileApi object can be used to get the Cache
    assert!(reader.has_object::<cmake_file_api::objects::CacheV2>());

    // Test that the CMakeFileApi object can be used to get the Toolchains
    assert!(reader.has_object::<cmake_file_api::objects::ToolchainsV1>());

    // Test that the CMakeFileApi object can be used to get the CMakeFiles
    assert!(reader.has_object::<cmake_file_api::objects::CMakeFilesV1>());

    // Test that the CMakeFileApi object can be used to get the codemodel
    let codemodel: objects::CodeModelV2 = reader.read_object().expect("codemodel should be available");
    assert!(codemodel.configurations.len() > 0);

    // targets should not be empty
    assert!(codemodel.configurations[0].targets.len() > 0);

    // targets and target_refs should have the same length
    assert_eq!(
        codemodel.configurations[0].targets.len(),
        codemodel.configurations[0].target_refs.len()
    );

    // directories should not be empty
    assert!(codemodel.configurations[0].directories.len() > 0);

    // directories and directory_refs should have the same length
    assert_eq!(
        codemodel.configurations[0].directories.len(),
        codemodel.configurations[0].directory_refs.len()
    );
}

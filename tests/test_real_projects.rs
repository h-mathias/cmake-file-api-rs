use cmake_file_api::{objects, reply};
use std::path::Path;
use std::process::Command;

fn validate_cmake_file_api<P: AsRef<Path>>(build_dir: P) {
    // Test that the API is available
    assert!(reply::is_available(&build_dir));

    // Test that the index_file function returns the index file
    assert!(reply::index_file(&build_dir)
        .expect("index file should be available")
        .is_file());

    // Test that the CMakeFileApi::from_build_dir function returns the CMakeFileApi object
    let reader = reply::Reader::from_build_dir(&build_dir).expect("CMakeFileApi should be created");

    // Test that the CMakeFileApi object can be used to get the configure log
    assert!(
        reader.has_object::<cmake_file_api::objects::CMakeFilesV1>(),
        "configure log should be available"
    );

    // Test that the CMakeFileApi object can be used to get the cache
    assert!(
        reader.has_object::<cmake_file_api::objects::CacheV2>(),
        "cache should be available"
    );

    // Test that the CMakeFileApi object can be used to get the toolchains
    assert!(
        reader.has_object::<cmake_file_api::objects::ToolchainsV1>(),
        "toolchains should be available"
    );

    // Test that the CMakeFileApi object can be used to get the cmake files
    assert!(
        reader.has_object::<cmake_file_api::objects::CMakeFilesV1>(),
        "cmake files should be available"
    );

    // Test that the CMakeFileApi object can be used to get the codemodel
    let codemodel: objects::CodeModelV2 = reader.read_object().expect("codemodel should be available");
    for config in &codemodel.configurations {
        for target in &config.targets {
            println!("{}", target.name);
            println!("{:#?}", target.sources);
        }
    }
}

#[test]
#[ignore]
fn test_llvm() {
    let tmp_dir = tempdir::TempDir::new("llvm").unwrap();
    let checkout_dir = tmp_dir.path();
    let cmake_source_dir = checkout_dir.join("llvm");
    let build_dir = checkout_dir.join("build");

    // clone llvm
    Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg("--branch")
        .arg("llvmorg-18.1.8")
        .arg("https://github.com/llvm/llvm-project.git")
        .arg(checkout_dir)
        .output()
        .expect("failed to clone llvm");

    // create build directory
    std::fs::create_dir(&build_dir).expect("failed to create build directory");

    // write query
    cmake_file_api::query::Writer::default()
        .request_object::<objects::CodeModelV2>()
        .request_object::<objects::ConfigureLogV1>()
        .request_object::<objects::CacheV2>()
        .request_object::<objects::ToolchainsV1>()
        .request_object::<objects::CMakeFilesV1>()
        .write_stateless(&build_dir)
        .expect("failed to write query");

    // run cmake
    assert!(Command::new("cmake")
        .arg("-S")
        .arg(cmake_source_dir)
        .arg("-B")
        .arg(&build_dir)
        .arg("-G")
        .arg("Ninja")
        .arg("-DCMAKE_BUILD_TYPE=Debug")
        .status()
        .expect("failed to run cmake")
        .success());

    // test api
    validate_cmake_file_api(&build_dir);
}

#[test]
#[ignore]
fn test_abseil() {
    let tmp_dir = tempdir::TempDir::new("abseil").unwrap();
    let checkout_dir = tmp_dir.path();
    let cmake_source_dir = checkout_dir.join("abseil-cpp");
    let build_dir = checkout_dir.join("build");

    // clone abseil
    Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg("--branch")
        .arg("20240722.0")
        .arg("https://github.com/abseil/abseil-cpp.git")
        .arg(&cmake_source_dir)
        .output()
        .expect("failed to clone abseil");

    // create build directory
    std::fs::create_dir(&build_dir).expect("failed to create build directory");

    // write query
    cmake_file_api::query::Writer::default()
        .request_all_objects()
        .write_stateless(&build_dir)
        .expect("failed to write query");

    // run cmake
    assert!(Command::new("cmake")
        .arg("-S")
        .arg(&cmake_source_dir)
        .arg("-B")
        .arg(&build_dir)
        .arg("-G")
        .arg("Ninja")
        .arg("-DCMAKE_BUILD_TYPE=Debug")
        .status()
        .expect("failed to run cmake")
        .success());

    // test api
    validate_cmake_file_api(&build_dir);
}

#[test]
#[ignore]
fn test_googletest() {
    let tmp_dir = tempdir::TempDir::new("googletest").unwrap();
    let checkout_dir = tmp_dir.path();
    let cmake_source_dir = checkout_dir.join("googletest");
    let build_dir = checkout_dir.join("build");

    // clone googletest
    Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg("--branch")
        .arg("v1.15.2")
        .arg("https://github.com/google/googletest.git")
        .arg(&cmake_source_dir)
        .output()
        .expect("failed to clone googletest");

    // create build directory
    std::fs::create_dir(&build_dir).expect("failed to create build directory");

    // write query
    cmake_file_api::query::Writer::default()
        .request_all_objects()
        .write_stateless(&build_dir)
        .expect("failed to write query");

    // run cmake
    assert!(Command::new("cmake")
        .arg("-S")
        .arg(&cmake_source_dir)
        .arg("-B")
        .arg(&build_dir)
        .arg("-G")
        .arg("Ninja")
        .arg("-DCMAKE_BUILD_TYPE=Debug")
        .status()
        .expect("failed to run cmake")
        .success());

    // test api
    validate_cmake_file_api(&build_dir);
}

use cmake_file_api::{objects, reply};
use std::path::PathBuf;
use std::process::ExitCode;

/// This example demonstrates how to use `cmake_file_api` to get information about a `CMake` project.
fn main() -> Result<ExitCode, Box<dyn std::error::Error>> {
    // source directory from argument or default to examples/cpp
    let source_dir = if let Some(arg) = std::env::args().nth(1) {
        PathBuf::from(arg)
    } else {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("cpp")
    };

    // build directory from argument or default to examples/cpp/build
    let build_dir = if let Some(arg) = std::env::args().nth(2) {
        PathBuf::from(arg)
    } else {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("cpp")
            .join("build")
    };

    if !source_dir.exists() {
        println!("Source directory does not exist: {}", source_dir.display());
        return Ok(ExitCode::FAILURE);
    }

    if !build_dir.exists() {
        println!("Creating build directory: {}", build_dir.display());
        std::fs::create_dir_all(&build_dir).unwrap();
    }

    if !reply::is_available(&build_dir) {
        println!("CMake File API is not available, generating it now");

        println!("Writing CMake File API query");
        cmake_file_api::query::Writer::default()
            .request_object::<objects::CodeModelV2>()
            .write_stateless(&build_dir)?;

        // run cmake
        println!("Running cmake");
        assert!(std::process::Command::new("cmake")
            .arg("-S")
            .arg(&source_dir)
            .arg("-B")
            .arg(&build_dir)
            .status()?
            .success());
    }

    // load the file api
    println!("Loading CMake File API");
    let reader = reply::Reader::from_build_dir(&build_dir)?;

    // get the codemodel
    let codemodel: objects::CodeModelV2 = reader.read_object()?;

    // print all source files and their compile flags
    for target in &codemodel.configurations[0].targets {
        if target.sources.is_empty() {
            continue;
        }

        println!("Source files for target {}:", target.name);
        for source in &target.sources {
            println!("  {}", source.path.display());

            if let Some(compile_group) = &source
                .compile_group_index
                .and_then(|i| target.compile_groups.get(i))
            {
                println!("    Includes:");
                for include in &compile_group.includes {
                    println!("      * {}", include.path.display());
                }

                println!("    Defines:");
                for define in compile_group.defines() {
                    println!("      * {define}");
                }

                println!("    Flags:");
                for flag in compile_group.flags() {
                    println!("      * {flag}");
                }
            }
        }
    }

    Ok(ExitCode::SUCCESS)
}

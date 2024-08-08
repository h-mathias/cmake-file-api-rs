![GitHub Workflow Status](https://github.com/h-mathias/cmake-file-api-rs/actions/workflows/ci.yaml/badge.svg)

cmake-file-api-rs
=======

Library for interacting with the [cmake-file-api](https://cmake.org/cmake/help/latest/manual/cmake-file-api.7.html)
- Writing queries
- Reading replies

Dual-licensed under MIT or the [UNLICENSE](https://unlicense.org).

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
cmake-file-api = "0.1"
```

### Example

Build query and parse cmake-file-api:

```rust
use cmake_file_api::{objects, query, reply};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source_dir = std::path::Path::new("path/to/source/dir");
    let build_dir = std::path::Path::new("path/to/build/dir");

    // write query for codemodel-v2
    query::Writer::default()
        .request_object::<objects::CodeModelV2>()
        .write_stateless(build_dir)?;

    // run cmake
    assert!(std::process::Command::new("cmake")
        .arg("-S")
        .arg(source_dir)
        .arg("-B")
        .arg(build_dir)
        .status()?
        .success());

    // parse cmake-file-api
    let reader = reply::Reader::from_build_dir(build_dir)?;

    // read and print codemodel-v2
    let codemodel: objects::CodeModelV2 = reader.read_object()?;
    codemodel.configurations.iter().for_each(|config| {
        config.targets.iter().for_each(|target| {
            println!("{}", target.name);
            println!("{:#?}", target.sources);
        });
    });

    Ok(())
}
```

# CMake-file-api
The `cmake-file-api` is the predecessor of the `cmaker-server` and was introduced in `CMake` 3.14. It provides a rich interface for querying configuration and project information.
The API is versioned, and the current version is v1. As the name suggests, the API is based on files, which are written to disk by `CMake` and read by client tools. `CMake` generates these files in a directory named `.cmake/api/v1` in the build directory.
The V1 API is a collection of JSON files that describe the configuration of the `CMake` project, and it always contains an `index-*.json` file which lists all available objects.
The objects are also versioned on their own, e.g. `codemodel-v2.json`. `CMake` will generate the files on demand,
and expects clients to first write queries inside `.cmake/api/v1/query` before configuration.
The query describes which objects the client is interested in. With stateful queries, the client can also provide additional client data which is available in the reply.  
The API is commonly used insides IDE's but can also be used for other tooling purposes like invoking tools which need compile flags.

# Related projects
- [python-cmake-file-api](https://github.com/madebr/python-cmake-file-api): Python bindings for the CMake File API
- [cfi-java](https://github.com/WalkerKnapp/cfi-java): Java bindings for the CMake File API 

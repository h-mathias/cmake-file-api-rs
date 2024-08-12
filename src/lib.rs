//! Library for interacting with the [cmake-file-api](https://cmake.org/cmake/help/latest/manual/cmake-file-api.7.html)
//! - Writing queries
//! - Reading replies
//!
//! # Example
//!
//! Build query and parse cmake-file-api
//!
//! ```no_run
//! # use std::error::Error;
//! # use std::path::Path;
//! use cmake_file_api::{query, reply, objects};
//!
//! #
//! # fn try_main() -> Result<(), Box<dyn Error>> {
//! #     let source_dir = Path::new(".");
//! #     let build_dir = Path::new(".");
//!
//! // generate query
//! query::Writer::default()
//!   .request_object::<objects::CodeModelV2>()
//!   .write_stateless(&build_dir)?;
//!
//! // run cmake
//! assert!(std::process::Command::new("cmake")
//!   .arg("-S")
//!   .arg(&source_dir)
//!   .arg("-B")
//!   .arg(&build_dir)
//!   .status()?
//!   .success());
//!
//! // parse cmake-file-api
//! let reader = reply::Reader::from_build_dir(build_dir)?;
//!
//! // interact with api objects
//! let codemodel: objects::CodeModelV2 = reader.read_object()?;
//! for config in &codemodel.configurations{
//!   for target in &config.targets {
//!     println!("{}", target.name);
//!     println!("{:#?}", target.sources)
//!   }
//! }
//! #   Ok(())
//! # }
//! ```
//!
//! # cmake-file-api
//! The `CMake File API` is a new feature in `CMake` 3.14 that provides a rich interface for querying `CMake's` configuration and project information.
//! As the name suggests, the API is based on files, which are written to disk by `CMake` and read by client tools.
//! `CMake` generates these files in a directory named `.cmake/api/v1` in the build directory. The API is versioned, and the current version is v1.
//! The V1 API is a collection of JSON files that describe the configuration of the `CMake` project it always contains an `index-*.json` file which lists all available objects.
//! The objects are also versioned on their own, e.g. `codemodel-v2.json`. `CMake` will generate the files on demand,
//! and expects clients to first write a query file to the query directory `.cmake/api/v1/query` before configuration step.
//! The query describes which objects the client is interested in. With stateful queries, the client can also provide additional client data which is available in the reply.
//! The API is designed to be used by tools that need to interact with `CMake` (IDE) but can also be used for other tooling purposes e.g. generate `compile_commands.json`.
//!
//!
//!

#![forbid(unsafe_code)]
#![forbid(clippy::panic)]
#![forbid(clippy::shadow_reuse)]
#![forbid(clippy::shadow_unrelated)]
#![forbid(clippy::exhaustive_enums)]

pub mod index;
pub mod objects;
pub mod query;
pub mod reply;

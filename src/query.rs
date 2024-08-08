use crate::objects;
use crate::objects::ObjectKind;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Errors for writing queries
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum WriterError {
    #[error("IO error: {0}")]
    IO(io::Error),

    #[error("Failed to serialize query: {0}")]
    Parse(serde_json::Error),

    #[error("Client name not set")]
    ClientNameNotSet,
}

impl From<io::Error> for WriterError {
    fn from(err: io::Error) -> Self {
        WriterError::IO(err)
    }
}

impl From<serde_json::Error> for WriterError {
    fn from(err: serde_json::Error) -> Self {
        WriterError::Parse(err)
    }
}

/// Write queries for cmake-file-api.
///
/// # Example
///
/// ```no_run
/// use cmake_file_api::{query, objects};
/// # let build_dir = std::path::Path::new(".");
///
/// query::Writer::default()
///   .request_object::<objects::CodeModelV2>()
///   .write_stateless(&build_dir)
///   .expect("Failed to write query");
/// ```
#[derive(Default)]
pub struct Writer {
    query: Query,
    client_name: Option<String>,
}

impl Writer {
    /// Request cmake-file-api object
    pub fn request_object<T: objects::Object>(&mut self) -> &mut Self {
        self.query.requests.push(Request {
            kind: T::kind(),
            version: OptionalVersion {
                major: T::major(),
                minor: None,
            },
        });
        self
    }

    /// Request cmake-file-api object with exact version (minor version only used for stateful queries)
    pub fn add_request_exact<T: objects::Object>(&mut self, minor: u32) -> &mut Self {
        self.query.requests.push(Request {
            kind: T::kind(),
            version: OptionalVersion {
                major: T::major(),
                minor: Some(minor),
            },
        });
        self
    }

    /// Helper function to request all objects
    pub fn request_all_objects(&mut self) -> &mut Self {
        self.request_object::<objects::CodeModelV2>()
            .request_object::<objects::ConfigureLogV1>()
            .request_object::<objects::CacheV2>()
            .request_object::<objects::ToolchainsV1>()
            .request_object::<objects::CMakeFilesV1>()
    }

    /// Set client data
    /// Only used for stateful queries
    ///
    /// # Arguments
    ///
    /// * `client_name` - Client name
    /// * `client_data` - Client data (JSON)
    pub fn set_client(&mut self, client_name: &str, client_data: serde_json::Value) -> &mut Self {
        self.query.client = Some(client_data);
        self.client_name = Some(client_name.to_owned());
        self
    }

    /// Write stateless query
    /// For every object requested, a file is created in the query folder e.g. `<build_dir>/.cmake/api/v1/query/codemodel-v2`
    ///
    /// # Errors
    ///
    /// Returns an error if the query folder could not be created
    /// Returns an error if the query file could not be written
    pub fn write_stateless<P: AsRef<Path>>(&self, build_dir: P) -> Result<(), WriterError> {
        let query_dir = dir(build_dir);

        // create query folder
        fs::create_dir_all(&query_dir)?;

        for obj in &self.query.requests {
            let query_file =
                query_dir.join(format!("{}-v{}", obj.kind.as_str(), obj.version.major));
            fs::write(&query_file, "")?;
        }

        Ok(())
    }

    /// Write stateful query
    /// A single `<client-name>/query.json` file is created in the query folder containing all requested objects and when set the client data
    ///
    /// # Arguments
    ///
    /// * `build_dir` - Build directory
    ///
    /// # Errors
    ///
    /// Returns an error if the query file could not be written
    pub fn write_stateful<P: AsRef<Path>>(&self, build_dir: P) -> Result<(), WriterError> {
        let query_dir = dir(build_dir);
        let client_dir = query_dir.join(
            self.client_name
                .as_ref()
                .ok_or(WriterError::ClientNameNotSet)?,
        );

        // create query folder
        fs::create_dir_all(&client_dir)?;

        // create query file
        let query_file = client_dir.join("query.json");
        let query = serde_json::to_string(&self.query)?;
        fs::write(query_file, query)?;

        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct OptionalVersion {
    major: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    minor: Option<u32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Request {
    kind: ObjectKind,
    version: OptionalVersion,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Query {
    requests: Vec<Request>,
    client: Option<serde_json::Value>,
}
/// Get query folder for a given build directory
pub fn dir<P: AsRef<Path>>(build_dir: P) -> PathBuf {
    Path::new(build_dir.as_ref())
        .join(".cmake")
        .join("api")
        .join("v1")
        .join("query")
}

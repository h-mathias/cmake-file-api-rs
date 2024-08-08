use crate::{index, objects, reply};
use serde::de::DeserializeOwned;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Errors for reading replies
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum ReaderError {
    #[error("IO error: {0}")]
    IO(io::Error),

    #[error("Failed to deserialize reply: {0}")]
    Parse(serde_json::Error),

    #[error("cmake-file-api is not generated for this build directory")]
    FileApiNotGenerated,

    #[error("failed to find object")]
    ObjectNotFound,
}

impl From<io::Error> for ReaderError {
    fn from(err: io::Error) -> Self {
        ReaderError::IO(err)
    }
}

impl From<serde_json::Error> for ReaderError {
    fn from(err: serde_json::Error) -> Self {
        ReaderError::Parse(err)
    }
}

/// Reader for cmake-file-api replies
///
/// Example:
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
pub struct Reader {
    /// Build directory
    build_dir: PathBuf,

    /// Index file
    index: index::Index,
}

impl Reader {
    /// Create a new reader from a build directory
    ///
    /// # Errors
    ///
    /// `ReaderError::FileApiNotGenerated`: if the cmake-file-api is not generated for the build directory
    /// `ReaderError::IO`: if an IO error occurs while reading the index file
    /// `ReaderError::Parse`: if an error occurs while parsing the index file
    pub fn from_build_dir<P: AsRef<Path>>(build_dir: P) -> Result<Self, ReaderError> {
        let index_file = index_file(build_dir.as_ref()).ok_or(ReaderError::FileApiNotGenerated)?;
        let index = Reader::parse_reply(index_file)?;
        Ok(Reader {
            build_dir: build_dir.as_ref().to_path_buf(),
            index,
        })
    }

    #[must_use]
    pub fn build_dir(&self) -> &Path {
        &self.build_dir
    }

    #[must_use]
    pub fn index(&self) -> &index::Index {
        &self.index
    }

    #[must_use]
    pub fn has_object<T: objects::Object>(&self) -> bool {
        self.find_object(T::kind(), T::major()).is_some()
    }

    /// read object
    ///
    /// # Errors
    ///
    /// `ReaderError::ObjectNotFound`: if the index file does not contain the requested object
    /// `ReaderError::IO`: if an IO error occurs while reading the object file
    /// `ReaderError::Parse`: if an error occurs while parsing the object file
    pub fn read_object<T: objects::Object + DeserializeOwned>(&self) -> Result<T, ReaderError> {
        let reply_reference = self
            .find_object(T::kind(), T::major())
            .ok_or(ReaderError::ObjectNotFound)?;
        let reply_file = reply::dir(&self.build_dir).join(&reply_reference.json_file);
        let mut object: T = Reader::parse_reply(reply_file)?;

        object.resolve_references(self)?;

        Ok(object)
    }

    /// Parse a reply file into a given object type
    pub(crate) fn parse_reply<P: AsRef<Path>, Object: DeserializeOwned>(
        reply_file: P,
    ) -> Result<Object, ReaderError> {
        let content = fs::read_to_string(&reply_file)?;

        let object = serde_json::from_str(content.as_str())?;

        Ok(object)
    }

    /// Find an object in the index file
    fn find_object(
        &self,
        kind: objects::ObjectKind,
        major: u32,
    ) -> Option<&index::ReplyFileReference> {
        self.index
            .objects
            .iter()
            .find(|obj| obj.kind == kind && obj.version.major == major)
    }
}

/// Get cmake-file-api reply path for a given build directory
pub fn dir<P: AsRef<Path>>(build_dir: P) -> PathBuf {
    Path::new(build_dir.as_ref())
        .join(".cmake")
        .join("api")
        .join("v1")
        .join("reply")
}

/// Get cmake-file-api index file path for a given build directory
pub fn index_file<P: AsRef<Path>>(build_dir: P) -> Option<PathBuf> {
    let reply_dir = dir(build_dir);

    if !reply_dir.exists() {
        return None;
    }

    // find json file with 'index-' prefix
    fs::read_dir(&reply_dir).ok()?.find_map(|entry| {
        let path = entry.ok()?.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(OsStr::to_str) {
                if file_name.starts_with("index-")
                    && path
                        .extension()
                        .map_or(false, |ext| ext.eq_ignore_ascii_case("json"))
                {
                    return Some(path);
                }
            }
        }
        None
    })
}

/// Check if cmake-file-api is available for a given build directory
pub fn is_available<P: AsRef<Path>>(build_dir: P) -> bool {
    index_file(build_dir).is_some()
}

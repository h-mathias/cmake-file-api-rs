use serde::{Deserialize, Serialize};

pub mod cache_v2;
pub mod cmake_files_v1;
pub mod codemodel_v2;
pub mod configure_log_v1;
pub mod toolchains_v1;

pub use cache_v2::Cache as CacheV2;
pub use cmake_files_v1::CMakeFiles as CMakeFilesV1;
pub use codemodel_v2::CodeModel as CodeModelV2;
pub use configure_log_v1::ConfigureLog as ConfigureLogV1;
pub use toolchains_v1::Toolchains as ToolchainsV1;

use crate::reply;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MajorMinor {
    pub major: u32,
    pub minor: u32,
}

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum ObjectKind {
    #[default]
    #[serde(rename = "codemodel")]
    CodeModel,
    #[serde(rename = "toolchains")]
    Toolchains,
    #[serde(rename = "cache")]
    Cache,
    #[serde(rename = "cmakeFiles")]
    CMakeFiles,
    #[serde(rename = "configureLog")]
    ConfigureLog,
}

impl ObjectKind {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            ObjectKind::CodeModel => "codemodel",
            ObjectKind::Toolchains => "toolchains",
            ObjectKind::Cache => "cache",
            ObjectKind::CMakeFiles => "cmakeFiles",
            ObjectKind::ConfigureLog => "configureLog",
        }
    }
}

pub trait Object {
    fn kind() -> ObjectKind;
    fn major() -> u32;

    /// Resolve references in the object
    ///
    /// Some objects contain references to other json files. This method is called after the object
    /// is deserialized to resolve these references.
    /// Currently only the codemodel-v2 object has references (targets, directories) that need to be resolved.
    ///
    /// # Errors
    ///
    /// `ReaderError::IO`: if an IO error occurs while reading the object file
    /// `ReaderError::Parse`: if an error occurs while parsing the object file
    fn resolve_references(&mut self, _: &reply::Reader) -> Result<(), reply::ReaderError> {
        Ok(())
    }
}

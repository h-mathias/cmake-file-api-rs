#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::module_name_repetitions)]

use super::backtrace_graph::BacktraceGraph;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A codemodel "directory" object is referenced by a "codemodel" version 2 object's directories array.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Directory {
    /// Paths of the directory object.
    pub paths: DirectoryPaths,

    /// A "codemodel" version 2 "backtrace graph" whose nodes are referenced from backtrace members elsewhere in this "directory" object.
    pub backtrace_graph: BacktraceGraph,

    /// Entries corresponding to install() rules
    pub installers: Vec<Installer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DirectoryPaths {
    /// A string specifying the path to the source directory, represented with forward slashes.
    /// If the directory is inside the top-level source directory then the path is specified
    /// relative to that directory (with . for the top-level source directory itself).
    /// Otherwise, the path is absolute.
    pub build: PathBuf,

    /// A string specifying the path to the build directory, represented with forward slashes.
    /// If the directory is inside the top-level build directory then the path is specified
    /// relative to that directory (with . for the top-level build directory itself).
    /// Otherwise, the path is absolute.
    pub source: PathBuf,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Installer {
    /// A string specifying the component selected by the corresponding to install() command invocation.
    pub component: String,

    /// Optional member that is present for specific type values below. The value is a string specifying the installation destination path.
    /// The path may be absolute or relative to the installation prefix.
    pub destination: Option<String>,

    /// Optional member that is present for specific installer_type values below.
    #[serde(default)]
    pub paths: Vec<InstallPath>,

    /// A string specifying the type of installation rule. The value is one of the following, with some variants providing additional members:
    /// * file: An install(FILES) or install(PROGRAMS) call. The destination and paths members are populated, with paths under the top-level source directory expressed relative to it. The isOptional member may exist. This type has no additional members.
    /// * directory: An install(DIRECTORY) call. The destination and paths members are populated, with paths under the top-level source directory expressed relative to it. The isOptional member may exist. This type has no additional members.
    /// * target: An install(TARGETS) call. The destination and paths members are populated, with paths under the top-level build directory expressed relative to it. The isOptional member may exist. This type has additional members targetId, targetIndex, targetIsImportLibrary, and targetInstallNamelink.
    /// * export: An install(EXPORT) call. The destination and paths members are populated, with paths under the top-level build directory expressed relative to it. The paths entries refer to files generated automatically by CMake for installation, and their actual values are considered private implementation details. This type has additional members exportName and exportTargets.
    /// * script: An install(SCRIPT) call. This type has additional member scriptFile.
    /// * code: An install(CODE) call. This type has no additional members.
    /// * importedRuntimeArtifacts: An install(IMPORTED_RUNTIME_ARTIFACTS) call. The destination member is populated. The isOptional member may exist. This type has no additional members.
    /// * runtimeDependencySet: An install(RUNTIME_DEPENDENCY_SET) call or an install(TARGETS) call with RUNTIME_DEPENDENCIES. The destination member is populated. This type has additional members runtimeDependencySetName and runtimeDependencySetType.
    /// * fileSet: An install(TARGETS) call with FILE_SET. The destination and paths members are populated. The isOptional member may exist. This type has additional members fileSetName, fileSetType, fileSetDirectories, and fileSetTarget.
    /// This type was added in codemodel version 2.4.
    #[serde(rename = "type")]
    pub installer_type: String,

    /// True when install() is called with the EXCLUDE_FROM_ALL option.
    #[serde(default)]
    pub is_exclude_from_all: bool,

    /// True when install(SCRIPT|CODE) is called with the ALL_COMPONENTS option.
    #[serde(default)]
    pub is_for_all_components: bool,

    /// True when install() is called with the OPTIONAL option.
    /// This is allowed when type is file, directory, or target.
    #[serde(default)]
    pub is_optional: bool,

    /// Optional member that is present when type is target. The value is a string uniquely identifying the target to be installed.
    /// This matches the id member of the target in the main "codemodel" object's targets array.
    pub target_id: Option<String>,

    /// Optional member that is present when type is target.
    /// The value is an unsigned integer 0-based index into the main "codemodel" object's targets array for the target to be installed.
    pub target_index: Option<usize>,

    /// True when type is target and the installer is for a Windows DLL import library file or for an AIX linker import file.
    #[serde(default)]
    pub target_is_import_library: bool,

    /// Optional member that is present when type is target and the installer corresponds to a target that may use symbolic links
    /// to implement the VERSION and SOVERSION target properties.
    /// The value is a string indicating how the installer is supposed to handle the symlinks:
    /// <b>skip</b> means the installer should skip the symlinks and install only the real file
    /// <b>only</b> means the installer should install only the symlinks and not the real file.
    /// In all cases the paths member lists what it actually installs.
    pub target_install_namelink: Option<String>,

    /// Optional member that is present when type is export.
    /// The value is a string specifying the name of the export.
    pub export_name: Option<String>,

    /// Optional member that is present when <b>type</b> equals export.
    #[serde(default)]
    pub export_targets: Vec<TargetIdAndIndex>,

    /// Optional member that is present when type is runtimeDependencySet and the installer was created by an install(RUNTIME_DEPENDENCY_SET) call.
    /// The value is a string specifying the name of the runtime dependency set that was installed.
    pub runtime_dependency_set_name: Option<String>,

    /// Optional member that is present when type is runtimeDependencySet.
    /// The value is a string with one of the following values:
    /// * library: Indicates that this installer installs dependencies that are not macOS frameworks.
    /// * framework: Indicates that this installer installs dependencies that are macOS frameworks.
    pub runtime_dependency_set_type: Option<String>,

    /// Optional member that is present when type is fileSet. The value is a string with the name of the file set.
    /// This field was added in codemodel version 2.4.
    pub file_set_name: Option<String>,

    /// Optional member that is present when type is fileSet. The value is a string with the type of the file set.
    /// This field was added in codemodel version 2.4.
    pub file_set_type: Option<String>,

    /// Optional member that is present when type is fileSet.
    /// The value is a list of strings with the file set's base directories (determined by genex-evaluation of HEADER_DIRS or `HEADER_DIRS_<NAME>`).
    /// This field was added in codemodel version 2.4.
    #[serde(default)]
    pub file_set_directories: Vec<String>,

    /// Optional member that is present when type is fileSet.
    /// This field was added in codemodel version 2.4.
    pub file_set_target: Option<TargetIdAndIndex>,

    /// Optional member that is present when type is script.
    /// The value is a string specifying the path to the script file on disk, represented with forward slashes.
    /// If the file is inside the top-level source directory then the path is specified relative to that directory.
    /// Otherwise, the path is absolute.
    pub script_file: Option<PathBuf>,

    /// Optional member that is present when a CMake language backtrace to the install() or other command invocation
    /// that added this installer is available.
    /// The value is an unsigned integer 0-based index into the backtraceGraph member's nodes array.
    pub backtrace: Option<usize>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TargetIdAndIndex {
    /// A string uniquely identifying the target.
    /// This matches the id member of the target in the main "codemodel" object's targets array.
    pub id: String,

    /// An unsigned integer 0-based index into the main "codemodel" object's targets array for the target.
    pub index: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FromToPaths {
    /// Path from which a file or directory is to be installed.
    pub from: PathBuf,

    /// Path to which the file or directory is to be installed under the destination.
    pub to: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum InstallPath {
    /// A string specifying the path from which a file or directory is to be installed.
    /// The portion of the path not preceded by a / also specifies the path (name) to which the file or directory is to be installed
    /// under the destination.
    PathCombination(String),

    /// A pair of paths specifying the path from which a file or directory is to be installed and
    /// the path to which the file or directory is to be installed under the destination.
    FromTo(FromToPaths),
}

#[cfg(test)]
mod tests {
    use crate::objects::codemodel_v2::directory::*;
    use serde_json::json;
    use std::path::PathBuf;

    #[test]
    fn test_directory() {
        let json = json!({
            "backtraceGraph" :
            {
                "commands" : [],
                "files" : [],
                "nodes" : []
            },
            "installers" : [],
            "paths" :
            {
                "build" : ".",
                "source" : "."
            }
        });

        let dir = serde_json::from_value::<Directory>(json).unwrap();
        assert_eq!(
            dir,
            Directory {
                backtrace_graph: BacktraceGraph {
                    ..Default::default()
                },
                installers: vec![],
                paths: DirectoryPaths {
                    build: PathBuf::from("."),
                    source: PathBuf::from(".")
                }
            }
        );
    }
}

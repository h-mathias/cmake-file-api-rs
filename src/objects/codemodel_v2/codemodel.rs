#![allow(clippy::module_name_repetitions)]

use crate::objects::codemodel_v2::{Directory, Target};
use crate::objects::{MajorMinor, Object, ObjectKind};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::reply;

/// The codemodel object kind describes the build system structure as modeled by `CMake`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CodeModel {
    /// Kind of the codemodel object.
    pub kind: ObjectKind,

    /// Version of the codemodel object.
    pub version: MajorMinor,

    /// Paths of the codemodel object.
    pub paths: CodemodelPaths,

    /// Available build configurations.
    /// On single-configuration generators there is one entry for the value of the CMAKE_BUILD_TYPE variable.
    /// For multi-configuration generators there is an entry for each configuration listed in the CMAKE_CONFIGURATION_TYPES variable.
    pub configurations: Vec<Configuration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CodemodelPaths {
    /// Absolute path to the top-level source directory, represented with forward slashes.
    pub build: PathBuf,

    /// Absolute path to the top-level build directory, represented with forward slashes.
    pub source: PathBuf,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Configuration {
    /// A string specifying the name of the configuration, e.g. Debug.
    pub name: String,

    /// Top-level project and subprojects defined in the build system.
    /// Each (sub-)project corresponds to a source directory whose CMakeLists.txt file calls the project() command with a project name different from its parent directory.
    /// The first entry corresponds to the top-level project.
    pub projects: Vec<Project>,

    /// Build system directory info whose source directory contains a CMakeLists.txt file.
    /// The first entry corresponds to the top-level directory
    #[serde(rename = "directories")]
    pub directory_refs: Vec<DirectoryReference>,

    /// Build system targets.
    /// Such targets are created by calls to add_executable(), add_library(), and add_custom_target(),
    /// excluding imported targets and interface libraries (which do not generate any build rules).
    #[serde(rename = "targets")]
    pub target_refs: Vec<TargetReference>,

    /// The following members are not part of the JSON file.
    /// They are used to store the actual objects that the references point to.

    /// Directory objects.
    /// The position in the vector corresponds to the index in the directory_refs vector.
    #[serde(skip)]
    pub directories: Vec<Directory>,

    /// Target objects.
    /// The position in the vector corresponds to the index in the target_refs vector.
    #[serde(skip)]
    pub targets: Vec<Target>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DirectoryReference {
    /// Path to the source directory, represented with forward slashes.
    /// If the directory is inside the top-level source directory then the path is specified
    /// relative to that directory (with . for the top-level source directory itself).
    /// Otherwise, the path is absolute.
    pub source: PathBuf,

    /// Path to the build directory, represented with forward slashes.
    /// If the directory is inside the top-level build directory then the path is specified
    /// relative to that directory (with . for the top-level build directory itself).
    /// Otherwise, the path is absolute.
    pub build: PathBuf,

    /// Optional member that is present when the directory is not top-level.
    /// The value is an unsigned integer 0-based index of another entry in the main directories array
    /// that corresponds to the parent directory that added this directory as a subdirectory.
    pub parent_index: Option<usize>,

    /// Optional member that is present when the directory has subdirectories.
    /// Each entry corresponding to child directory created by the add_subdirectory() or subdirs() command.
    /// Each entry is an unsigned integer 0-based index of another entry in the main directories array.
    #[serde(default)]
    pub child_indexes: Vec<usize>,

    /// An unsigned integer 0-based index into the main projects array indicating the build system project to which the directory belongs.
    pub project_index: usize,

    /// Optional member that is present when the directory itself has targets, excluding those belonging to subdirectories.
    /// Each entry corresponding to the targets.
    /// Each entry is an unsigned integer 0-based index into the main targets array.
    #[serde(default)]
    pub target_indexes: Vec<usize>,

    /// Optional member present when a minimum required version of CMake is known for the directory.
    /// This is the `<min>` version given to the most local call to the cmake_minimum_required(VERSION) command in the directory itself or
    /// one of its ancestors.
    #[serde(rename = "minimumCMakeVersion")]
    pub minimum_cmake_version: Option<MinimumCmakeVersion>,

    /// True when the directory or one of its subdirectories contains any install() rules, i.e. whether a make install or equivalent rule is available.
    #[serde(default)]
    pub has_install_rule: bool,

    /// Path relative to the codemodel file to another JSON file containing a "codemodel" version 2 "directory" object.
    pub json_file: PathBuf,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MinimumCmakeVersion {
    /// A string specifying the minimum required version in the format
    /// \<major\>.\<minor\>.\[\<patch\>\[.\<tweak\>]]\[\<suffix\>]
    /// Each component is an unsigned integer and the suffix may be an arbitrary string.
    #[serde(rename = "string")]
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Project {
    /// A string specifying the name given to the project() command.
    pub name: String,

    /// Optional member that is present when the project is not top-level.
    /// The value is an unsigned integer 0-based index of another entry in the main projects array that corresponds to the parent project
    /// that added this project as a subproject.
    pub parent_index: Option<usize>,

    /// Optional member that is present when the project has subprojects.
    /// Entries corresponding to the subprojects.
    /// Each entry is an unsigned integer 0-based index of another entry in the main projects array.
    #[serde(default)]
    pub child_indexes: Vec<usize>,

    /// Entries corresponding to build system directories that are part of the project.
    /// The first entry corresponds to the top-level directory of the project.
    /// Each entry is an unsigned integer 0-based index into the main directories array.
    pub directory_indexes: Vec<usize>,

    /// Optional member that is present when the project itself has targets, excluding those belonging to subprojects.
    /// Entries corresponding to the targets.
    /// Each entry is an unsigned integer 0-based index into the main targets array.
    #[serde(default)]
    pub target_indexes: Vec<usize>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TargetReference {
    /// A string specifying the target name.
    pub name: String,

    /// A string uniquely identifying the target.
    /// This matches the id field in the file referenced by jsonFile.
    pub id: String,

    /// An unsigned integer 0-based index into the main directories array indicating
    /// the build system directory in which the target is defined.
    pub directory_index: usize,

    /// An unsigned integer 0-based index into the main projects array indicating the
    /// build system project in which the target is defined.
    pub project_index: usize,

    /// Path relative to the codemodel file to another JSON file containing a "codemodel" version 2 "target" object.
    pub json_file: PathBuf,
}

impl Object for CodeModel {
    fn kind() -> ObjectKind {
        ObjectKind::CodeModel
    }

    fn major() -> u32 {
        2
    }

    fn resolve_references(&mut self, reader: &reply::Reader) -> Result<(), reply::ReaderError> {
        let reply_dir = reply::dir(reader.build_dir());

        // resolve targets and directories references
        for config in &mut self.configurations {
            for target_ref in &config.target_refs {
                config
                    .targets
                    .push(reply::Reader::parse_reply(reply_dir.join(&target_ref.json_file))?);
            }

            for directory_ref in &config.directory_refs {
                config.directories.push(reply::Reader::parse_reply(
                    reply_dir.join(&directory_ref.json_file),
                )?);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::objects;
    use crate::objects::codemodel_v2::*;
    use crate::objects::MajorMinor;
    use serde_json::json;
    use std::path::PathBuf;

    #[test]
    fn test_model() {
        let json = json!({
          "kind": "codemodel",
          "version": { "major": 2, "minor": 6 },
          "paths": {
            "source": "/path/to/top-level-source-dir",
            "build": "/path/to/top-level-build-dir"
          },
          "configurations": [
            {
              "name": "Debug",
              "directories": [
                {
                  "source": ".",
                  "build": ".",
                  "childIndexes": [ 1 ],
                  "projectIndex": 0,
                  "targetIndexes": [ 0 ],
                  "hasInstallRule": true,
                  "minimumCMakeVersion": {
                    "string": "3.14"
                  },
                  "jsonFile": "<file>"
                },
                {
                  "source": "sub",
                  "build": "sub",
                  "parentIndex": 0,
                  "projectIndex": 0,
                  "targetIndexes": [ 1 ],
                  "minimumCMakeVersion": {
                    "string": "3.14"
                  },
                  "jsonFile": "<file>"
                }
              ],
              "projects": [
                {
                  "name": "MyProject",
                  "directoryIndexes": [ 0, 1 ],
                  "targetIndexes": [ 0, 1 ]
                }
              ],
              "targets": [
                {
                  "name": "MyExecutable",
                  "directoryIndex": 0,
                  "projectIndex": 0,
                  "jsonFile": "<file>",
                  "id": "0"
                },
                {
                  "name": "MyLibrary",
                  "directoryIndex": 1,
                  "projectIndex": 0,
                  "jsonFile": "<file>",
                  "id": "1"
                }
              ]
            }
          ]
        });

        let model = serde_json::from_value::<CodeModel>(json).unwrap();
        assert_eq!(model.kind, objects::ObjectKind::CodeModel);
        assert_eq!(model.version, MajorMinor { major: 2, minor: 6 });
        assert_eq!(
            model.paths,
            CodemodelPaths {
                source: "/path/to/top-level-source-dir".into(),
                build: "/path/to/top-level-build-dir".into()
            }
        );
        assert_eq!(model.configurations.len(), 1);
        assert_eq!(model.configurations[0].name, "Debug");
        assert_eq!(model.configurations[0].directory_refs.len(), 2);
        assert_eq!(
            model.configurations[0].directory_refs[0].source,
            PathBuf::from(".")
        );
        assert_eq!(model.configurations[0].projects.len(), 1);
        assert_eq!(model.configurations[0].projects[0].name, "MyProject");
        assert_eq!(model.configurations[0].target_refs.len(), 2);
        assert_eq!(model.configurations[0].target_refs[0].name, "MyExecutable");
    }
}

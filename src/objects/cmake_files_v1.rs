use crate::objects::{MajorMinor, Object, ObjectKind};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The cmakeFiles object kind lists files used by `CMake` while configuring and generating the build system.
/// These include the CMakeLists.txt files as well as included .cmake files.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CMakeFiles {
    /// Kind of the CMakeFiles object.
    pub kind: ObjectKind,

    /// Version of the CMakeFiles object.
    pub version: MajorMinor,

    /// Paths of the CMakeFiles object.
    pub paths: Paths,

    /// Input file used by CMake when configuring and generating the build system.
    pub inputs: Vec<Input>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Paths {
    /// Absolute path to the top-level source directory, represented with forward slashes.
    pub build: PathBuf,

    /// Absolute path to the top-level build directory, represented with forward slashes.
    pub source: PathBuf,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Input {
    /// path to an input file to CMake, represented with forward slashes.
    /// If the file is inside the top-level source directory then the path is specified relative to that directory.
    /// Otherwise, the path is absolute.
    pub path: PathBuf,

    /// True if the path specifies a file that is under the top-level build directory and the build is out-of-source.
    #[serde(default)]
    pub is_generated: bool,

    /// True if the path specifies a file that is not under the top-level source or build directories.
    #[serde(default)]
    pub is_external: bool,

    /// True if the path specifies a file in the CMake installation.
    #[serde(default, rename = "isCMake")]
    pub is_cmake: bool,
}

impl Object for CMakeFiles {
    fn kind() -> ObjectKind {
        ObjectKind::CMakeFiles
    }

    fn major() -> u32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::cmake_files_v1::*;
    use serde_json::json;

    #[test]
    fn test_configure_log() {
        let json = json!({
          "kind": "cmakeFiles",
          "version": { "major": 1, "minor": 0 },
          "paths": {
            "build": "/path/to/top-level-build-dir",
            "source": "/path/to/top-level-source-dir"
          },
          "inputs": [
            {
              "path": "CMakeLists.txt"
            },
            {
              "isGenerated": true,
              "path": "/path/to/top-level-build-dir/../CMakeSystem.cmake"
            },
            {
              "isExternal": true,
              "path": "/path/to/external/third-party/module.cmake"
            },
            {
              "isCMake": true,
              "isExternal": true,
              "path": "/path/to/cmake/Modules/CMakeGenericSystem.cmake"
            }
          ]
        });

        let cmake_files = serde_json::from_value::<CMakeFiles>(json).unwrap();
        assert_eq!(
            cmake_files,
            CMakeFiles {
                kind: ObjectKind::CMakeFiles,
                version: MajorMinor { major: 1, minor: 0 },
                paths: Paths {
                    build: "/path/to/top-level-build-dir".into(),
                    source: "/path/to/top-level-source-dir".into()
                },
                inputs: vec![
                    Input {
                        path: "CMakeLists.txt".into(),
                        ..Default::default()
                    },
                    Input {
                        is_generated: true,
                        path: "/path/to/top-level-build-dir/../CMakeSystem.cmake".into(),
                        ..Default::default()
                    },
                    Input {
                        is_external: true,
                        path: "/path/to/external/third-party/module.cmake".into(),
                        ..Default::default()
                    },
                    Input {
                        is_cmake: true,
                        is_external: true,
                        path: "/path/to/cmake/Modules/CMakeGenericSystem.cmake".into(),
                        ..Default::default()
                    }
                ]
            }
        );
    }
}

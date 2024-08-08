use crate::objects::{MajorMinor, Object, ObjectKind};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The toolchains object kind lists properties of the toolchains used during the build
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Toolchains {
    /// Kind of the toolchains object.
    pub kind: ObjectKind,

    /// Version of the toolchains object.
    pub version: MajorMinor,

    /// Toolchains.
    pub toolchains: Vec<Toolchain>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Toolchain {
    /// Toolchain language, like C or CXX.
    pub language: String,

    /// Compiler information.
    pub compiler: Compiler,

    /// Optional member that is present when the `CMAKE_<LANG>_SOURCE_FILE_EXTENSIONS` variable is defined for the current language.
    /// Each string holds a file extension (without the leading dot) for the language
    #[serde(default)]
    pub source_file_extensions: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Compiler {
    /// Optional member that is present when the `CMAKE_<LANG>_COMPILER` variable is defined for the current language.
    /// Holding the absolute path to the compiler.
    pub path: Option<PathBuf>,

    /// Optional member that is present when the `CMAKE_<LANG>_COMPILER_ID` variable is defined for the current language.
    /// Holding the ID (GNU, MSVC, etc.) of the compiler.
    pub id: Option<String>,

    /// Optional member that is present when the `CMAKE_<LANG>_COMPILER_VERSION` variable is defined for the current language.
    /// Holding the version of the compiler.
    pub version: Option<String>,

    /// Optional member that is present when the `CMAKE_<LANG>_COMPILER_TARGET` variable is defined for the current language.
    /// Holding the cross-compiling target of the compiler.
    pub target: Option<String>,

    /// Implicit compiler info for `CMAKE_<LANG>_IMPLICIT_*` variables.
    pub implicit: Implicit,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Implicit {
    /// Optional member that is present when the `CMAKE_<LANG>_IMPLICIT_INCLUDE_DIRECTORIES` variable is defined for the current language.
    /// Each path points to an implicit include directory for the compiler.
    #[serde(default)]
    pub include_directories: Vec<PathBuf>,

    /// Optional member that is present when the `CMAKE_<LANG>_IMPLICIT_LINK_DIRECTORIES` variable is defined for the current language.
    /// Each path points to an implicit link directory for the compiler.
    #[serde(default)]
    pub link_directories: Vec<PathBuf>,

    /// Optional member that is present when the `CMAKE_<LANG>_IMPLICIT_LINK_FRAMEWORK_DIRECTORIES` variable is defined for the current language.
    /// Each path points to an implicit link framework directory for the compiler.
    #[serde(default)]
    pub link_framework_directories: Vec<PathBuf>,

    /// Optional member that is present when the `CMAKE_<LANG>_IMPLICIT_LINK_LIBRARIES` variable is defined for the current language.
    /// Each path points to an implicit link library for the compiler.
    #[serde(default)]
    pub link_libraries: Vec<PathBuf>,
}

impl Object for Toolchains {
    fn kind() -> ObjectKind {
        ObjectKind::Toolchains
    }

    fn major() -> u32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::toolchains_v1::*;
    use serde_json::json;

    #[test]
    fn test_toolchains() {
        let json = json!({
          "kind": "toolchains",
          "version": { "major": 1, "minor": 0 },
          "toolchains": [
            {
              "language": "C",
              "compiler": {
                "path": "/usr/bin/cc",
                "id": "GNU",
                "version": "9.3.0",
                "implicit": {
                  "includeDirectories": [
                    "/usr/lib/gcc/x86_64-linux-gnu/9/include",
                    "/usr/local/include",
                    "/usr/include/x86_64-linux-gnu",
                    "/usr/include"
                  ],
                  "linkDirectories": [
                    "/usr/lib/gcc/x86_64-linux-gnu/9",
                    "/usr/lib/x86_64-linux-gnu",
                    "/usr/lib",
                    "/lib/x86_64-linux-gnu",
                    "/lib"
                  ],
                  "linkFrameworkDirectories": [],
                  "linkLibraries": [ "gcc", "gcc_s", "c", "gcc", "gcc_s" ]
                }
              },
              "sourceFileExtensions": [ "c", "m" ]
            },
            {
              "language": "CXX",
              "compiler": {
                "path": "/usr/bin/c++",
                "id": "GNU",
                "version": "9.3.0",
                "implicit": {
                  "includeDirectories": [
                    "/usr/include/c++/9",
                    "/usr/include/x86_64-linux-gnu/c++/9",
                    "/usr/include/c++/9/backward",
                    "/usr/lib/gcc/x86_64-linux-gnu/9/include",
                    "/usr/local/include",
                    "/usr/include/x86_64-linux-gnu",
                    "/usr/include"
                  ],
                  "linkDirectories": [
                    "/usr/lib/gcc/x86_64-linux-gnu/9",
                    "/usr/lib/x86_64-linux-gnu",
                    "/usr/lib",
                    "/lib/x86_64-linux-gnu",
                    "/lib"
                  ],
                  "linkFrameworkDirectories": [],
                  "linkLibraries": [
                    "stdc++", "m", "gcc_s", "gcc", "c", "gcc_s", "gcc"
                  ]
                }
              },
              "sourceFileExtensions": [
                "C", "M", "c++", "cc", "cpp", "cxx", "mm", "CPP"
              ]
            }
          ]
        });

        let toolchains = serde_json::from_value::<Toolchains>(json).unwrap();
        assert_eq!(toolchains.kind, ObjectKind::Toolchains);
        assert_eq!(toolchains.version, MajorMinor { major: 1, minor: 0 });
        assert_eq!(toolchains.toolchains.len(), 2);
        assert_eq!(toolchains.toolchains[0].language, "C");
        assert_eq!(toolchains.toolchains[1].language, "CXX");

        assert_eq!(
            toolchains.toolchains[0].compiler.id.as_ref().unwrap(),
            "GNU"
        );
        assert_eq!(
            toolchains.toolchains[1].compiler.id.as_ref().unwrap(),
            "GNU"
        );
    }
}

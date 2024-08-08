#![allow(clippy::module_name_repetitions)]
#![allow(clippy::redundant_closure_for_method_calls)]

use super::backtrace_graph::BacktraceGraph;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A codemodel "target" object is referenced by a "codemodel" version 2 object's targets array.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Target {
    /// A string specifying the logical name of the target.
    pub name: String,

    /// A string uniquely identifying the target.
    /// The format is unspecified and should not be interpreted by clients.
    pub id: String,

    /// A string specifying the type of the target.
    /// The value is one of:
    /// * EXECUTABLE
    /// * STATIC_LIBRARY
    /// * SHARED_LIBRARY
    /// * MODULE_LIBRARY
    /// * OBJECT_LIBRARY
    /// * INTERFACE_LIBRARY
    /// * UTILITY
    #[serde(rename = "type")]
    pub type_name: String,

    /// Optional member that is present when a CMake language backtrace to the command in
    /// the source code that created the target is available.
    /// The value is an unsigned integer 0-based index into the backtraceGraph member's nodes array.
    pub backtrace: Option<usize>,

    /// Optional member that is present when the FOLDER target property is set.
    pub folder: Option<Folder>,

    /// Paths to the target's build and source directories.
    pub paths: TargetPaths,

    /// Optional member that is present for executable and library targets that are linked or archived into a single primary artifact.
    /// The value is a string specifying the file name of that artifact on disk.
    pub name_on_disk: Option<String>,

    /// Optional member that is present for executable and library targets that
    /// produce artifacts on disk meant for consumption by dependents.
    /// The value is a JSON array of entries corresponding to the artifacts.
    #[serde(default)]
    pub artifacts: Vec<Artifact>,

    /// Optional member that is present with boolean value true if the target is provided by CMake's
    /// build system generator rather than by a command in the source code.
    #[serde(default)]
    pub is_generator_provided: bool,

    /// Optional member that is present when the target has an install() rule.
    pub install: Option<Install>,

    /// Optional member that is present on executable targets that have at least one launcher specified by the project.
    #[serde(default)]
    pub launchers: Vec<Launcher>,

    /// Optional member that is present for executables and shared library targets that link into a runtime binary.
    pub link: Option<Link>,

    /// Optional member that is present for static library targets.
    pub archive: Option<Archive>,

    /// Optional member that is present when the target depends on other targets.
    #[serde(default)]
    pub dependencies: Vec<Dependency>,

    /// target's file sets
    #[serde(default)]
    pub file_sets: Vec<FileSet>,

    /// target's sources
    #[serde(default)]
    pub sources: Vec<Source>,

    /// Optional member that is present when sources are grouped together by the source_group() command or by default.
    #[serde(default)]
    pub source_groups: Vec<SourceGroup>,

    /// Optional member that is present when the target has sources that compile.
    #[serde(default)]
    pub compile_groups: Vec<CompileGroup>,

    /// A "codemodel" version 2 "backtrace graph" whose nodes are referenced from backtrace members elsewhere in this "target" object.
    pub backtrace_graph: BacktraceGraph,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Folder {
    /// A string specifying the name of the target folder.
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TargetPaths {
    /// Path to the target's source directory, represented with forward slashes.
    /// If the directory is inside the top-level source directory then the path is specified
    /// relative to that directory (with . for the top-level source directory itself).
    /// Otherwise, the path is absolute.
    pub build: PathBuf,

    /// Path to the target's build directory, represented with forward slashes.
    /// If the directory is inside the top-level build directory then the path is specified
    /// relative to that directory (with . for the top-level build directory itself).
    /// Otherwise, the path is absolute.
    pub source: PathBuf,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Artifact {
    /// Path to the file on disk, represented with forward slashes.
    /// If the file is inside the top-level build directory then the path is specified
    /// relative to that directory.
    /// Otherwise, the path is absolute.
    pub path: PathBuf,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Install {
    /// installation prefix
    pub prefix: Prefix,

    /// installation destination paths
    #[serde(default)]
    pub destinations: Vec<Destination>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Prefix {
    /// Path value of CMAKE_INSTALL_PREFIX.
    pub path: PathBuf,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Destination {
    /// Path of the installation destination path.
    /// The path may be absolute or relative to the install prefix.
    pub path: PathBuf,

    /// Optional member that is present when a CMake language backtrace to the install() command invocation
    /// that specified this destination is available.
    /// The value is an unsigned integer 0-based index into the backtraceGraph member's nodes array.
    pub backtrace: Option<usize>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Launcher {
    ///  string specifying the path to the launcher on disk, represented with forward slashes.
    /// If the file is inside the top-level source directory then the path is specified relative to that directory.
    pub command: String,

    /// Optional member that is present when the launcher command has arguments preceding the executable to be launched.
    #[serde(default)]
    pub arguments: Vec<String>,

    /// A string specifying the type of launcher.
    /// The value is one of the following:
    ///  * emulator: An emulator for the target platform when cross-compiling. See the CROSSCOMPILING_EMULATOR target property.
    /// * test: A start program for the execution of tests. See the TEST_LAUNCHER target property.
    pub launcher_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Link {
    /// A string specifying the language (e.g. C, CXX, Fortran) of the toolchain is used to invoke the linker.
    pub language: String,

    /// Optional member that is present when fragments of the link command line invocation are available.
    #[serde(default)]
    pub command_fragments: Vec<CommandFragment>,

    /// True when link-time optimization (a.k.a. interprocedural optimization or link-time code generation) is enabled.
    #[serde(default)]
    pub lto: bool,

    /// Optional member that is present when the CMAKE_SYSROOT_LINK or CMAKE_SYSROOT variable is defined.
    #[serde(default)]
    pub sysroot: Option<SysRootPath>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CommandFragment {
    /// A string specifying a fragment of the link command line invocation.
    /// The value is encoded in the build system's native shell format.
    pub fragment: String,

    /// A string specifying the role of the fragment's content:
    ///  * flags: archiver flags
    pub role: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SysRootPath {
    /// Absolute path to the sysroot, represented with forward slashes.
    pub path: PathBuf,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Archive {
    /// fragments of the archiver command line invocation.
    #[serde(default)]
    pub command_fragments: Vec<CommandFragment>,

    /// True when link-time optimization (a.k.a. interprocedural optimization or link-time code generation) is enabled.
    #[serde(default)]
    pub lto: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Dependency {
    /// A string uniquely identifying the target on which this target depends.
    /// This matches the main id member of the other target.
    pub id: String,

    /// Optional member that is present when a CMake language backtrace to the add_dependencies(), target_link_libraries(),
    /// or other command invocation that created this dependency is available.
    /// The value is an unsigned integer 0-based index into the backtraceGraph member's nodes array.
    pub backtrace: Option<usize>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FileSet {
    /// A string specifying the name of the file set.
    pub name: String,

    /// A string specifying the type of the file set. See target_sources() supported file set types.
    #[serde(rename = "type")]
    pub type_name: String,

    /// A string specifying the visibility of the file set; one of PUBLIC, PRIVATE, or INTERFACE.
    pub visibility: String,

    /// Base directories containing sources in the file set.
    /// If the directory is inside the top-level source directory then the path is specified
    /// relative to that directory.
    /// Otherwise, the path is absolute.
    pub base_directories: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Source {
    /// Path to the source file on disk, represented with forward slashes.
    /// If the file is inside the top-level source directory then the path is specified relative to that directory.
    /// Otherwise the path is absolute.
    pub path: PathBuf,

    /// Optional member that is present when the source is compiled.
    /// The value is an unsigned integer 0-based index into the compileGroups array.
    pub compile_group_index: Option<usize>,

    /// Optional member that is present when the source is part of a source group either via the source_group() command or by default.
    /// The value is an unsigned integer 0-based index into the sourceGroups array.
    pub source_group_index: Option<usize>,

    /// True if the source is GENERATED.
    #[serde(default)]
    pub is_generated: bool,

    /// Optional member that is present when the source is part of a file set.
    /// The value is an unsigned integer 0-based index into the fileSets array.
    /// This field was added in codemodel version 2.5.
    pub file_set_index: Option<usize>,

    /// Optional member that is present when a CMake language backtrace to the target_sources(), add_executable(), add_library(),
    /// add_custom_target(), or other command invocation that added this source to the target is available.
    /// The value is an unsigned integer 0-based index into the backtraceGraph member's nodes array.
    pub backtrace: Option<usize>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SourceGroup {
    /// A string specifying the name of the source group.
    pub name: String,

    /// Indices to sources belonging to the group.
    /// Each entry is an unsigned integer 0-based index into the main sources array for the target.
    pub source_indexes: Vec<usize>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CompileGroup {
    /// Indices to sources belonging to the compile-group.
    pub source_indexes: Vec<usize>,

    /// A string specifying the language (e.g. C, CXX, Fortran) of the toolchain is used to compile the source file.
    pub language: String,

    /// Optional member that is present when the language standard is set explicitly (e.g. via CXX_STANDARD) or
    /// implicitly by compile features.
    /// This field was added in codemodel version 2.2.
    pub language_standard: Option<LanguageStandard>,

    /// Optional member that is present when fragments of the compiler command line invocation are available.
    #[serde(default)]
    pub compile_command_fragments: Vec<CompileCommandFragment>,

    /// include directories.
    #[serde(default)]
    pub includes: Vec<Include>,

    /// available frameworks (Apple)
    /// This field was added in codemodel version 2.6.
    #[serde(default)]
    pub frameworks: Vec<Framework>,

    /// precompiled headers
    #[serde(default)]
    pub precompile_headers: Vec<PrecompileHeader>,

    /// defines
    #[serde(default)]
    pub defines: Vec<Define>,

    /// Optional member that is present when the `CMAKE_SYSROOT_COMPILE` or `CMAKE_SYSROOT` variable is defined.
    pub sysroot: Option<SysRootPath>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LanguageStandard {
    /// Optional member that is present when a CMake language backtrace to the `<LANG>_STANDARD` setting is available.
    /// If the language standard was set implicitly by compile features those are used as the backtrace(s).
    /// It's possible for multiple compile features to require the same language standard so there could be multiple backtraces.
    /// Each entry being an unsigned integer 0-based index into the backtraceGraph member's nodes array.
    #[serde(default)]
    pub backtraces: Vec<usize>,

    /// String representing the language standard.
    pub standard: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CompileCommandFragment {
    /// A string specifying a fragment of the compile command line invocation.
    /// The value is encoded in the build system's native shell format.
    pub fragment: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Include {
    /// Path to the include directory, represented with forward slashes.
    pub path: PathBuf,

    /// True if the include directory is marked as a system include directory.
    #[serde(default)]
    pub is_system: bool,

    /// Optional member that is present when a CMake language backtrace to the target_include_directories() or
    /// other command invocation that added this include directory is available.
    /// The value is an unsigned integer 0-based index into the backtraceGraph member's nodes array.
    pub backtrace: Option<usize>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Framework {
    /// Path to the framework directory, represented with forward slashes.
    pub path: PathBuf,

    /// True if the framework is marked as a system one.
    #[serde(default)]
    pub is_system: bool,

    /// Optional member that is present when a CMake language backtrace to the target_link_libraries() or
    /// other command invocation that added this framework is available.
    /// The value is an unsigned integer 0-based index into the backtraceGraph member's nodes array.
    pub backtrace: Option<usize>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PrecompileHeader {
    /// Full path to the precompile header file.
    pub header: PathBuf,

    /// Optional member that is present when a CMake language backtrace to the target_precompile_headers() or
    /// other command invocation that added this precompiled header is available.
    /// The value is an unsigned integer 0-based index into the backtraceGraph member's nodes array.
    pub backtrace: Option<usize>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Define {
    /// A string specifying the preprocessor definition in the format `<name>[=<value>]`, e.g. `DEF` or `DEF=1`.
    pub define: String,

    /// Optional member that is present when a CMake language backtrace to the target_compile_definitions() or
    /// other command invocation that added this preprocessor definition is available.
    /// The value is an unsigned integer 0-based index into the backtraceGraph member's nodes array.
    pub backtrace: Option<usize>,
}

impl CompileGroup {
    /// Returns a list of defines for the compile group
    ///
    /// Compile command fragments can contain defines as well (/D or -D).
    #[must_use]
    pub fn defines(&self) -> Vec<String> {
        let mut defines: Vec<String> = self
            .defines
            .iter()
            .map(|define| define.define.clone())
            .collect();
        defines.extend(self.compile_fragments().iter().filter_map(|flag| {
            if Self::is_define(flag) {
                flag.get(2..).map(|define| define.to_owned())
            } else {
                None
            }
        }));
        defines
    }

    /// Returns a list of compile flags for the compile group
    ///
    /// Compile command fragments are split into single flags and defines (/D or -D) are filtered out.
    #[must_use]
    pub fn flags(&self) -> Vec<String> {
        self.compile_fragments()
            .iter()
            .filter(|&flag| !Self::is_define(flag))
            .cloned()
            .collect()
    }

    fn is_define(flag: &str) -> bool {
        flag.starts_with("/D") || flag.starts_with("-D")
    }

    #[must_use]
    /// Compile command fragments are split into single flags.
    pub fn compile_fragments(&self) -> Vec<String> {
        self.compile_command_fragments
            .iter()
            .filter_map(|frag| shlex::split(&frag.fragment))
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::codemodel_v2::target::*;
    use crate::objects::codemodel_v2::Node;
    use serde_json::json;

    #[test]
    fn test_target() {
        let json = json!({
            "backtrace" : 0,
            "backtraceGraph" :
            {
                "commands" : [],
                "files" :
                [
                    "CMakeLists.txt"
                ],
                "nodes" :
                [
                    {
                        "file" : 0
                    }
                ]
            },
            "dependencies" :
            [
                {
                    "id" : "ZERO_CHECK::@6890427a1f51a3e7e1df"
                },
                {
                    "id" : "subbinary::@6890427a1f51a3e7e1df"
                }
            ],
            "id" : "ALL_BUILD::@6890427a1f51a3e7e1df",
            "isGeneratorProvided" : true,
            "name" : "ALL_BUILD",
            "paths" :
            {
                "build" : ".",
                "source" : "."
            },
            "sources" : [],
            "type" : "UTILITY"
        }
        );

        let target = serde_json::from_value::<Target>(json).unwrap();
        assert_eq!(
            target,
            Target {
                backtrace: Some(0),
                backtrace_graph: BacktraceGraph {
                    commands: vec![],
                    files: vec!["CMakeLists.txt".into()],
                    nodes: vec![Node {
                        file: 0,
                        ..Default::default()
                    }]
                },
                dependencies: vec![
                    Dependency {
                        id: "ZERO_CHECK::@6890427a1f51a3e7e1df".to_string(),
                        ..Default::default()
                    },
                    Dependency {
                        id: "subbinary::@6890427a1f51a3e7e1df".to_string(),
                        ..Default::default()
                    }
                ],
                id: "ALL_BUILD::@6890427a1f51a3e7e1df".to_string(),
                is_generator_provided: true,
                name: "ALL_BUILD".to_string(),
                paths: TargetPaths {
                    build: ".".into(),
                    source: ".".into()
                },
                sources: vec![],
                type_name: "UTILITY".to_string(),
                ..Default::default()
            }
        );
    }
}

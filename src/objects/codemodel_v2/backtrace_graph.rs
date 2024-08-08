use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The backtraceGraph member of a "codemodel" version 2 "directory" object, or "codemodel" version 2 "target" object.
/// Describes a graph of backtraces.
/// Its nodes are referenced from backtrace members elsewhere in the containing object.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BacktraceGraph {
    /// Backtrace nodes.
    pub nodes: Vec<Node>,

    /// Command names referenced by backtrace nodes.
    /// Each entry is a string specifying a command name.
    pub commands: Vec<String>,

    /// CMake's language files referenced by backtrace nodes
    /// Each entry is a path to a file, represented with forward slashes.
    /// If the file is inside the top-level source directory then the path is specified relative to that directory.
    /// Otherwise, the path is absolute.
    pub files: Vec<PathBuf>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Node {
    /// An unsigned integer 0-based index into the backtrace files array.
    pub file: usize,

    /// An optional member present when the node represents a line within the file.
    /// The value is an unsigned integer 1-based line number.
    pub line: Option<usize>,

    /// An optional member present when the node represents a command invocation within the file.
    /// The value is an unsigned integer 0-based index into the backtrace commands array.
    pub command: Option<usize>,

    /// An optional member present when the node is not the bottom of the call stack.
    /// The value is an unsigned integer 0-based index of another entry in the backtrace nodes array.
    pub parent: Option<usize>,
}

#[cfg(test)]
mod tests {
    use crate::objects::codemodel_v2::backtrace_graph::*;
    use serde_json::json;

    #[test]
    fn test_backtrace_graph() {
        let json = json!({
            "commands" :
            [
                "add_executable",
                "target_link_libraries"
            ],
            "files" :
            [
                "CMakeLists.txt"
            ],
            "nodes" :
            [
                {
                    "file" : 0
                },
                {
                    "command" : 0,
                    "file" : 0,
                    "line" : 4,
                    "parent" : 0
                },
                {
                    "command" : 1,
                    "file" : 0,
                    "line" : 9,
                    "parent" : 0
                }
            ]
        });

        let graph = serde_json::from_value::<BacktraceGraph>(json).unwrap();
        assert_eq!(
            graph,
            BacktraceGraph {
                commands: vec![
                    "add_executable".to_string(),
                    "target_link_libraries".to_string()
                ],
                files: vec![PathBuf::from("CMakeLists.txt")],
                nodes: vec![
                    Node {
                        file: 0,
                        ..Default::default()
                    },
                    Node {
                        file: 0,
                        command: Some(0),
                        line: Some(4),
                        parent: Some(0)
                    },
                    Node {
                        file: 0,
                        command: Some(1),
                        line: Some(9),
                        parent: Some(0)
                    }
                ]
            }
        );
    }
}

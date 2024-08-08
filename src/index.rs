use crate::objects::{MajorMinor, ObjectKind};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Index {
    /// information about the instance of `CMake` that generated the reply
    pub cmake: CMake,

    /// list of objects that are referenced in the reply
    pub objects: Vec<ReplyFileReference>,

    /// map of replies to client queries
    pub reply: HashMap<String, ReplyField>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CMake {
    pub version: CMakeVersion,
    pub paths: CMakePaths,
    pub generator: CMakeGenerator,
}

/// information about the instance of `CMake` that generated the reply
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CMakeVersion {
    /// specifying the major version component
    pub major: i32,

    /// specifying the minor version component
    pub minor: i32,

    /// specifying the patch version component
    pub patch: i32,

    /// specifying the version suffix, if any, e.g. g0abc3
    pub suffix: String,

    /// specifying the full version in the format `<major>.<minor>.<patch>[-<suffix>]`
    pub string: String,

    /// indicating whether the version was built from a version controlled source tree with local modifications
    pub is_dirty: bool,
}

/// paths to things that come with `CMake`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CMakePaths {
    /// absolute path to cmake tool
    pub cmake: PathBuf,

    /// absolute path to ctest tool
    pub ctest: PathBuf,

    /// absolute path to cpack tool
    pub cpack: PathBuf,

    /// absolute path to the directory containing CMake resources like the Modules/ directory
    pub root: PathBuf,
}

/// describing the `CMake` generator used for the build
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CMakeGenerator {
    /// specifying whether the generator supports multiple output configurations
    pub multi_config: bool,

    /// specifying the name of the generator
    pub name: String,

    /// If the generator supports CMAKE_GENERATOR_PLATFORM, this is a string specifying the generator platform name
    pub platform: Option<String>,
}

/// represents a reference to another reply file
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ReplyFileReference {
    /// specifying one of the Object Kinds
    pub kind: ObjectKind,

    /// object version
    pub version: MajorMinor,

    /// path relative to the reply index file to another JSON file containing the object
    pub json_file: PathBuf,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Error {
    pub error: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ClientField {
    Error(Error),
    ReplyFileReference(ReplyFileReference),
    QueryJson(QueryJson),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ReplyField {
    Error(Error),
    ReplyFileReference(ReplyFileReference),
    Client(HashMap<String, ClientField>),
    #[default]
    Unknown,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct QueryJson {
    pub client: Option<Value>,
    pub requests: Option<Value>,
    pub responses: Option<Value>,
}

#[cfg(test)]
mod testing {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_cmake() {
        let json = json!({
          "generator" :
          {
            "multiConfig" : true,
            "name" : "Visual Studio 16 2019",
            "platform" : "x64"
          },
          "paths" :
          {
            "cmake" : "C:/Program Files/CMake/bin/cmake.exe",
            "cpack" : "C:/Program Files/CMake/bin/cpack.exe",
            "ctest" : "C:/Program Files/CMake/bin/ctest.exe",
            "root" : "C:/Program Files/CMake/share/cmake-3.27"
          },
          "version" : {
            "isDirty": false,
            "major": 3,
            "minor": 27,
            "patch": 7,
            "string": "3.27.7",
            "suffix": ""
          }
        });

        let cmake = serde_json::from_value::<CMake>(json).unwrap();

        assert_eq!(
            cmake,
            CMake {
                version: CMakeVersion {
                    is_dirty: false,
                    major: 3,
                    minor: 27,
                    patch: 7,
                    string: "3.27.7".into(),
                    suffix: String::new(),
                },
                paths: CMakePaths {
                    cmake: "C:/Program Files/CMake/bin/cmake.exe".into(),
                    cpack: "C:/Program Files/CMake/bin/cpack.exe".into(),
                    ctest: "C:/Program Files/CMake/bin/ctest.exe".into(),
                    root: "C:/Program Files/CMake/share/cmake-3.27".into(),
                },
                generator: CMakeGenerator {
                    multi_config: true,
                    platform: Some("x64".into()),
                    name: "Visual Studio 16 2019".into(),
                },
            }
        );
    }

    #[test]
    fn test_cmake_with_unknown_field() {
        let json = json!({
          "generator" :
          {
            "multiConfig" : true,
            "name" : "Visual Studio 16 2019",
            "platform" : "x64",
            "test" : "test"
          },
          "paths" :
          {
            "cmake" : "C:/Program Files/CMake/bin/cmake.exe",
            "cpack" : "C:/Program Files/CMake/bin/cpack.exe",
            "ctest" : "C:/Program Files/CMake/bin/ctest.exe",
            "root" : "C:/Program Files/CMake/share/cmake-3.27"
          },
          "version" : {
            "isDirty": false,
            "major": 3,
            "minor": 27,
            "patch": 7,
            "string": "3.27.7",
            "suffix": ""
          }
        });

        assert_eq!(
            serde_json::from_value::<CMake>(json)
                .unwrap_err()
                .to_string(),
            "unknown field `test`, expected one of `multiConfig`, `name`, `platform`"
        );
    }

    #[test]
    fn test_objects() {
        let json = json!([
          {
            "jsonFile" : "codemodel-v2-b29a741ae0dbe513e631.json",
            "kind" : "codemodel",
            "version" :
            {
              "major" : 2,
              "minor" : 6
            }
          },
          {
            "jsonFile" : "configureLog-v1-cac906d276896c7cc320.json",
            "kind" : "configureLog",
            "version" :
            {
              "major" : 1,
              "minor" : 0
            }
          }
        ]);

        let objects = serde_json::from_value::<Vec<ReplyFileReference>>(json).unwrap();
        assert_eq!(
            objects,
            vec![
                ReplyFileReference {
                    json_file: "codemodel-v2-b29a741ae0dbe513e631.json".into(),
                    kind: ObjectKind::CodeModel,
                    version: MajorMinor { major: 2, minor: 6 }
                },
                ReplyFileReference {
                    json_file: "configureLog-v1-cac906d276896c7cc320.json".into(),
                    kind: ObjectKind::ConfigureLog,
                    version: MajorMinor { major: 1, minor: 0 }
                }
            ]
        );
    }

    #[test]
    fn test_reply_with_error() {
        let json = json!({
          "test_error" :
          {
            "error" : "test error"
          }
        });

        let reply = serde_json::from_value::<HashMap<String, ReplyField>>(json).unwrap();
        let item = reply.iter().next().unwrap();

        assert!(match item.1 {
            ReplyField::Error(e) => e.error == "test error",
            _ => false,
        });
    }
    #[test]
    fn test_reply_with_reply_ref() {
        let json = json!({
          "codemodel-v2" :
          {
            "jsonFile" : "codemodel-v2-b29a741ae0dbe513e631.json",
            "kind" : "codemodel",
            "version" :
            {
              "major" : 2,
              "minor" : 6
            }
          }
        });

        let reply = serde_json::from_value::<HashMap<String, ReplyField>>(json).unwrap();
        let item = reply.iter().next().unwrap();
        assert_eq!(item.0, "codemodel-v2");
        assert!(match item.1 {
            ReplyField::ReplyFileReference(e) =>
                *e == ReplyFileReference {
                    json_file: "codemodel-v2-b29a741ae0dbe513e631.json".into(),
                    kind: ObjectKind::CodeModel,
                    version: MajorMinor { major: 2, minor: 6 },
                },
            _ => false,
        });
    }

    #[test]
    fn test_reply_client_with_reply_ref() {
        let json = json!({
            "codemodel-v2" :
            {
                "jsonFile" : "codemodel-v2-b29a741ae0dbe513e631.json",
                "kind" : "codemodel",
                "version" :
                {
                    "major" : 2,
                    "minor" : 6
                }
            }
        });

        let reply = serde_json::from_value::<HashMap<String, ClientField>>(json).unwrap();
        let item = reply.iter().next().unwrap();
        assert_eq!(item.0, "codemodel-v2");
        assert!(match item.1 {
            ClientField::ReplyFileReference(e) =>
                *e == ReplyFileReference {
                    json_file: "codemodel-v2-b29a741ae0dbe513e631.json".into(),
                    kind: ObjectKind::CodeModel,
                    version: MajorMinor { major: 2, minor: 6 },
                },
            _ => false,
        });
    }

    #[test]
    fn test_reply_client_with_error() {
        let json = json!({
            "bad_query.json" :
            {
                "error" : "unknown query file"
            }
        });

        let reply = serde_json::from_value::<HashMap<String, ClientField>>(json).unwrap();
        let item = reply.iter().next().unwrap();
        assert_eq!(item.0, "bad_query.json");
        assert!(match item.1 {
            ClientField::Error(e) => e.error == "unknown query file",
            _ => false,
        });
    }

    #[test]
    fn test_reply_query_json_with_client() {
        let json = json!({
            "client" :
            {
                "myData" : 10
            },
        });

        let query_json = serde_json::from_value::<QueryJson>(json).unwrap();
        assert_eq!(query_json.client.unwrap()["myData"], 10);
    }

    #[test]
    fn test_reply_query_json_with_requests() {
        let json = json!({
            "requests" :
            [
                {
                    "kind" : "codemodel",
                    "version" : 2
                }
            ]
        });

        let query_json = serde_json::from_value::<QueryJson>(json).unwrap();
        assert!(query_json
            .requests
            .unwrap()
            .as_array()
            .unwrap()
            .first()
            .unwrap()
            .is_object());
    }

    #[test]
    fn test_reply_query_json_with_responses() {
        let json = json!({
            "responses" :
            [
                {
                    "jsonFile" : "codemodel-v2-b29a741ae0dbe513e631.json",
                    "kind" : "codemodel",
                    "version" :
                    {
                        "major" : 2,
                        "minor" : 6
                    }
                },
                {
                    "error": "error"
                }
            ]
        });

        let query_json = serde_json::from_value::<QueryJson>(json).unwrap();
        assert!(query_json
            .responses
            .unwrap()
            .as_array()
            .unwrap()
            .first()
            .unwrap()
            .is_object());
    }
    #[test]
    fn test_reply_query_json_with_response_error() {
        let json = json!({
            "responses" :
                {
                    "error" : "unknown request kind 'bad_name'"
                }
        });

        let query_json = serde_json::from_value::<QueryJson>(json).unwrap();
        assert!(query_json.responses.unwrap().is_object());
    }

    #[test]
    fn test_index() {
        let json = json!({
          "cmake": {
            "version": {
              "major": 3, "minor": 14, "patch": 0, "suffix": "",
              "string": "3.14.0", "isDirty": false
            },
            "paths": {
              "cmake": "/prefix/bin/cmake",
              "ctest": "/prefix/bin/ctest",
              "cpack": "/prefix/bin/cpack",
              "root": "/prefix/share/cmake-3.14"
            },
            "generator": {
              "multiConfig": false,
              "name": "Unix Makefiles"
            }
          },
          "objects": [
            { "kind": "codemodel",
              "version": { "major": 1, "minor": 0 },
              "jsonFile": "test.json" },
          ],
          "reply": {
            "<kind>-v<major>": { "kind": "codemodel",
                                 "version": { "major": 1, "minor": 0 },
                                 "jsonFile": "test.json" },
            "<unknown>": { "error": "unknown query file" },
            "client-<client>": {
              "<kind>-v<major>": { "kind": "codemodel",
                                   "version": { "major": 1, "minor": 0 },
                                   "jsonFile": "test.json" },
              "<unknown>": { "error": "unknown query file" },
              "query.json": {
                "requests": [ {}, {}, {} ],
                "responses": [
                  { "kind": "codemodel",
                    "version": { "major": 1, "minor": 0 },
                    "jsonFile": "test.json" },
                  { "error": "unknown query file" },
                ],
                "client": {}
              }
            }
          }
        });

        serde_json::from_value::<Index>(json).unwrap();
    }
}

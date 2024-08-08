use crate::objects::{MajorMinor, Object, ObjectKind};
use serde::{Deserialize, Serialize};

/// The cache object kind lists cache entries.
/// These are the Variables stored in the persistent cache (CMakeCache.txt) for the build tree.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Cache {
    /// Kind of the cache object
    pub kind: ObjectKind,

    /// Version of the cache object
    pub version: MajorMinor,

    /// Entries in the cache
    pub entries: Vec<Entry>,
}

/// Entry in the cache
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Entry {
    /// Name of the entry
    pub name: String,

    /// Value of the entry
    pub value: String,

    /// Type of the entry
    #[serde(rename = "type")]
    pub type_name: String,

    /// Properties of the entry
    pub properties: Vec<Property>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Property {
    /// Name of the property
    pub name: String,

    /// Value of the property
    pub value: String,
}

impl Object for Cache {
    fn kind() -> ObjectKind {
        ObjectKind::Cache
    }

    fn major() -> u32 {
        2
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::cache_v2::*;
    use serde_json::json;

    #[test]
    fn test_configure_log() {
        let json = json!({
          "kind": "cache",
          "version": { "major": 2, "minor": 0 },
          "entries": [
            {
              "name": "BUILD_SHARED_LIBS",
              "value": "ON",
              "type": "BOOL",
              "properties": [
                {
                  "name": "HELPSTRING",
                  "value": "Build shared libraries"
                }
              ]
            },
            {
              "name": "CMAKE_GENERATOR",
              "value": "Unix Makefiles",
              "type": "INTERNAL",
              "properties": [
                {
                  "name": "HELPSTRING",
                  "value": "Name of generator."
                }
              ]
            }
          ]
        });

        let cache = serde_json::from_value::<Cache>(json).unwrap();
        assert_eq!(
            cache,
            Cache {
                kind: ObjectKind::Cache,
                version: MajorMinor { major: 2, minor: 0 },
                entries: vec![
                    Entry {
                        name: "BUILD_SHARED_LIBS".into(),
                        value: "ON".into(),
                        type_name: "BOOL".into(),
                        properties: vec![Property {
                            name: "HELPSTRING".into(),
                            value: "Build shared libraries".into(),
                        }]
                    },
                    Entry {
                        name: "CMAKE_GENERATOR".into(),
                        value: "Unix Makefiles".into(),
                        type_name: "INTERNAL".into(),
                        properties: vec![Property {
                            name: "HELPSTRING".into(),
                            value: "Name of generator.".into(),
                        }]
                    }
                ]
            }
        );
    }
}

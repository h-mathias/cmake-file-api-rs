use crate::objects::{MajorMinor, Object, ObjectKind};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The configureLog object kind describes the location and contents of a cmake-configure-log(7) file.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ConfigureLog {
    /// Kind of the configureLog object.
    pub kind: ObjectKind,

    /// Version of the configureLog object.
    pub version: MajorMinor,

    /// Path to the configure log file.
    /// Clients must read the log file from this path, which may be different from the path documented by cmake-configure-log(7).
    /// The log file may not exist if no events are logged.
    pub path: PathBuf,

    /// Names of the event kinds that are logged in the configure log.
    pub event_kind_names: Vec<String>,
}

impl Object for ConfigureLog {
    fn kind() -> ObjectKind {
        ObjectKind::ConfigureLog
    }

    fn major() -> u32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::configure_log_v1::*;
    use crate::objects::MajorMinor;
    use serde_json::json;

    #[test]
    fn test_configure_log() {
        let json = json!({
          "kind" : "configureLog",
          "path" : "build/CMakeFiles/CMakeConfigureLog.yaml",
          "version" :
          {
            "major" : 1,
            "minor" : 0
          },
          "eventKindNames" :
          [
            "message-v1",
            "try_compile-v1",
            "try_run-v1"
          ]
        });

        let configure_log = serde_json::from_value::<ConfigureLog>(json).unwrap();
        assert_eq!(
            configure_log,
            ConfigureLog {
                event_kind_names: vec![
                    "message-v1".into(),
                    "try_compile-v1".into(),
                    "try_run-v1".into()
                ],
                kind: ObjectKind::ConfigureLog,
                path: "build/CMakeFiles/CMakeConfigureLog.yaml".into(),
                version: MajorMinor { major: 1, minor: 0 }
            }
        );
    }
}

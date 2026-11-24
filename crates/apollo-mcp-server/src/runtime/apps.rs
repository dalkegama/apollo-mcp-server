use std::path::PathBuf;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct AppsConfig {
    /// Directory containing MCP Apps. Each subdirectory holding an
    /// `.application-manifest.json` file is loaded as an app.
    pub path: PathBuf,

    /// Serve every app on a connection that has no `app` query parameter.
    /// The connection lists each app's tools and resources alongside the
    /// operation tools and the enabled built-in tools.
    pub expose_all: bool,
}

impl Default for AppsConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("apps"),
            expose_all: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AppsConfig;

    #[test]
    fn default_path_matches_existing_apps_directory() {
        assert_eq!(AppsConfig::default().path, std::path::PathBuf::from("apps"));
    }

    #[test]
    fn deserializes_custom_path() {
        let config = yaml_serde::from_str::<AppsConfig>("path: /config/apps\n")
            .expect("Apps config should parse");

        assert_eq!(config.path, std::path::PathBuf::from("/config/apps"));
    }

    #[test]
    fn expose_all_defaults_to_false() {
        assert!(!AppsConfig::default().expose_all);
    }

    #[test]
    fn deserializes_expose_all() {
        let config = yaml_serde::from_str::<AppsConfig>("expose_all: true\n")
            .expect("Apps config should parse");

        assert!(config.expose_all);
        assert_eq!(config.path, std::path::PathBuf::from("apps"));
    }

    #[test]
    fn rejects_unknown_fields() {
        let err = yaml_serde::from_str::<AppsConfig>("dir: /config/apps\n")
            .expect_err("unknown field should be rejected")
            .to_string();

        assert!(err.contains("unknown field"), "unexpected error: {err}");
    }
}

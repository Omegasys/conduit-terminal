//! Conduit default configuration.

use std::collections::BTreeMap;

use super::schema::ConfigValue;

/// Default configuration container.
#[derive(Debug, Clone)]
pub struct DefaultConfig {
    value: ConfigValue,
}

impl DefaultConfig {
    pub fn new() -> Self {
        Self {
            value:
                default_config(),
        }
    }

    pub fn value(
        &self,
    ) -> &ConfigValue {
        &self.value
    }
}

impl Default for DefaultConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Builds Conduit's default configuration.
pub fn default_config() -> ConfigValue {
    let mut root =
        BTreeMap::new();

    root.insert(
        "terminal".into(),
        table([
            (
                "shell",
                ConfigValue::String(
                    detect_default_shell()
                ),
            ),
            (
                "scrollback",
                ConfigValue::Integer(
                    10_000
                ),
            ),
            (
                "cursor_style",
                ConfigValue::String(
                    "block".into()
                ),
            ),
            (
                "cursor_blink",
                ConfigValue::Boolean(
                    true
                ),
            ),
        ]),
    );

    root.insert(
        "window".into(),
        table([
            (
                "width",
                ConfigValue::Integer(
                    1200
                ),
            ),
            (
                "height",
                ConfigValue::Integer(
                    800
                ),
            ),
            (
                "resizable",
                ConfigValue::Boolean(
                    true
                ),
            ),
        ]),
    );

    root.insert(
        "appearance".into(),
        table([
            (
                "theme",
                ConfigValue::String(
                    "default".into()
                ),
            ),
            (
                "font_size",
                ConfigValue::Float(
                    12.0
                ),
            ),
            (
                "font_family",
                ConfigValue::String(
                    "monospace".into()
                ),
            ),
        ]),
    );

    root.insert(
        "security".into(),
        table([
            (
                "safe_mode",
                ConfigValue::Boolean(
                    false
                ),
            ),
            (
                "allow_osc8_links",
                ConfigValue::Boolean(
                    true
                ),
            ),
            (
                "allow_clipboard_write",
                ConfigValue::Boolean(
                    true
                ),
            ),
        ]),
    );

    root.insert(
        "behavior".into(),
        table([
            (
                "confirm_close",
                ConfigValue::Boolean(
                    true
                ),
            ),
            (
                "restore_session",
                ConfigValue::Boolean(
                    true
                ),
            ),
            (
                "live_reload",
                ConfigValue::Boolean(
                    true
                ),
            ),
        ]),
    );

    ConfigValue::Table(root)
}

fn table(
    entries: impl IntoIterator<
        Item = (&'static str, ConfigValue),
    >,
) -> ConfigValue {
    ConfigValue::Table(
        entries
            .into_iter()
            .map(
                |(key, value)| {
                    (
                        key.to_string(),
                        value,
                    )
                }
            )
            .collect()
    )
}

fn detect_default_shell() -> String {
    std::env::var("SHELL")
        .unwrap_or_else(
            |_| {
                if cfg!(target_os = "windows") {
                    "cmd.exe".into()
                } else {
                    "/bin/sh".into()
                }
            }
        )
}

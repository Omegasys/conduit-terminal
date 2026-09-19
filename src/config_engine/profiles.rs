//! Configuration profiles.
//!
//! Profiles provide named configuration presets such as `default`,
//! `minimal`, `development`, `security`, `hardened`, and `ssh`.

use std::collections::BTreeMap;

use super::{
    defaults::default_config,
    schema::ConfigValue,
};

/// A named configuration profile.
#[derive(Debug, Clone)]
pub struct ConfigProfile {
    name: String,
    description: String,
    settings: ConfigValue,
}

impl ConfigProfile {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        settings: ConfigValue,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            settings,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(
        &self,
    ) -> &str {
        &self.description
    }

    pub fn settings(
        &self,
    ) -> &ConfigValue {
        &self.settings
    }
}

/// Profile selection state.
#[derive(Debug, Clone)]
pub struct ProfileSelection {
    active: String,
}

impl ProfileSelection {
    pub fn new(
        profile: impl Into<String>,
    ) -> Self {
        Self {
            active: profile.into(),
        }
    }

    pub fn active(
        &self,
    ) -> &str {
        &self.active
    }

    pub fn set_active(
        &mut self,
        profile: impl Into<String>,
    ) {
        self.active =
            profile.into();
    }
}

impl Default for ProfileSelection {
    fn default() -> Self {
        Self::new("default")
    }
}

/// Manages available configuration profiles.
#[derive(Debug)]
pub struct ProfileManager {
    profiles:
        BTreeMap<String, ConfigProfile>,

    selection:
        ProfileSelection,
}

impl ProfileManager {
    pub fn new() -> Self {
        let mut manager =
            Self {
                profiles:
                    BTreeMap::new(),

                selection:
                    ProfileSelection::default(),
            };

        manager.register_builtin_profiles();

        manager
    }

    pub fn profiles(
        &self,
    ) -> &BTreeMap<String, ConfigProfile> {
        &self.profiles
    }

    pub fn get(
        &self,
        name: &str,
    ) -> Option<&ConfigProfile> {
        self.profiles.get(name)
    }

    pub fn active(
        &self,
    ) -> Option<&ConfigProfile> {
        self.get(
            self.selection.active()
        )
    }

    pub fn selection(
        &self,
    ) -> &ProfileSelection {
        &self.selection
    }

    pub fn set_active(
        &mut self,
        name: &str,
    ) -> bool {
        if !self.profiles.contains_key(
            name
        ) {
            return false;
        }

        self.selection
            .set_active(name);

        true
    }

    pub fn register(
        &mut self,
        profile: ConfigProfile,
    ) {
        self.profiles.insert(
            profile.name()
                .to_string(),
            profile,
        );
    }

    pub fn remove(
        &mut self,
        name: &str,
    ) -> Option<ConfigProfile> {
        if name == "default" {
            return None;
        }

        self.profiles.remove(name)
    }

    pub fn merged_settings(
        &self,
    ) -> Option<ConfigValue> {
        let profile =
            self.active()?;

        Some(
            merge_values(
                default_config(),
                profile.settings()
                    .clone(),
            )
        )
    }

    fn register_builtin_profiles(
        &mut self,
    ) {
        self.register(
            ConfigProfile::new(
                "default",
                "Conduit's standard configuration.",
                default_config(),
            )
        );

        self.register(
            ConfigProfile::new(
                "minimal",
                "A minimal interface with reduced visual effects.",
                profile_settings([
                    (
                        "appearance.theme",
                        ConfigValue::String(
                            "default".into()
                        ),
                    ),
                    (
                        "appearance.font_size",
                        ConfigValue::Float(
                            11.0
                        ),
                    ),
                    (
                        "behavior.confirm_close",
                        ConfigValue::Boolean(
                            false
                        ),
                    ),
                ]),
            )
        );

        self.register(
            ConfigProfile::new(
                "development",
                "Settings useful for Conduit development.",
                profile_settings([
                    (
                        "behavior.live_reload",
                        ConfigValue::Boolean(
                            true
                        ),
                    ),
                    (
                        "terminal.scrollback",
                        ConfigValue::Integer(
                            50_000
                        ),
                    ),
                ]),
            )
        );

        self.register(
            ConfigProfile::new(
                "security",
                "Security-focused terminal defaults.",
                profile_settings([
                    (
                        "security.safe_mode",
                        ConfigValue::Boolean(
                            true
                        ),
                    ),
                    (
                        "security.allow_osc8_links",
                        ConfigValue::Boolean(
                            false
                        ),
                    ),
                    (
                        "security.allow_clipboard_write",
                        ConfigValue::Boolean(
                            false
                        ),
                ]),
            )
        );

        self.register(
            ConfigProfile::new(
                "hardened",
                "Restrictive configuration for high-security environments.",
                profile_settings([
                    (
                        "security.safe_mode",
                        ConfigValue::Boolean(
                            true
                        ),
                    ),
                    (
                        "security.allow_osc8_links",
                        ConfigValue::Boolean(
                            false
                        ),
                    ),
                    (
                        "security.allow_clipboard_write",
                        ConfigValue::Boolean(
                            false
                        ),
                    ),
                    (
                        "behavior.confirm_close",
                        ConfigValue::Boolean(
                            true
                        ),
                    ),
                ]),
            )
        );

        self.register(
            ConfigProfile::new(
                "ssh",
                "Settings intended for remote SSH sessions.",
                profile_settings([
                    (
                        "terminal.scrollback",
                        ConfigValue::Integer(
                            20_000
                        ),
                    ),
                    (
                        "terminal.cursor_blink",
                        ConfigValue::Boolean(
                            false
                        ),
                ]),
            )
        );
    }
}

impl Default for ProfileManager {
    fn default() -> Self {
        Self::new()
    }
}

fn profile_settings(
    entries: impl IntoIterator<
        Item = (&'static str, ConfigValue),
    >,
) -> ConfigValue {
    let mut root =
        BTreeMap::new();

    for (path, value) in entries {
        insert_path(
            &mut root,
            path,
            value,
        );
    }

    ConfigValue::Table(root)
}

fn insert_path(
    root: &mut BTreeMap<String, ConfigValue>,
    path: &str,
    value: ConfigValue,
) {
    let parts: Vec<&str> =
        path.split('.').collect();

    if parts.is_empty() {
        return;
    }

    let mut current = root;

    for part in &parts[..parts.len() - 1] {
        let entry =
            current
                .entry(
                    (*part).to_string()
                )
                .or_insert_with(
                    || {
                        ConfigValue::Table(
                            BTreeMap::new()
                        )
                    }
                );

        let ConfigValue::Table(table) =
            entry
        else {
            return;
        };

        current = table;
    }

    current.insert(
        parts[parts.len() - 1]
            .to_string(),
        value,
    );
}

fn merge_values(
    base: ConfigValue,
    overlay: ConfigValue,
) -> ConfigValue {
    match (base, overlay) {
        (
            ConfigValue::Table(mut base),
            ConfigValue::Table(overlay),
        ) => {
            for (
                key,
                value,
            ) in overlay {
                let merged =
                    match base.remove(&key) {
                        Some(existing) =>
                            merge_values(
                                existing,
                                value,
                            ),

                        None => value,
                    };

                base.insert(
                    key,
                    merged,
                );
            }

            ConfigValue::Table(base)
        }

        (_, overlay) => overlay,
    }
}

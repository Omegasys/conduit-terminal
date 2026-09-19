use std::collections::BTreeMap;

use super::errors::ConfigError;
use super::ConfigValue;

pub trait ConfigMigration {
    fn from_version(&self) -> u32;
    fn to_version(&self) -> u32;

    fn migrate(
        &self,
        values: &mut BTreeMap<String, ConfigValue>,
    ) -> Result<(), ConfigError>;
}

pub struct RenameKeyMigration {
    from_version: u32,
    to_version: u32,
    old_key: String,
    new_key: String,
}

impl RenameKeyMigration {
    pub fn new<S: Into<String>>(
        from_version: u32,
        to_version: u32,
        old_key: S,
        new_key: S,
    ) -> Self {
        Self {
            from_version,
            to_version,
            old_key: old_key.into(),
            new_key: new_key.into(),
        }
    }
}

impl ConfigMigration for RenameKeyMigration {
    fn from_version(&self) -> u32 {
        self.from_version
    }

    fn to_version(&self) -> u32 {
        self.to_version
    }

    fn migrate(
        &self,
        values: &mut BTreeMap<String, ConfigValue>,
    ) -> Result<(), ConfigError> {
        if let Some(value) = values.remove(&self.old_key) {
            values.insert(self.new_key.clone(), value);
        }

        Ok(())
    }
}

pub struct ConfigMigrationManager {
    migrations: Vec<Box<dyn ConfigMigration + Send + Sync>>,
    current_version: u32,
}

impl ConfigMigrationManager {
    pub fn new(current_version: u32) -> Self {
        Self {
            migrations: Vec::new(),
            current_version,
        }
    }

    pub fn register<M>(&mut self, migration: M)
    where
        M: ConfigMigration + Send + Sync + 'static,
    {
        self.migrations.push(Box::new(migration));

        self.migrations
            .sort_by_key(|migration| migration.from_version());
    }

    pub fn current_version(&self) -> u32 {
        self.current_version
    }

    pub fn migrate(
        &self,
        values: &mut BTreeMap<String, ConfigValue>,
        version: u32,
    ) -> Result<u32, ConfigError> {
        let mut version = version;

        if version > self.current_version {
            return Err(ConfigError::FutureConfigVersion {
                found: version,
                supported: self.current_version,
            });
        }

        while version < self.current_version {
            let migration = self
                .migrations
                .iter()
                .find(|migration| migration.from_version() == version)
                .ok_or(ConfigError::MissingMigration {
                    from: version,
                    to: self.current_version,
                })?;

            migration.migrate(values)?;

            version = migration.to_version();
        }

        Ok(version)
    }
}

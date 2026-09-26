pub mod bash;
pub mod database;
pub mod export;
pub mod filtering;
pub mod fish;
pub mod import;
pub mod indexing;
pub mod manager;
pub mod metadata;
pub mod powershell;
pub mod privacy;
pub mod redaction;
pub mod search;
pub mod shell_history;
pub mod storage;
pub mod zsh;

pub use bash::BashHistory;
pub use database::{HistoryDatabase, HistoryDatabaseEntry};
pub use export::{HistoryExportFormat, HistoryExporter};
pub use filtering::{HistoryFilter, HistoryFilterBuilder};
pub use fish::FishHistory;
pub use import::{HistoryImportFormat, HistoryImporter};
pub use indexing::{HistoryIndex, HistoryIndexEntry};
pub use manager::HistoryManager;
pub use metadata::HistoryMetadata;
pub use powershell::PowerShellHistory;
pub use privacy::{HistoryPrivacyMode, HistoryPrivacyPolicy};
pub use redaction::{HistoryRedactionPolicy, RedactionRule};
pub use search::{HistorySearch, HistorySearchOptions, HistorySearchResult};
pub use shell_history::{
    ShellHistory,
    ShellHistoryEntry,
    ShellHistoryFormat,
};
pub use storage::{
    HistoryStorage,
    HistoryStorageConfig,
    HistoryStorageError,
};
pub use zsh::ZshHistory;

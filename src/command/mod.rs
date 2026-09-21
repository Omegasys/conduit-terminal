pub mod boundaries;
pub mod database;
pub mod duration;
pub mod execution;
pub mod history;
pub mod palette;
pub mod recording;
pub mod search;
pub mod sensitive;

pub use boundaries::{
    CommandBoundary,
    CommandBoundaryDetector,
    CommandBoundaryKind,
};

pub use database::{
    CommandDatabase,
    CommandDatabaseEntry,
    CommandDatabaseStats,
};

pub use duration::{
    CommandDuration,
    CommandDurationTracker,
};

pub use execution::{
    CommandExecution,
    CommandExecutionManager,
    CommandExecutionState,
};

pub use history::{
    CommandHistory,
    CommandHistoryEntry,
    CommandHistorySource,
};

pub use palette::{
    CommandPalette,
    CommandPaletteEntry,
    CommandPaletteItem,
};

pub use recording::{
    CommandRecording,
    CommandRecordingEvent,
    CommandRecordingEventKind,
    CommandRecorder,
};

pub use search::{
    CommandSearch,
    CommandSearchMatch,
    CommandSearchOptions,
};

pub use sensitive::{
    SensitiveCommandDetector,
    SensitiveCommandPattern,
    SensitiveCommandResult,
};

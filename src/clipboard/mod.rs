pub mod history;
pub mod large_paste;
pub mod manager;
pub mod paste_protection;
pub mod primary;
pub mod security;
pub mod sensitive_data;

pub use history::{
    ClipboardHistory,
    ClipboardHistoryEntry,
};

pub use large_paste::{
    LargePasteConfig,
    LargePasteHandler,
    LargePasteStrategy,
};

pub use manager::{
    ClipboardManager,
    ClipboardProvider,
    ClipboardType,
};

pub use paste_protection::{
    PasteDecision,
    PasteProtection,
    PasteProtectionConfig,
};

pub use primary::{
    PrimarySelection,
    PrimarySelectionManager,
};

pub use security::{
    ClipboardPermission,
    ClipboardSecurityPolicy,
};

pub use sensitive_data::{
    SensitiveDataDetector,
    SensitiveDataKind,
    SensitiveDataResult,
};

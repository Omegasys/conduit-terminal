pub mod autocomplete;
pub mod editor;
pub mod syntax;
pub mod validation;

pub use autocomplete::{
    AutoCompleteContext,
    AutoCompleteEngine,
    CompletionItem,
    CompletionItemKind,
};

pub use editor::{
    ConfigEditor,
    ConfigEditorState,
    EditorCursor,
    EditorSelection,
};

pub use syntax::{
    SyntaxElement,
    SyntaxElementKind,
    SyntaxHighlighter,
    SyntaxLine,
};

pub use validation::{
    ConfigDiagnostic,
    ConfigDiagnosticLevel,
    ConfigValidationState,
    ConfigValidatorUi,
};
pub mod apply;
pub mod autocomplete;
pub mod diagnostics;
pub mod documentation;
pub mod editor;
pub mod preview;
pub mod revert;
pub mod syntax;
pub mod validation;

pub use apply::{
    ApplyConfirmation,
    ApplyResult,
    ConfigApplyController,
};

pub use autocomplete::{
    AutoCompleteContext,
    AutoCompleteEngine,
    CompletionItem,
    CompletionItemKind,
};

pub use diagnostics::{
    DiagnosticCollection,
    DiagnosticDisplayMode,
    DiagnosticsSettings,
};

pub use documentation::{
    ConfigDocumentation,
    DocumentationRegistry,
};

pub use editor::{
    ConfigEditor,
    ConfigEditorState,
    ConfigValueMap,
    EditorCursor,
    EditorSelection,
};

pub use preview::{
    ConfigPreview,
    PreviewChange,
    PreviewChangeKind,
};

pub use revert::{
    ConfigRevertController,
    RevertMode,
    RevertResult,
};

pub use syntax::{
    SyntaxElement,
    SyntaxElementKind,
    SyntaxHighlighter,
    SyntaxLine,
};

pub use validation::{
    ConfigDiagnostic,
    ConfigDiagnosticLevel,
    ConfigValidationState,
    ConfigValidatorUi,
};

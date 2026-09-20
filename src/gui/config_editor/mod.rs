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

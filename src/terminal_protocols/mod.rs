pub mod core;
pub mod ecma48;
pub mod vt;
pub mod dec;
pub mod graphics;
pub mod modern;

// Existing protocol modules.
pub mod ansi;
pub mod csi;
pub mod sgr;
pub mod osc;
pub mod dcs;
pub mod mouse;
pub mod vt100;
pub mod vt220;
pub mod vt320;
pub mod vt420;
pub mod vt520;
pub mod xterm;
pub mod clipboard;
pub mod bracketed_paste;
pub mod kitty;
pub mod iterm2;
pub mod sixel;

// Core protocol infrastructure.
pub use core::{
    AnsiParser,
    AnsiSequence,
    ApcParser,
    ApcSequence,
    CharacterSet,
    CharacterSetState,
    ControlCode,
    ControlFunction,
    CsiParser,
    CsiSequence,
    DcsParser,
    DcsSequence,
    OscParser,
    OscSequence,
    TerminalEncoding,
    TerminalTextDecoder,
    TerminalStatus,
    TerminalStatusType,
    UnicodeWidthMode,
    UnicodeWidthProvider,
};

// VT family.
pub use vt::{
    VtCapabilities,
    VtLevel,
    Vt52,
    Vt100 as Vt100Protocol,
    Vt102,
    Vt125,
    Vt131,
    Vt180,
    Vt220 as Vt220Protocol,
    Vt240,
    Vt320 as Vt320Protocol,
    Vt340,
    Vt420 as Vt420Protocol,
    Vt520 as Vt520Protocol,
    Vt525,
};

// Modern extensions.
pub use modern::{
    KittyCapability,
    KittyProtocol,
    ShellSemanticEvent,
    ShellSemanticEventKind,
    VsCodeShellIntegration,
    WezTermProtocol,
};

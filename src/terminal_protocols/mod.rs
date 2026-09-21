//! Terminal protocol support for Conduit.
//!
//! This module provides protocol capability descriptions and escape-sequence
//! decoding for ANSI, VT100, VT220, VT320, VT420, VT520, and xterm-style
//! terminals.
//!
//! Protocol implementations intentionally normalize terminal sequences into
//! shared actions so the terminal core does not need to know which protocol
//! produced them.

pub mod ansi;
pub mod vt100;
pub mod vt220;
pub mod vt320;
pub mod vt420;
pub mod vt520;
pub mod xterm;

pub use ansi::{
    AnsiAction,
    AnsiDecoder,
    AnsiParserState,
};

pub use vt100::{
    Vt100Capabilities,
    Vt100Protocol,
};

pub use vt220::{
    Vt220Capabilities,
    Vt220Protocol,
};

pub use vt320::{
    Vt320Capabilities,
    Vt320Protocol,
};

pub use vt420::{
    Vt420Capabilities,
    Vt420Protocol,
};

pub use vt520::{
    Vt520Capabilities,
    Vt520Protocol,
};

pub use xterm::{
    XtermAction,
    XtermCapabilities,
    XtermProtocol,
};
pub mod csi;
pub mod dec;
pub mod dcs;
pub mod mouse;
pub mod osc;
pub mod sgr;

pub use csi::{
    CsiError,
    CsiParser,
    CsiSequence,
    CsiState,
};

pub use dec::{
    DecAction,
    DecMode,
    DecParser,
};

pub use dcs::{
    DcsParser,
    DcsSequence,
    DcsState,
};

pub use mouse::{
    MouseAction,
    MouseButton,
    MouseEvent,
    MouseModifiers,
    MouseParser,
    MouseProtocol,
};

pub use osc::{
    OscCommand,
    OscParser,
    OscTerminator,
};

pub use sgr::{
    SgrAttributes,
    SgrBlink,
    SgrIntensity,
    SgrUnderline,
};
pub mod bracketed_paste;
pub mod clipboard;
pub mod iterm2;
pub mod kitty;
pub mod sixel;
pub use bracketed_paste::{
    BracketedPaste,
    BracketedPasteEvent,
    BracketedPasteParser,
    BracketedPasteState,
};

pub use clipboard::{
    ClipboardError,
    ClipboardOperation,
    ClipboardParser,
    ClipboardRequest,
    ClipboardSelection,
};

pub use iterm2::{
    Iterm2Command,
    Iterm2FileCommand,
    Iterm2ImageCommand,
    Iterm2Protocol,
};

pub use kitty::{
    KittyGraphicsAction,
    KittyGraphicsCommand,
    KittyGraphicsFormat,
    KittyKeyboardFlags,
    KittyKeyboardMode,
    KittyKeyboardState,
    KittyProtocol,
};

pub use sixel::{
    SixelColor,
    SixelDecoder,
    SixelImage,
    SixelPalette,
    SixelState,
};
pub mod custom;

pub use custom::{
    CustomProtocolAction,
    CustomProtocolCapabilities,
    CustomProtocolError,
    CustomProtocolEvent,
    CustomProtocolLoader,
    CustomProtocolManifest,
    CustomProtocolParser,
    CustomProtocolRegistry,
    CustomTerminalProtocol,
    ProtocolContext,
    ProtocolId,
    ProtocolRegistration,
    ProtocolVersion,
};

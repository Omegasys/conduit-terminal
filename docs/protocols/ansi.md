# ANSI Terminal Protocol

## Overview

ANSI terminal control sequences are a foundational part of terminal communication. Conduit uses ANSI escape sequences as one of the primary protocol families understood by the terminal core.

ANSI support allows applications running inside Conduit to control terminal behavior such as text formatting, cursor movement, screen clearing, colors, and terminal modes.

The protocol implementation lives under:

```text
src/terminal_protocols/
└── ansi/
```

The ANSI protocol layer is responsible for recognizing ANSI escape sequences and translating them into Conduit's internal terminal commands.

## Design Goals

Conduit's ANSI implementation should:

* Support commonly used ANSI escape sequences.
* Integrate with the shared terminal parser.
* Normalize sequences into internal terminal operations.
* Preserve compatibility with existing Unix applications.
* Handle malformed sequences safely.
* Avoid allowing terminal output to bypass security controls.
* Support incremental parsing across multiple PTY reads.
* Avoid blocking the terminal when processing large output streams.

## Escape Sequences

ANSI terminal control sequences commonly begin with the ESC character:

```text
ESC = 0x1B
```

A common Control Sequence Introducer is:

```text
ESC [
```

This is commonly represented as:

```text
CSI
```

For example:

```text
ESC [ 2 J
```

requests that the terminal clear the display.

Conduit should parse escape sequences as a stream rather than assuming that one PTY read contains one complete sequence.

## Parser Architecture

ANSI parsing should be integrated into the common terminal protocol parser.

The parser should operate incrementally:

```text
PTY output
    ↓
Byte stream
    ↓
Protocol detector
    ↓
ANSI parser
    ↓
Normalized terminal command
    ↓
Terminal state
    ↓
Renderer
```

Incomplete sequences should remain buffered until sufficient input arrives.

## Cursor Control

ANSI sequences can control cursor movement and position.

Conduit should support operations including:

* Cursor up
* Cursor down
* Cursor left
* Cursor right
* Cursor positioning
* Cursor movement by row and column
* Horizontal positioning
* Vertical positioning
* Save cursor position
* Restore cursor position

These operations should modify the internal terminal state rather than directly manipulating the renderer.

## Screen Operations

ANSI control sequences can request operations such as:

* Erasing portions of the display
* Clearing the entire display
* Erasing portions of a line
* Clearing the current line
* Moving the cursor
* Resetting terminal state

Conduit should translate these into state changes and allow the renderer to determine the minimum region requiring redraw.

## Select Graphic Rendition

ANSI SGR sequences control text attributes.

Conduit should support commonly used attributes including:

* Reset
* Bold
* Dim
* Italic
* Underline
* Blink
* Reverse video
* Hidden text
* Strikethrough
* Foreground colors
* Background colors
* Extended 256-color modes
* Truecolor modes

The internal representation should remain independent of the ANSI encoding.

For example:

```text
ANSI SGR
    ↓
Terminal style
    ↓
Renderer
```

This allows different protocols to produce the same internal styling representation.

## Colors

Conduit should support:

* Standard ANSI colors
* Bright ANSI colors
* 256-color palettes
* RGB truecolor

Theme configuration should determine how logical terminal colors are ultimately displayed.

Protocol parsing should not directly depend on a particular Conduit theme.

## Terminal Modes

ANSI sequences may change terminal modes.

Conduit should route supported mode changes through the terminal state and mode manager.

Examples include:

* Cursor visibility
* Application cursor mode
* Insert mode
* Line wrapping
* Keyboard modes
* Mouse reporting modes

Mode changes should generate internal events when other Conduit components need to react.

## Unsupported Sequences

Unknown or unsupported ANSI sequences should not normally terminate a session.

Conduit should:

1. Parse the sequence safely.
2. Determine whether it is supported.
3. Execute it when supported.
4. Ignore or safely preserve unsupported behavior when necessary.
5. Optionally record diagnostics.

Unknown sequences should not be interpreted as arbitrary executable data.

## Security

Terminal output must be treated as untrusted input.

ANSI processing should therefore be subject to Conduit's terminal security model.

Security controls may include:

* Restricted control sequences
* Hyperlink restrictions
* Clipboard protection
* Notification restrictions
* Suspicious sequence diagnostics
* Maximum sequence lengths
* Parser timeouts or limits
* Protection against excessive output
* Safe handling of malformed escape sequences

## Performance

ANSI parsing occurs on potentially high-volume PTY streams.

The implementation should therefore:

* Avoid unnecessary allocations.
* Process byte streams incrementally.
* Reuse parser buffers.
* Batch terminal updates.
* Avoid forcing a renderer update for every byte.
* Support large output bursts.
* Provide diagnostics for pathological output.

## Integration

ANSI support should integrate with:

* Terminal core
* PTY subsystem
* Screen state
* Cursor state
* Color system
* Input modes
* Renderer
* Scrollback
* Recording
* Diagnostics
* Security system
* Event bus
* Flow View

## Conduit Implementation Principle

ANSI is a protocol, not the terminal state itself.

Conduit should convert ANSI input into a stable internal representation so that the rest of the application does not need to know how the original control sequence was encoded.

This keeps protocol compatibility separate from rendering, configuration, and terminal state management.

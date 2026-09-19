# xterm Terminal Protocol

## Overview

xterm is a widely used terminal emulator environment and protocol extension family.

Conduit provides xterm compatibility because many Unix and Linux applications identify xterm-like terminals and rely on xterm extensions beyond traditional ANSI and VT behavior.

The implementation lives under:

```text
src/terminal_protocols/
└── xterm/
```

## Design Goals

Conduit's xterm support should:

* Provide broad compatibility with xterm-oriented applications.
* Support commonly used xterm extensions.
* Share parsing infrastructure with ANSI and VT protocols.
* Provide configurable terminal capability responses.
* Integrate with OSC, CSI, and DEC mechanisms.
* Preserve security boundaries.
* Avoid claiming support for features that Conduit does not actually implement.

## Architecture

xterm support should act as an extension layer:

```text
ANSI / VT
    ↓
DEC extensions
    ↓
xterm extensions
    ↓
Common terminal commands
    ↓
Terminal state
    ↓
Renderer
```

xterm-specific behavior should not be duplicated when an existing protocol implementation already provides the required functionality.

## Terminal Identification

Conduit should allow the terminal identity advertised to applications to be configured.

The configured identity may affect:

* `$TERM`
* Device attribute responses
* Application feature detection
* Keyboard behavior
* Mouse behavior

The advertised terminal identity should accurately reflect the compatibility mode selected by the user.

## CSI Extensions

Conduit should support commonly used xterm CSI functionality where implemented.

CSI processing should be handled by the shared control-sequence parser.

Parameters should be validated before being applied.

## OSC Support

xterm uses Operating System Command sequences for several terminal features.

Conduit should route OSC handling through the dedicated OSC subsystem:

```text
xterm
  ↓
OSC dispatcher
  ↓
OSC handler
  ↓
Terminal state / UI / security subsystem
```

This prevents xterm-specific code from becoming tightly coupled to individual OSC features.

## Clipboard

xterm-compatible clipboard operations may interact with the host clipboard.

Conduit should apply its clipboard security policy before allowing terminal applications to read from or modify clipboard data.

Clipboard access should be configurable.

## Hyperlinks

xterm-compatible applications may emit terminal hyperlinks.

Conduit should:

* Parse supported hyperlink sequences.
* Store link metadata separately from text.
* Allow user-configured security controls.
* Require appropriate user interaction before opening external resources when policy requires it.

## Window and Title Control

Applications may request changes to:

* Window title
* Icon title
* Tab title
* Session title

Conduit should distinguish between protocol-provided titles and user-defined titles.

Security-sensitive or disruptive title behavior should be configurable.

## Mouse Reporting

xterm mouse protocols should integrate with Conduit's common mouse-reporting subsystem.

The flow should be:

```text
Mouse event
    ↓
Input manager
    ↓
Active terminal mouse mode
    ↓
xterm encoder
    ↓
PTY
```

## Bracketed Paste

Conduit should support xterm-style bracketed paste behavior.

When enabled by the terminal application, pasted content should be wrapped appropriately before being sent to the PTY.

The global multiline-paste security system should remain active.

## Focus Reporting

Where supported, xterm applications can receive terminal focus events.

Conduit should expose focus reporting through terminal modes rather than directly coupling it to GUI implementation details.

## Alternate Screen

xterm applications commonly use alternate-screen behavior.

Conduit should integrate this with the common screen-buffer implementation.

## Graphics Extensions

xterm itself has a broad ecosystem of terminal extensions.

Conduit should keep extensions such as:

* Sixel
* iTerm2 graphics
* Kitty graphics

in separate protocol modules while allowing the xterm compatibility layer to advertise or coordinate their availability.

## Security

xterm sequences are untrusted terminal input.

Conduit should protect against:

* Malformed sequences
* Excessive OSC/DCS payloads
* Unsafe clipboard operations
* Untrusted hyperlinks
* Window manipulation
* Resource exhaustion
* Unsupported control sequences

## Configuration

xterm compatibility should be configurable through the common configuration engine.

Settings may include:

* Terminal identity
* Mouse reporting
* Focus reporting
* Clipboard behavior
* Hyperlinks
* Title handling
* Graphics extensions
* Compatibility behavior

## Testing

Tests should cover:

* CSI extensions
* OSC integration
* Titles
* Clipboard behavior
* Hyperlinks
* Mouse reporting
* Focus reporting
* Bracketed paste
* Alternate screen
* Device identification
* Malformed sequences

## Integration

xterm integrates with:

* ANSI
* VT protocols
* OSC
* CSI
* DCS
* Mouse handling
* Clipboard
* Input
* Graphics
* Renderer
* Security
* Configuration
* Diagnostics
* Event bus

## Conduit Implementation Principle

xterm support should provide application compatibility without turning Conduit into a collection of unrelated protocol implementations.

Each extension should have a clearly defined capability boundary and feed into shared Conduit subsystems.

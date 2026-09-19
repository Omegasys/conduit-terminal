# OSC Terminal Protocol

## Overview

OSC stands for Operating System Command.

OSC sequences are used by terminal applications to communicate information and request terminal-side behavior outside ordinary screen drawing and cursor movement.

Conduit implements OSC as a dedicated subsystem because modern terminal applications use OSC for features including titles, hyperlinks, colors, clipboard operations, notifications, and other terminal extensions.

The implementation lives under:

```text
src/terminal_protocols/
└── osc/
```

## Architecture

OSC processing should be centralized:

```text
PTY output
    ↓
Escape parser
    ↓
OSC detector
    ↓
OSC dispatcher
    ↓
OSC handler
    ↓
Security policy
    ↓
Conduit subsystem
```

This allows individual OSC commands to remain isolated.

## OSC Command Numbers

OSC commands are generally identified by a numeric command parameter.

Conduit should maintain a registry of supported OSC commands.

Each handler should define:

* Command identifier
* Accepted parameters
* Parser behavior
* Maximum payload size
* Required permissions
* Target subsystem
* Security classification
* Failure behavior

## Terminators

OSC sequences may use supported string terminators.

The parser must handle sequences that:

* Arrive in fragments.
* Contain large payloads.
* End using different valid terminators.
* Contain malformed data.

## Window Titles

Supported OSC commands may allow applications to request changes to terminal titles.

Conduit should distinguish:

* Window title
* Tab title
* Session title
* Workspace title
* User-defined title

User configuration should determine which protocol-driven title changes are accepted.

## Hyperlinks

OSC-based hyperlink sequences should create metadata associated with terminal cells rather than changing the text itself.

The internal representation should allow:

* URI
* Display text
* Link boundaries
* Link identifier
* Security state

Opening links should be subject to Conduit's security policy.

## Clipboard Operations

OSC clipboard mechanisms can potentially transfer data between terminal applications and the host.

Because clipboard access can expose sensitive information, Conduit should provide explicit controls.

Possible policies include:

* Disabled
* Write-only
* User-confirmed
* Allowed
* Restricted by security profile

## Notifications

OSC may be used by terminal applications to request notifications.

Conduit should route notification requests through its notification subsystem.

Notifications should be subject to:

* User settings
* Workspace policy
* Security profile
* Session origin
* Rate limits

## Color Queries and Changes

OSC can be used for terminal color operations.

Conduit should translate supported color operations into the common terminal color system.

Changes should respect:

* Active theme
* User overrides
* Workspace configuration
* Profile configuration

## Progress Indicators

Modern terminal applications may use OSC-based conventions to communicate task progress.

Conduit can expose this information through:

* Tab indicators
* Session indicators
* Status bar
* Workspace UI
* Command palette

Progress information should not be treated as trusted application state.

## OSC Registry

The OSC subsystem should use a registry architecture:

```text
OSC command
    ↓
Registry lookup
    ↓
Handler
    ↓
Validation
    ↓
Security policy
    ↓
Action
```

This makes additional OSC support easier to implement.

## Unknown OSC Commands

Unknown commands should not terminate the terminal session.

Conduit should safely consume unsupported OSC data according to parser rules.

Diagnostics may optionally record unsupported commands.

## Resource Limits

OSC payloads can potentially be much larger than ordinary control sequences.

Conduit should enforce:

* Maximum payload sizes
* Maximum processing time
* Maximum queued operations
* Notification rate limits
* Clipboard limits
* Graphics-related limits

## Security

OSC is a major terminal security boundary.

Conduit should consider terminal output untrusted even when it originates from a local application.

Security controls should cover:

* Clipboard access
* Hyperlinks
* Notifications
* Title manipulation
* Color changes
* File-related operations
* Graphics payloads
* Excessive payload sizes

## Live Configuration

OSC security policies should be configurable through the central configuration system.

Changes should be applied using Conduit's live-reload architecture whenever possible.

## Testing

Tests should cover:

* OSC parsing
* Multiple terminators
* Fragmented sequences
* Titles
* Hyperlinks
* Clipboard
* Notifications
* Color operations
* Progress indicators
* Unknown commands
* Oversized payloads
* Malformed payloads
* Security policy enforcement

## Integration

OSC integrates with:

* Terminal core
* xterm
* ANSI/VT parser
* Clipboard
* Notifications
* Themes
* Renderer
* Security
* Configuration
* Event bus
* Diagnostics
* Flow View

## Conduit Implementation Principle

OSC should be treated as a controlled interface between applications running inside a terminal and the terminal emulator itself.

Every OSC feature should therefore pass through validation and the appropriate security policy before interacting with Conduit.

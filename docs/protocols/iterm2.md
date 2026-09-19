# iTerm2 Terminal Protocol

## Overview

The iTerm2 terminal protocol family provides terminal extensions used by iTerm2-compatible applications, particularly for inline graphics and terminal metadata.

Conduit implements supported iTerm2 extensions through a dedicated protocol module.

The implementation lives under:

```text
src/terminal_protocols/
└── iterm2/
```

## Design Goals

Conduit's iTerm2 implementation should:

* Support commonly used iTerm2 terminal extensions.
* Provide compatibility with applications targeting iTerm2-style terminals.
* Keep graphics handling separate from the terminal core.
* Reuse common OSC and escape-sequence infrastructure.
* Enforce resource and security limits.
* Provide configurable capability support.

## Architecture

iTerm2 extensions should flow through shared protocol infrastructure:

```text
PTY output
    ↓
Escape parser
    ↓
iTerm2 detector
    ↓
Command parser
    ↓
Validation
    ↓
Security policy
    ↓
Graphics / terminal subsystem
```

## Inline Images

One of the major iTerm2 extensions supported by Conduit is inline image transfer.

Image payloads should be processed by the graphics subsystem.

```text
iTerm2 image sequence
        ↓
Payload decoder
        ↓
Image validation
        ↓
Graphics resource
        ↓
Placement
        ↓
Renderer
```

## Image Data

The implementation should support the encoding mechanisms required by the supported iTerm2 graphics features.

Payload processing should be incremental where practical.

Conduit should not assume that an entire image arrives in one PTY read.

## Image Limits

Graphics payloads must be bounded.

Conduit should provide configurable limits for:

* Maximum image dimensions
* Maximum encoded payload size
* Maximum decoded size
* Maximum number of active images
* Maximum graphics memory
* Maximum images per session

## Image Placement

The terminal state should track image placement independently of the renderer.

Placement information may include:

* Position
* Dimensions
* Cell association
* Session
* Image identifier
* Visibility
* Lifetime

The renderer converts this state into actual drawing operations.

## Graphics Cache

Conduit may cache decoded images to reduce repeated processing.

The cache should be:

* Per-session or globally managed according to configuration.
* Memory bounded.
* Evictable.
* Observable through diagnostics.

## Terminal Metadata

Supported iTerm2 metadata features should be routed through the appropriate Conduit subsystem.

Metadata should not automatically gain permission to alter arbitrary application state.

## Compatibility Detection

Applications may identify the terminal through environment variables or terminal capability queries.

Conduit should advertise iTerm2 compatibility only when the relevant capabilities are actually available.

## Configuration

iTerm2 protocol support should be configurable.

For example:

```toml
[protocols.iterm2]
enabled = true

[protocols.iterm2.graphics]
enabled = true
max_memory_mb = 256
```

The actual options should be defined by the shared configuration schema.

## Security

Terminal applications should never be assumed trustworthy merely because they run locally.

iTerm2 graphics processing should therefore protect against:

* Oversized payloads
* Invalid image data
* Memory exhaustion
* Decoder failures
* Excessive image creation
* Resource leaks
* Excessive renderer workload

Security profiles should be able to disable graphics independently from other protocol functionality.

## Error Handling

Invalid iTerm2 sequences should not normally terminate the terminal session.

Conduit should:

1. Detect malformed input.
2. Stop processing the invalid operation.
3. Restore parser state safely.
4. Optionally record diagnostics.
5. Continue processing subsequent terminal data.

## Live Reload

Protocol configuration should integrate with Conduit's live configuration system.

Changes should apply without restarting the entire application whenever possible.

If a graphics backend must be recreated, only the smallest affected component should be restarted.

## Testing

Tests should cover:

* Sequence recognition
* Fragmented image data
* Image decoding
* Image placement
* Image deletion
* Size limits
* Memory limits
* Malformed sequences
* Capability detection
* Security restrictions

## Integration

iTerm2 integrates with:

* ANSI
* VT protocols
* OSC
* xterm
* Graphics
* Renderer
* Resource manager
* Configuration
* Security
* Diagnostics
* Event bus
* Flow View

## Conduit Implementation Principle

iTerm2 support should provide useful compatibility without making iTerm2-specific graphics assumptions part of Conduit's general terminal model.

Graphics are resources. Protocol sequences describe those resources. The graphics subsystem manages them, and the renderer displays them.

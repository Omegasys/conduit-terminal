# Kitty Terminal Protocol

## Overview

The Kitty terminal protocol family provides modern terminal extensions, including advanced graphics, keyboard handling, hyperlinks, and other terminal capabilities.

Conduit supports selected Kitty protocols through modular protocol components.

The implementation lives under:

```text
src/terminal_protocols/
└── kitty/
```

Kitty-specific capabilities should be independently detectable and configurable.

## Design Goals

Conduit's Kitty implementation should:

* Support useful Kitty terminal extensions.
* Keep Kitty features modular.
* Avoid coupling the terminal core to Kitty-specific encodings.
* Provide capability detection.
* Protect against malicious or oversized payloads.
* Integrate with Conduit's graphics and input subsystems.
* Allow individual Kitty features to be enabled or disabled.

## Capability Architecture

Kitty support should be capability-based.

```text
Kitty protocol
    ├── Keyboard
    ├── Graphics
    ├── Graphics attributes
    ├── Hyperlinks
    └── Other supported extensions
```

Each capability can have its own implementation and security policy.

## Kitty Graphics Protocol

Kitty graphics allows applications to transmit and display graphical data through the terminal.

Conduit should route Kitty graphics through the graphics subsystem:

```text
PTY
 ↓
Kitty graphics parser
 ↓
Payload validation
 ↓
Graphics resource manager
 ↓
Graphics cache
 ↓
Terminal cell placement
 ↓
Renderer
```

The graphics parser must not directly manipulate GPU resources.

## Graphics Payloads

Graphics data should be:

* Incrementally decoded.
* Size-limited.
* Validated.
* Stored using managed resources.
* Released when no longer referenced.

Conduit should avoid unbounded memory growth from terminal applications repeatedly sending images.

## Graphics Placement

Graphics should be represented separately from ordinary text cells while retaining their terminal-relative placement.

The terminal state may track:

* Image identifier
* Placement identifier
* Position
* Dimensions
* Z-order where applicable
* Visibility
* Source session
* Resource lifetime

## Graphics Cache

Conduit may maintain a graphics cache to avoid repeatedly decoding identical resources.

The cache should have configurable resource limits.

Possible limits include:

* Maximum total memory
* Maximum individual image size
* Maximum image count
* Per-session limits

## Keyboard Protocol

Kitty keyboard functionality should be implemented through the input subsystem.

```text
Physical input
    ↓
Input manager
    ↓
Kitty keyboard mode
    ↓
Protocol encoder
    ↓
PTY
```

This allows Kitty keyboard behavior to coexist with VT/xterm compatibility.

## Hyperlinks

Where Kitty-compatible hyperlink mechanisms overlap with OSC functionality, Conduit should use the common hyperlink representation rather than maintaining duplicate state.

## Capability Detection

Conduit should maintain explicit capability information.

Applications should receive capability responses consistent with the features actually enabled.

Conduit should not advertise disabled or unsupported Kitty features.

## Configuration

Kitty capabilities should be configurable independently.

For example:

```toml
[protocols.kitty]
enabled = true

[protocols.kitty.graphics]
enabled = true
max_memory_mb = 256

[protocols.kitty.keyboard]
enabled = true
```

These settings are illustrative and should ultimately be defined by the configuration schema.

## Security

Kitty graphics and other extensions can consume substantial resources.

Conduit should protect against:

* Oversized payloads
* Excessive image counts
* Memory exhaustion
* Decoder abuse
* Excessive rendering operations
* Malformed graphics commands
* Untrusted image content

Security profiles should be able to disable Kitty graphics while leaving other Kitty functionality enabled.

## Live Reload

Kitty feature configuration should use Conduit's live configuration system.

Disabling a capability should affect future protocol processing without unnecessarily restarting the terminal application.

Changes that require renderer reinitialization should restart only the affected renderer component where possible.

## Testing

Tests should cover:

* Graphics parsing
* Fragmented payloads
* Image placement
* Image deletion
* Cache limits
* Keyboard modes
* Capability detection
* Hyperlink integration
* Malformed commands
* Oversized payloads
* Security restrictions

## Integration

Kitty integrates with:

* ANSI
* VT protocols
* xterm
* OSC
* Input
* Graphics
* Renderer
* Configuration
* Security
* Resource manager
* Diagnostics
* Event bus
* Flow View

## Conduit Implementation Principle

Kitty support should be modular enough that users can enable the functionality they need without requiring every Kitty extension to be active.

Advanced terminal capabilities should behave like controlled protocol modules rather than becoming mandatory parts of the terminal core.

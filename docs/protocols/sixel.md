# Sixel Graphics Protocol

## Overview

Sixel is a terminal graphics protocol originally associated with DEC terminal systems.

Sixel allows graphical information to be transmitted through terminal output and rendered alongside ordinary terminal text.

Conduit supports Sixel through a dedicated graphics protocol module.

The implementation lives under:

```text
src/terminal_protocols/
└── sixel/
```

## Design Goals

Conduit's Sixel implementation should:

* Provide compatibility with Sixel-capable applications.
* Integrate with the shared graphics subsystem.
* Keep graphics decoding separate from terminal rendering.
* Support streaming input.
* Enforce strict resource limits.
* Handle malformed data safely.
* Work across supported rendering backends.

## Architecture

Sixel data should follow a dedicated processing path:

```text
PTY output
    ↓
Escape / DCS parser
    ↓
Sixel detector
    ↓
Sixel decoder
    ↓
Image validation
    ↓
Graphics resource manager
    ↓
Terminal placement
    ↓
Renderer
```

The Sixel decoder should never directly issue GPU commands.

## DCS Integration

Sixel is commonly transported using a Device Control String mechanism.

Conduit should therefore integrate Sixel detection with the common DCS parser.

The general flow is:

```text
DCS
 ↓
Protocol identification
 ↓
Sixel payload
 ↓
Decoder
```

This allows DCS parsing infrastructure to be shared with other terminal protocols.

## Streaming

Sixel data may arrive over multiple PTY reads.

The decoder must maintain state between reads until a complete graphics operation has been received.

The implementation should avoid assuming that:

* A complete image fits inside one read.
* A PTY read boundary corresponds to a protocol boundary.
* All data is immediately available.

## Image Representation

Decoded Sixel data should be converted into Conduit's common graphics resource representation.

This allows Sixel images to coexist with:

* Kitty graphics
* iTerm2 graphics
* Other supported terminal graphics systems

The renderer should not need to know which protocol originally produced the image.

## Color Handling

Sixel graphics can contain palette-based color information.

Conduit should provide a conversion layer between Sixel color information and the internal graphics representation.

The renderer should handle final color-space and display conversion.

## Placement

Sixel graphics need to be associated with terminal coordinates.

The terminal state should track:

* Image bounds
* Cell position
* Image dimensions
* Session
* Visibility
* Lifetime

Placement should remain independent from the rendering backend.

## Graphics Resources

Sixel images should be managed by the graphics resource manager.

Resources should have:

* Unique identifiers
* Memory accounting
* Ownership
* Lifetime information
* Session association
* Cache state

## Resource Limits

Sixel decoding can consume significant CPU and memory resources.

Conduit should enforce limits such as:

* Maximum image width
* Maximum image height
* Maximum decoded size
* Maximum encoded payload
* Maximum active image count
* Maximum graphics memory
* Maximum decoder workload

## Security

Sixel data is untrusted terminal input.

The decoder should protect against:

* Malformed escape sequences
* Invalid palette information
* Oversized images
* Resource exhaustion
* Decoder state exhaustion
* Excessive rendering requests
* Memory allocation attacks

Security profiles should be able to disable Sixel independently.

## Renderer Integration

The Sixel protocol should terminate at the common graphics representation.

```text
Sixel
  ↓
Graphics resource
  ↓
Terminal placement
  ↓
Renderer abstraction
  ├── GPU backend
  └── Software backend
```

This keeps protocol compatibility independent of whether Conduit is using OpenGL, Vulkan, software rendering, or another backend.

## Configuration

Sixel support should be configurable.

For example:

```toml
[protocols.sixel]
enabled = true
max_width = 4096
max_height = 4096
max_memory_mb = 256
```

These settings are illustrative and should ultimately be defined by the configuration schema.

## Live Reload

Sixel configuration should participate in Conduit's live configuration system.

Disabling Sixel should prevent future Sixel operations from being accepted.

Existing graphics resources should be handled according to the configured resource-lifetime policy.

## Diagnostics

The diagnostics system should be able to report:

* Sixel images decoded
* Decode failures
* Current graphics memory
* Active images
* Resource-limit violations
* Parser errors
* Renderer upload failures

## Testing

Tests should cover:

* DCS detection
* Sixel parsing
* Streaming input
* Palette handling
* Image dimensions
* Image placement
* Resource limits
* Malformed data
* Decoder failures
* Security restrictions
* Renderer integration

## Integration

Sixel integrates with:

* DCS
* ANSI
* VT protocols
* Terminal core
* Graphics
* Renderer
* Resource manager
* Configuration
* Security
* Diagnostics
* Event bus
* Flow View

## Conduit Implementation Principle

Sixel should be treated as a graphics transport protocol, not as a special renderer.

The protocol converts terminal output into managed graphics resources. The terminal state determines where those resources belong, and the renderer determines how they are displayed.

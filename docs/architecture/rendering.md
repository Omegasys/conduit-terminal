# Conduit Rendering Architecture

## Overview

The Conduit renderer converts terminal state into visible output.

Rendering is intentionally separated from the terminal core so that terminal sessions can continue operating independently of the graphical rendering backend.

This separation allows Conduit to support multiple rendering technologies, display servers, fallback modes, and future rendering implementations.

## Architecture

The rendering system is located under:

```text
src/renderer/
├── renderer.rs
├── gpu.rs
├── software.rs
├── opengl.rs
├── vulkan.rs
├── wayland.rs
├── x11.rs
├── text.rs
├── cursor.rs
├── images.rs
├── scaling.rs
├── fonts.rs
├── ligatures.rs
└── frame.rs
```

## Rendering Pipeline

The general rendering pipeline is:

```text
PTY Output
   ↓
Terminal Protocol Parser
   ↓
Terminal State
   ↓
Screen Grid
   ↓
Render State
   ↓
Text / Graphics Layout
   ↓
GPU or Software Renderer
   ↓
Window Surface
   ↓
Display Server
```

The renderer should never need to interpret raw shell output directly.

## Terminal State

The terminal core maintains the authoritative terminal state.

This includes:

* Characters
* Cell attributes
* Cursor position
* Selection
* Scrollback
* Alternate screen
* Colors
* Hyperlinks
* Images
* Mouse state
* Terminal modes

The renderer receives a representation of this state.

## Rendering Backends

Conduit should support multiple rendering backends.

### GPU Rendering

GPU rendering should provide the primary high-performance path where supported.

Possible APIs include:

* OpenGL
* Vulkan

The backend should be selected according to platform support and configuration.

### Software Rendering

A software renderer provides a fallback when GPU acceleration is unavailable, unsupported, unstable, or intentionally disabled.

Software rendering is also useful for diagnostics and compatibility testing.

## Display Backends

Linux display support should include:

* Wayland
* X11

The display backend should remain separate from the rendering API.

This allows the same rendering architecture to work across different Linux desktop environments.

## Text Rendering

Terminal text rendering must account for:

* Font selection
* Font fallback
* Unicode
* Combining characters
* Wide characters
* Emoji
* Bidirectional text where supported
* Ligatures
* Variable font features
* Font scaling

The text renderer should convert terminal cells into glyph runs and efficiently render them.

## Font Management

Font management should support:

* User fonts
* System fonts
* Font fallback
* Multiple font families
* Font size changes
* Weight changes
* Italic styles
* Bold styles
* Ligatures
* Live font configuration changes

Font changes should normally be applied without restarting terminal sessions.

## Cell Rendering

The terminal is fundamentally represented as a grid of cells.

A cell can contain:

* Character or glyph
* Foreground color
* Background color
* Style attributes
* Hyperlink information
* Underline information
* Selection state

The renderer converts these cells into visual elements.

## Cursor Rendering

Cursor rendering should support:

* Block cursor
* Beam cursor
* Underline cursor
* Blinking
* Non-blinking
* Custom cursor appearance
* Cursor color
* Cursor opacity

Cursor state is separate from the terminal process.

## Graphics

Conduit can support terminal graphics protocols through the graphics subsystem.

Supported or planned formats include:

* Sixel
* Kitty graphics
* iTerm2 inline images

Graphics should be handled separately from ordinary text cells while remaining synchronized with terminal state.

## Scaling

The renderer should support:

* Fractional scaling
* HiDPI displays
* Per-monitor scaling
* Dynamic display changes
* Font scaling
* Window resizing

Scaling changes should propagate through the renderer without requiring terminal processes to restart.

## Frame Management

The frame subsystem coordinates rendering updates.

A frame may include:

* Terminal cell changes
* Cursor changes
* Image updates
* Selection changes
* UI overlays
* Window state

Conduit should avoid redrawing unchanged content when possible.

## Damage Tracking

The renderer should track portions of the terminal that changed.

Examples include:

* Newly received output
* Cursor movement
* Selection changes
* Scrolling
* Window resizing

Damage tracking reduces unnecessary rendering work.

## Rendering Performance

Performance considerations include:

* Efficient glyph caching
* Texture caching
* Batched drawing
* Incremental updates
* Damage tracking
* GPU acceleration
* Efficient scrollback handling
* Avoiding unnecessary allocations

Large terminal output should not cause the entire application to become unresponsive.

## Renderer Selection

The renderer manager should determine the appropriate backend based on:

* Platform
* Available graphics APIs
* User configuration
* Hardware capabilities
* Compatibility
* Safe mode
* Diagnostics

The selected renderer should be visible through the diagnostics system.

## Live Renderer Changes

Conduit should attempt to change rendering settings without restarting the entire application.

For example:

```text
Font Size
   ↓
Recalculate Layout
   ↓
Rebuild Glyph Metrics
   ↓
Refresh Frame
```

Changing from one graphics backend to another may require restarting the smallest affected rendering component or window.

Terminal processes and unrelated sessions should remain alive whenever possible.

## Rendering Failure

If a GPU renderer fails, Conduit should attempt a controlled fallback.

Example:

```text
GPU Renderer
     ↓
Failure
     ↓
Diagnostics
     ↓
Software Renderer
     ↓
Continue Session
```

This prevents renderer failures from unnecessarily terminating terminal sessions.

## Flow View Integration

The rendering subsystem exposes state to Flow View.

Flow View can represent:

```text
Terminal State
      ↓
Render State
      ↓
Renderer
      ↓
GPU / Software
      ↓
Display
```

This provides a visual representation of how terminal data reaches the screen.

## Security Considerations

The renderer must treat terminal-provided content as untrusted input.

Security-sensitive features include:

* Safe hyperlink handling
* Image resource validation
* Escape sequence handling
* Clipboard protection
* Resource limits
* Malformed input handling

The renderer should never assume that terminal output is trustworthy simply because it originated from a local shell.

## Design Goal

The renderer should make terminal output fast and visually flexible while remaining independent from terminal process execution.

The core principle is:

> The terminal should keep running even when the renderer changes.

# Conduit Architecture

Conduit is designed as a modular terminal platform built around a shared core.

The primary architectural goal is to allow different interfaces and components to interact with the same underlying terminal system without duplicating functionality.

## Core Architecture

The high-level architecture is:

GUI
→ Command API
→ Event Bus
→ Conduit Core

TUI
→ Command API
→ Event Bus
→ Conduit Core

CLI
→ Command API
→ Event Bus
→ Conduit Core

Configuration
→ Configuration Engine
→ Event Bus
→ Affected Components

Plugins
→ Plugin API
→ Permission System
→ Event Bus / Command API

The core then manages:

* Windows
* Tabs
* Panes
* Workspaces
* Sessions
* PTYs
* Processes
* Shells
* Terminal state
* Scrollback
* Environment
* Signals

## Shared Core

The core terminal implementation is located under:

```text
src/core/
```

The core is intentionally separated from the GUI.

This allows Conduit to provide:

* GUI operation
* TUI operation
* CLI management
* Headless operation
* Remote management
* Automated testing

without requiring multiple independent terminal implementations.

## GUI

The graphical interface is located under:

```text
src/gui/
```

The GUI provides:

* Traditional desktop menus
* Optional toolbar
* Tab bar
* Sidebar
* Status bar
* Terminal panes
* Settings
* Configuration editor
* Command palette
* Security Center
* Flow View
* Workspace management
* Profile management
* Theme management
* Diagnostics
* Accessibility features

The primary menu structure is:

File
Edit
View
Terminal
Tabs
Panes
Session
Tools
Help

## TUI

The terminal user interface is located under:

```text
src/tui/
```

The TUI exposes important Conduit functionality without requiring the graphical interface.

The TUI should use the same commands, events, configuration engine, resource system, and core components as the GUI.

## CLI

The command-line interface is located under:

```text
src/cli/
```

The CLI provides scripting and automation access to Conduit.

Examples include:

```bash
conduit config show
conduit theme list
conduit profile list
conduit workspace list
conduit diagnostics
```

The CLI should communicate through the same command API used by the GUI and TUI.

## Command API

Commands provide a common interface between Conduit components.

The general model is:

Interface
→ Command
→ Command Dispatcher
→ Core or Subsystem

This prevents GUI-specific logic from becoming embedded inside core functionality.

## Event Bus

The event system is located under:

```text
src/events/
```

The event bus provides communication between independent components.

Events can represent things such as:

* Configuration changes
* Theme changes
* Profile changes
* Workspace changes
* Terminal output
* Session changes
* Pane changes
* Tab changes
* Connection changes
* Plugin events
* Diagnostics events
* Security events
* Renderer changes

The event system allows components to react without tightly coupling every subsystem to every other subsystem.

## Configuration Engine

The configuration engine is located under:

```text
src/config_engine/
```

Its responsibilities include:

* Loading configuration
* Parsing TOML
* Schema validation
* Default values
* Profiles
* Workspaces
* Configuration diffs
* Transactions
* Rollback
* Migration
* Serialization
* Configuration state
* Error reporting

The main configuration file is:

```text
~/.config/conduit/config.toml
```

## Resource System

The resource system is located under:

```text
src/resources/
```

It provides automatic discovery of resources such as:

* Themes
* Profiles
* Workspaces

The general process is:

Resource Directory
→ Discovery
→ Parse
→ Validate
→ Register
→ Event
→ Component Update

Conduit should not require a hardcoded list of every resource.

If a supported resource is placed in an appropriate resource directory and passes validation, Conduit can discover it.

## Resource Precedence

Resources may exist at multiple levels.

A typical precedence model is:

User resources
→ Local application resources
→ System resources
→ Built-in defaults

This allows users to override system-provided resources without modifying system files.

## Live Reload

Live reload is located under:

```text
src/live/
```

The live system includes:

* File watching
* Change detection
* Dependency tracking
* Reload management
* Live state
* Restart policy
* Component restart
* Rollback
* Resource watching

The goal is to avoid restarting Conduit unnecessarily.

For example, changing a theme should normally reload the theme rather than restart the terminal.

Changing a setting that only affects the renderer should preferably reload the renderer component rather than the entire application.

## Smallest-Required-Restart Principle

When a change cannot be applied live, Conduit should determine the smallest affected component.

Conceptually:

Configuration Change
→ Dependency Graph
→ Determine Affected Components
→ Attempt Live Update
→ If Necessary, Restart Smallest Component
→ Restore State

The application should avoid full restarts whenever practical.

## Windows, Tabs, and Panes

Windows are managed under:

```text
src/windows/
```

Tabs are managed under:

```text
src/tabs/
```

Panes are managed under:

```text
src/panes/
```

A typical hierarchy is:

Window
→ Tab
→ Pane
→ Terminal Session
→ PTY
→ Shell / Process

Multiple windows can contain multiple tabs.

Tabs can contain multiple panes.

Panes contain terminal sessions.

## Workspaces

Workspaces are located under:

```text
src/workspaces/
```

A workspace can describe a complete working environment.

Possible workspace state includes:

* Window layout
* Tabs
* Panes
* Shells
* Working directories
* Connections
* Profiles
* Themes
* Session state

Workspace files can be discovered automatically.

## Terminal Core

The terminal core handles:

* PTY creation
* Process management
* Terminal state
* Screen buffers
* Cursor state
* Scrollback
* Environment
* Signals
* Session lifecycle

Terminal protocol parsing is separated into:

```text
src/terminal_protocols/
```

This keeps protocol processing independent from rendering.

## Terminal Protocols

Conduit is designed to support a broad range of terminal protocols.

Examples include:

* ANSI
* VT100
* VT220
* VT320
* VT420
* VT520
* xterm
* CSI
* SGR
* OSC
* DCS
* DEC sequences
* Mouse protocols
* Bracketed paste
* Kitty graphics
* iTerm2 graphics
* Sixel

Protocol parsing should produce structured terminal events that can be consumed by the terminal core and renderer.

## Renderer

Rendering is located under:

```text
src/renderer/
```

The renderer supports different rendering strategies and platform backends.

Potential backends include:

* GPU rendering
* Software rendering
* OpenGL
* Vulkan
* Wayland
* X11

Rendering should remain separate from terminal state and protocol parsing.

## Shell Integration

Shell integration is located under:

```text
src/shell/
```

Supported shells include:

* Bash
* Zsh
* Fish
* PowerShell

Shell integration can provide:

* Current directory
* Command boundaries
* Exit status
* Command duration
* Prompt information
* Environment information
* Shell history integration

## History Architecture

Conduit distinguishes between native shell history and Conduit metadata.

For example:

```text
~/.bash_history
```

remains owned by Bash.

Conduit can read and integrate with shell history while maintaining its own metadata.

Conduit-specific data can be stored under:

```text
~/.local/share/conduit/history/
```

Metadata may include:

* Timestamp
* Shell
* Session
* Workspace
* Pane
* Working directory
* Command duration
* Exit status
* Recording reference

Sensitive data handling and optional redaction are handled by the history subsystem.

## Search

Search functionality is located under:

```text
src/search/
```

Search can operate across:

* Terminal output
* Scrollback
* Panes
* Tabs
* Sessions
* Recordings
* Command history

Supported search modes can include:

* Case-sensitive
* Case-insensitive
* Whole-word
* Regular expressions

## Clipboard

Clipboard functionality is located under:

```text
src/clipboard/
```

Features include:

* Clipboard management
* Primary selection
* Clipboard history
* Paste protection
* Large-paste handling
* Sensitive-data handling
* Security controls

## Connections

Connection management is divided between:

```text
src/connections/
src/remote/
```

Supported connection types include:

* Local shells
* SSH
* Serial
* Containers
* Docker
* Podman
* Remote sessions

Remote functionality should integrate with Conduit's normal session, tab, pane, and workspace model.

## Plugins

Plugins are located under:

```text
src/plugins/
```

The plugin system includes:

* Plugin discovery
* Loading
* Lifecycle management
* Permissions
* Sandboxing
* Dependencies
* Manifests
* Registry
* Updates
* Plugin API

Plugins should not automatically receive unrestricted access to the system.

Permissions should be explicit and enforceable.

## Security

Security functionality is located under:

```text
src/security/
```

Security features include:

* Plugin permissions
* Sandboxing
* Filesystem restrictions
* Environment restrictions
* Clipboard protection
* Hyperlink protection
* Escape-sequence security
* Safe mode
* Hardened mode
* Auditing
* Security policies

## Flow View

Flow View is a visual representation of Conduit's internal information flow.

A conceptual flow is:

Shell
→ Process
→ PTY
→ Terminal
→ Pane
→ Renderer

Other events can also be represented:

Configuration
→ Configuration Engine
→ Event Bus
→ Component

The Flow View is intended primarily as a diagnostic and visualization system.

## Recording

Terminal recording is located under:

```text
src/recording/
```

Recording can capture terminal sessions for later replay or export.

Recording metadata can include:

* Session information
* Terminal dimensions
* Timing information
* Shell information
* Workspace information
* Recording format

Privacy controls should allow users to control what is recorded and how recordings are stored.

## Diagnostics

Diagnostics are located under:

```text
src/diagnostics/
```

The diagnostic system can expose information about:

* Renderer
* GPU
* Memory
* PTYs
* Protocols
* Configuration
* Resources
* Plugins
* Performance
* Crashes
* Sessions

Diagnostics can be accessed through the GUI, TUI, and CLI.

## IPC

Inter-process communication is located under:

```text
src/ipc/
```

IPC allows external tools and Conduit processes to communicate.

Possible uses include:

* Opening a new window
* Creating tabs
* Creating panes
* Switching workspaces
* Updating configuration
* Querying diagnostics
* Sending commands
* Receiving events

## Platform Layer

Linux-specific functionality is located under:

```text
src/platform/linux/
```

Platform functionality includes:

* Wayland
* X11
* D-Bus
* Desktop integration
* Notifications
* Filesystem integration
* Process management

## Architectural Principle

Conduit should follow this general rule:

Simple by default.

Powerful when opened up.

Configurable everywhere.

Modular underneath.

Live whenever possible.

Safe when functionality crosses trust boundaries.

The architecture should allow individual subsystems to evolve without requiring the entire terminal emulator to be rewritten.

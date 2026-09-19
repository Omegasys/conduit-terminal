# Conduit Architecture Overview

Conduit is a modular Linux terminal emulator built around a shared terminal core, event-driven communication, live configuration, automatic resource discovery, and multiple user interfaces.

The primary goal is to keep the terminal itself independent from the interface used to control it.

Conduit can therefore provide a GUI, TUI, CLI, plugins, and external IPC clients without maintaining separate implementations of the terminal engine.

## Architectural Model

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

Plugins
→ Plugin API
→ Permission System
→ Command API / Event Bus

Configuration
→ Configuration Engine
→ Validation
→ Diff
→ Event Bus
→ Affected Components

The core then manages terminal sessions, PTYs, processes, shells, windows, tabs, panes, workspaces, terminal state, and scrollback.

## Major Subsystems

Conduit's major subsystems include:

* Core terminal engine
* Windows
* Tabs
* Panes
* Workspaces
* Configuration engine
* Resource system
* Live reload system
* Event bus
* GUI
* TUI
* CLI
* Renderer
* Terminal protocols
* Shell integration
* Clipboard
* Graphics
* Remote connections
* Multiplexer integration
* Plugins
* Security
* Command history
* Search
* Recording
* Diagnostics
* Accessibility
* IPC
* Platform integration

## Shared Core Principle

The GUI, TUI, and CLI should not contain independent implementations of terminal functionality.

Instead, they communicate with the same core through commands and events.

For example:

GUI
→ Create Tab Command
→ Command Dispatcher
→ Tab Manager
→ Event Bus
→ GUI Update

The same operation can be performed through the CLI:

CLI
→ Create Tab Command
→ Command Dispatcher
→ Tab Manager
→ Event Bus
→ State Update

This keeps behavior consistent between interfaces.

## Command API

Commands represent requested actions.

Examples include:

* Create window
* Close window
* Create tab
* Close tab
* Split pane
* Resize pane
* Switch workspace
* Create session
* Connect to SSH
* Change theme
* Load profile
* Start recording
* Stop recording
* Query diagnostics

Commands should describe what should happen rather than how a particular interface displays the result.

## Event Bus

The event bus provides communication between subsystems.

For example, changing a theme can produce a theme-changed event.

The GUI, TUI, diagnostics system, and other interested components can subscribe to that event without being directly connected to the theme loader.

## Configuration

Configuration is handled by the configuration engine.

The main user configuration file is:

```text
~/.config/conduit/config.toml
```

Configuration can be changed through:

* GUI Settings
* Configuration editor
* TUI
* CLI
* Direct TOML editing

All methods ultimately use the same configuration system.

## Resource Discovery

Conduit treats themes, profiles, and workspaces as discoverable resources.

A resource follows this general path:

Resource File
→ Discovery
→ Parsing
→ Validation
→ Registration
→ Event
→ Component Update

This allows users to add resources without modifying Conduit itself.

## Live Reload

Conduit is designed around live updates.

A configuration or resource change follows this model:

Change Detected
→ Parse
→ Validate
→ Calculate Difference
→ Determine Dependencies
→ Apply Live Update

If the change cannot be applied safely, Conduit determines whether a smaller component can be restarted.

The entire application should only restart when there is no smaller safe alternative.

## Dependency Graph

The live system maintains relationships between components.

For example:

Theme
→ Appearance
→ Renderer

A change to a theme may therefore require the renderer to refresh, but should not normally require restarting the terminal session.

Similarly:

Font Configuration
→ Font System
→ Renderer

Only the components affected by the change should be touched.

## Terminal Data Flow

A typical terminal session follows this path:

Shell
→ Process
→ PTY
→ Terminal Parser
→ Terminal State
→ Pane
→ Renderer
→ Display

Input follows the reverse direction:

Keyboard / Mouse
→ Input System
→ Terminal Session
→ PTY
→ Shell / Process

## Flow View

Conduit includes a Flow View intended to visualize this architecture.

A simplified representation is:

Shell
→ Process
→ PTY
→ Terminal
→ Pane
→ Renderer

Other flows can also be represented:

Configuration
→ Configuration Engine
→ Event Bus
→ Component

Plugin
→ Permission System
→ Plugin API
→ Event Bus / Command API

## Security Boundaries

Security-sensitive components are separated from ordinary application logic.

Examples include:

* Plugin permissions
* Plugin sandboxing
* Filesystem access
* Clipboard access
* Hyperlinks
* Terminal escape sequences
* Environment access
* Remote connections

The security system can enforce policies before sensitive operations are allowed.

## Plugin Architecture

Plugins communicate with Conduit through defined APIs.

A plugin should not need direct access to internal implementation details.

The plugin system manages:

* Discovery
* Loading
* Permissions
* Sandboxing
* Dependencies
* Lifecycle
* Updates

Plugins can extend Conduit without modifying the core source tree.

## Remote Sessions

Remote connections use the same session model as local terminal sessions.

For example:

SSH
→ Remote Process
→ PTY
→ Terminal Session
→ Pane

The user can therefore place local and remote sessions in the same workspace.

## History Architecture

Conduit distinguishes between native shell history and Conduit-specific metadata.

For example, Bash continues to own:

```text
~/.bash_history
```

Conduit can read and integrate that history while maintaining additional metadata under:

```text
~/.local/share/conduit/history/
```

This metadata can associate commands with:

* Sessions
* Panes
* Tabs
* Workspaces
* Working directories
* Timestamps
* Exit status
* Command duration

## Design Principles

Conduit follows several core principles.

### One Core

There should be one authoritative terminal implementation.

### Multiple Interfaces

GUI, TUI, CLI, plugins, and IPC clients should access the same core.

### Modular Components

Subsystems should have clear boundaries and well-defined interfaces.

### Live Whenever Possible

Configuration and resource changes should be applied without restarting whenever technically safe.

### Smallest Required Restart

If a restart is unavoidable, restart the smallest affected component.

### User Ownership

User configuration, themes, profiles, workspaces, and shell history should remain accessible as normal files where appropriate.

### Explicit Security

Capabilities that cross security boundaries should be explicit and controllable.

### Diagnostics

The system should make its internal state observable enough to diagnose problems without requiring users to guess what Conduit is doing.

## Directory Mapping

The primary architecture is represented by:

```text
src/core/
src/windows/
src/tabs/
src/panes/
src/workspaces/
src/config_engine/
src/resources/
src/live/
src/events/
src/gui/
src/tui/
src/cli/
src/renderer/
src/terminal_protocols/
src/shell/
src/clipboard/
src/graphics/
src/remote/
src/connections/
src/plugins/
src/security/
src/command/
src/history/
src/search/
src/recording/
src/diagnostics/
src/accessibility/
src/flow/
src/ipc/
src/platform/
```

Each subsystem should remain independently understandable while communicating through shared interfaces.

## Long-Term Goal

The architecture is intended to allow Conduit to grow from a terminal emulator into a complete terminal workspace platform without turning the project into one tightly coupled application.

The core terminal should remain stable while interfaces, resources, plugins, rendering backends, integrations, and developer tools continue to evolve around it.

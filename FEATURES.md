# Conduit Features

Conduit is a modular Linux terminal emulator designed to combine a traditional terminal experience with extensive configuration, session management, resource discovery, and developer-oriented tools.

## Interfaces

Conduit provides multiple interfaces over the same underlying core.

### GUI

The graphical interface provides:

* Traditional desktop window
* Menu bar
* Optional toolbar
* Tab bar
* Sidebar
* Status bar
* Terminal panes
* Settings application
* Configuration editor
* Command palette
* Security Center
* Diagnostics
* Flow View
* Workspace management
* Profile management
* Theme management

### TUI

The TUI provides terminal-based access to major Conduit functionality.

### CLI

The CLI provides scripting and automation access.

GUI, TUI, and CLI operations use the same command and event infrastructure.

## Windows

Conduit supports:

* Multiple windows
* Window restoration
* Window placement
* Persistent window state
* Window-specific settings

## Tabs

Tabs support:

* Multiple terminal sessions
* Tab reordering
* Tab titles
* Tab icons
* Context menus
* Tab restoration
* Tab-specific state

## Panes

Panes support:

* Horizontal splitting
* Vertical splitting
* Pane resizing
* Pane zooming
* Pane swapping
* Layout management
* Pane synchronization

## Workspaces

Workspaces provide reusable terminal environments.

A workspace can contain:

* Tabs
* Panes
* Layouts
* Working directories
* Shells
* Remote connections
* Profiles
* Themes
* Session state

Workspaces can be saved and restored.

## Configuration

Conduit uses TOML configuration.

The primary file is:

```text
~/.config/conduit/config.toml
```

Configuration can be modified through:

* GUI Settings
* Configuration editor
* CLI
* Direct editing

All interfaces use the same configuration engine.

## Configuration Editor

The built-in configuration editor provides:

* TOML editing
* Syntax highlighting
* Autocomplete
* Schema information
* Validation
* Diagnostics
* Documentation
* Preview
* Apply
* Revert

## Profiles

Profiles allow users to define reusable configurations.

Examples include:

* Default
* Minimal
* Development
* Hardened
* SSH
* Custom

Profiles can be automatically discovered from supported resource directories.

## Themes

Themes are TOML resources.

Themes can define:

* Colors
* Fonts
* Cursor appearance
* Terminal appearance
* UI appearance
* Selection appearance
* Status indicators
* Other supported visual settings

User themes can be placed in:

```text
~/.config/conduit/themes/
```

Conduit can automatically discover new valid theme files.

## Automatic Resource Discovery

Conduit can automatically discover supported resources.

Supported resource types include:

* Themes
* Profiles
* Workspaces

The discovery system:

1. Searches configured resource directories.
2. Identifies supported files.
3. Parses the resource.
4. Validates the resource.
5. Registers the resource.
6. Notifies affected components.

This allows users to add resources without modifying Conduit source code.

## Live Resource Reloading

Supported resources can be monitored for changes.

For example:

```text
~/.config/conduit/themes/my-theme.toml
```

can be created, modified, or removed while Conduit is running.

When practical, changes are applied immediately.

Invalid changes should not destroy the currently valid state.

## Live Configuration

Conduit is designed around live configuration.

The configuration flow is:

Configuration Change
→ Parse
→ Validate
→ Diff
→ Determine Dependencies
→ Apply Change

If a change cannot be applied directly, Conduit determines whether a smaller component can be restarted.

## Command Palette

The Command Palette provides searchable access to Conduit commands.

It can expose:

* Terminal commands
* Tab commands
* Pane commands
* Workspace commands
* Configuration commands
* Theme commands
* Profile commands
* Plugin commands
* Diagnostic commands
* Security commands

## Search

Terminal search supports:

* Normal text search
* Regular expressions
* Case-sensitive search
* Case-insensitive search
* Whole-word matching
* Scrollback search
* Pane search
* Tab search
* Session search
* Recording search
* Command-history search

## Shell Integration

Supported shells include:

* Bash
* Zsh
* Fish
* PowerShell

Shell integration can provide:

* Current working directory
* Command boundaries
* Exit status
* Command duration
* Prompt state
* Environment information
* History integration

## Shell History Integration

Conduit does not replace native shell history.

For example, Bash continues to use:

```text
~/.bash_history
```

Conduit can integrate with native shell history and provide additional functionality such as:

* Search
* Filtering
* Metadata
* Session association
* Workspace association
* Command duration
* Exit status
* Optional redaction

## Conduit History Metadata

Conduit can maintain application-specific history information under:

```text
~/.local/share/conduit/history/
```

This can associate commands with:

* Sessions
* Panes
* Tabs
* Workspaces
* Working directories
* Timestamps
* Shell types
* Exit status
* Duration

## Clipboard

Clipboard features include:

* Normal clipboard
* Primary selection
* Clipboard history
* Paste protection
* Large paste detection
* Sensitive-data handling
* Security controls

## Terminal Protocols

Conduit is designed to support:

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

## Graphics

Supported graphics protocols can include:

* Sixel
* Kitty graphics
* iTerm2 graphics

## Rendering

Rendering options include:

* GPU rendering
* Software rendering
* OpenGL
* Vulkan
* Wayland
* X11

Additional rendering features include:

* Font selection
* Font scaling
* Ligatures
* Image scaling
* Cursor rendering
* Display scaling

## Remote Connections

Conduit can provide connection management for:

* Local sessions
* SSH
* SFTP
* SCP
* Serial connections
* Forwarding
* Remote hosts
* Container environments

## Container Integration

Potential integrations include:

* Docker
* Podman
* Kubernetes

Container sessions can be managed alongside normal terminal sessions.

## Multiplexer Integration

Conduit can integrate with existing terminal multiplexers such as:

* tmux
* GNU Screen
* Zellij

This allows Conduit to coexist with existing terminal session workflows.

## Plugins

The plugin system supports:

* Plugin discovery
* Plugin loading
* Plugin manifests
* Permissions
* Sandboxing
* Dependencies
* Updates
* Plugin lifecycle management
* Plugin APIs

Example plugins include:

* Git
* SSH
* Docker
* Kubernetes
* File preview

## Security Center

The Security Center provides visibility into security-related settings.

It can display:

* Plugin permissions
* Filesystem permissions
* Clipboard restrictions
* Hyperlink restrictions
* Escape-sequence restrictions
* Security profile
* Active warnings
* Audit information

## Security Modes

Conduit can provide configurable security modes such as:

* Standard
* Minimal
* Hardened
* Safe
* Custom

Security settings should be explicit and configurable.

## Safe Mode

Safe mode can be used to diagnose problems caused by:

* Plugins
* Custom themes
* Custom profiles
* Custom workspaces
* Configuration
* Optional integrations

## Terminal Recording

Conduit can record terminal sessions.

Features include:

* Recording
* Replay
* Metadata
* Compression
* Export
* Privacy controls

## Diagnostics

Diagnostics can report:

* CPU usage
* Memory usage
* GPU information
* Renderer information
* PTY information
* Protocol information
* Configuration state
* Resource state
* Plugin state
* Session state
* Performance information

## Flow View

Flow View visualizes Conduit's internal information flow.

Example:

Shell
→ Process
→ PTY
→ Terminal
→ Pane
→ Renderer

It can also visualize configuration and event flow.

## Accessibility

Accessibility features include support for:

* Screen readers
* High contrast
* Font scaling
* Keyboard navigation
* Reduced motion
* Accessibility announcements

## Modes

Conduit can provide different operating modes:

* Full
* Minimal
* Terminal-only
* Fullscreen
* Distraction-free
* Safe
* Experimental

## Notifications

Notification features include:

* Desktop notifications
* Terminal bell
* Visual notifications
* Activity notifications
* Urgency indicators

## File Integration

The integrated file manager can provide:

* File browsing
* File previews
* Path navigation
* Bookmarks
* Permission information
* Terminal integration

## IPC

Conduit provides IPC for external automation and integration.

Possible operations include:

* Create window
* Create tab
* Create pane
* Switch workspace
* Execute Conduit commands
* Query state
* Query diagnostics
* Subscribe to events

## Packaging

Conduit is designed to support multiple Linux distribution formats:

* Debian packages
* RPM packages
* Arch packages
* Flatpak
* Snap
* AppImage

## Design Philosophy

Conduit follows several architectural principles:

* One core
* Multiple interfaces
* Modular subsystems
* Live configuration
* Automatic resource discovery
* Minimal required restarts
* Explicit security boundaries
* Native shell compatibility
* User-configurable behavior
* Developer-friendly diagnostics
* Extensible plugins

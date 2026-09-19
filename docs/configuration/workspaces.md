# Conduit Workspace Configuration

## Overview

Workspace configuration allows each Conduit workspace to maintain its own environment.

A workspace can define:

* Layout
* Windows
* Tabs
* Panes
* Sessions
* Profile
* Theme
* Working directories
* Connection settings
* UI preferences
* Keybindings
* Other supported configuration

Workspace configuration integrates directly with the workspace system.

## Workspace Locations

User workspace resources are stored under:

```text
~/.config/conduit/workspaces/
```

System-provided workspaces may be installed under:

```text
/usr/share/conduit/workspaces/
```

## Example

```toml
[workspace]
name = "Development"
profile = "development"
theme = "cyberpunk"

[layout]
type = "split"

[[panes]]
id = "editor"
working_directory = "~/projects"

[[panes]]
id = "server"
working_directory = "~/projects"
```

The exact schema can evolve as the workspace system develops.

## Automatic Discovery

Workspace files are automatically discovered from supported directories.

A user can add:

```text
~/.config/conduit/workspaces/project.toml
```

and Conduit can make the workspace available without modifying the application.

## Live Discovery

The resource watcher detects:

* New workspace files
* Modified workspace files
* Renamed workspace files
* Removed workspace files

The workspace registry is updated automatically.

## Active Workspace Changes

If an active workspace file changes, Conduit compares the new definition against the current state.

Depending on the change, Conduit may:

* Apply it immediately.
* Update future sessions.
* Reconfigure a component.
* Rebuild a UI element.
* Reload a workspace component.
* Request confirmation.
* Defer the change.

Running terminal processes should not be terminated unnecessarily.

## Workspace Hierarchy

Workspace configuration sits above individual sessions.

A simplified relationship is:

```text
Global Configuration
        ↓
Profile
        ↓
Workspace
        ↓
Window
        ↓
Tab
        ↓
Pane
        ↓
Session
```

More specific configuration can override broader configuration according to the configuration engine's precedence rules.

## Profiles

A workspace can select a profile.

Example:

```toml
[workspace]
profile = "development"
```

The profile provides reusable defaults while the workspace can define environment-specific overrides.

## Themes

A workspace can select a theme:

```toml
[workspace]
theme = "nord"
```

Theme resources are loaded through the resource system.

## Working Directories

Workspace panes can specify starting directories.

For example:

```toml
[[panes]]
working_directory = "~/projects/conduit"
```

The directory should be validated before session creation.

## Sessions

A workspace can describe sessions that should be created or restored.

A workspace definition should distinguish between:

* Requested session configuration
* Existing running sessions
* Restorable session metadata

A workspace file should not assume that an arbitrary process can be restored exactly as it was.

## Remote Connections

Workspaces can associate panes or sessions with connection profiles.

For example:

```text
Development
├── Local Shell
├── Build Server
└── Test Container
```

Credentials should not be stored directly in ordinary workspace files.

## Layout

Workspace layout may describe:

* Window arrangement
* Tab order
* Pane splits
* Pane sizes
* Active tab
* Active pane

Layout changes can be applied independently from session processes where possible.

## Keybindings

Workspaces may optionally define workspace-specific keybindings.

These should be merged with global keybindings through the configuration engine.

## Workspace UI

The GUI should provide:

* Workspace selector
* Workspace sidebar
* Create
* Save
* Save As
* Rename
* Duplicate
* Import
* Export
* Delete
* Restore

## Command Palette

Workspace actions should be available through the command palette.

Examples include:

```text
Open Workspace
Save Workspace
Create Workspace
Switch Workspace
Reload Workspace
Export Workspace
```

## CLI

Example commands:

```text
conduit workspace list
conduit workspace create <name>
conduit workspace open <name>
conduit workspace save <name>
conduit workspace reload <name>
conduit workspace export <name>
conduit workspace import <file>
conduit workspace delete <name>
```

## Workspace Templates

Workspace templates can provide starting layouts.

Examples:

* Development
* SSH
* Monitoring
* Containers
* Security
* Minimal
* Custom

Templates are resources and can be discovered automatically.

## Workspace Switching

Switching workspaces should normally change the visible environment without destroying unrelated sessions.

This allows multiple environments to remain active.

## Workspace Restoration

Restoration should proceed incrementally:

```text
Load Definition
    ↓
Validate
    ↓
Create Layout
    ↓
Create / Attach Sessions
    ↓
Apply Profile
    ↓
Apply Theme
    ↓
Restore UI State
```

Failure in one pane should not necessarily prevent the rest of the workspace from loading.

## Security

Workspace configuration should not automatically execute arbitrary commands.

A workspace is configuration data, not executable code.

Commands requested through workspace automation must pass through the normal command and security systems.

## Design Goal

Workspace configuration turns Conduit into a persistent working environment.

The principle is:

> A workspace remembers the environment, not just the window.

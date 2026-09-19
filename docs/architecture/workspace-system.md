# Conduit Workspace System

## Overview

A workspace represents a complete working environment within Conduit.

A workspace can contain windows, tabs, panes, sessions, profiles, connections, and associated configuration.

Workspaces allow users to maintain separate environments without manually reconstructing their terminal layout.

Examples include:

* Development
* Administration
* SSH
* Security
* Personal
* Testing
* Project-specific environments

## Architecture

Workspace functionality is implemented across:

```text
src/workspaces/
src/windows/
src/tabs/
src/panes/
src/core/session.rs
src/resources/
```

## Workspace Structure

A simplified workspace hierarchy is:

```text
Workspace
├── Window
│   ├── Tab
│   │   ├── Pane
│   │   │   └── Session
│   │   └── Pane
│   │       └── Session
│   └── Tab
│       └── Pane
│           └── Session
└── Metadata
```

## Workspace State

A workspace can contain:

* Name
* Identifier
* Layout
* Windows
* Tabs
* Panes
* Sessions
* Profiles
* Theme
* Working directories
* Connection definitions
* Keybindings
* UI preferences
* Metadata

## Workspace Files

Workspace resources can be stored as TOML files.

For example:

```text
~/.config/conduit/workspaces/
├── development.toml
├── ssh.toml
└── security.toml
```

Conduit should automatically discover valid workspace files.

## Resource Discovery

The resource system watches workspace directories.

When a workspace is:

* Added
* Modified
* Renamed
* Removed

Conduit should update its workspace registry automatically.

Invalid workspace files should be reported without corrupting currently active workspaces.

## Creating a Workspace

A workspace can be created through:

* GUI
* TUI
* CLI
* Command palette
* Configuration editor
* Plugin API

The creation process can capture the current layout or start from a template.

## Saving

Workspace saving should preserve enough information to reconstruct the workspace.

This may include:

* Window geometry
* Tab ordering
* Pane layout
* Session metadata
* Working directories
* Selected profiles
* Theme
* UI state

## Restoration

Workspace restoration should occur in stages:

```text
Load Workspace Definition
        ↓
Validate
        ↓
Create Windows
        ↓
Create Tabs
        ↓
Create Panes
        ↓
Restore Sessions
        ↓
Apply Profiles
        ↓
Apply Theme
        ↓
Workspace Ready
```

Failures should be isolated whenever possible.

## Session Restoration

A workspace can request session restoration.

However, restoring a shell process is not always equivalent to restoring the session's metadata.

Conduit should therefore distinguish:

* Layout restoration
* Session recreation
* Remote reconnection
* Process restoration

## Switching Workspaces

Switching workspaces should not necessarily terminate existing sessions.

A workspace switch may simply change which environment is visible.

For example:

```text
Workspace A
   ↓
Hide / Detach View
   ↓
Workspace B
   ↓
Display B
```

Sessions belonging to Workspace A may continue running in the background according to configuration.

## Workspace Isolation

Workspaces can provide logical separation.

For example, a security-focused workspace may use:

* Hardened profile
* Restricted plugins
* Reduced clipboard access
* Restricted hyperlinks
* Recording disabled
* Different shell settings

Workspace isolation is configuration-level isolation unless stronger operating-system isolation is explicitly configured.

## Workspace Profiles

A workspace may select a profile.

Example:

```toml
[workspace]
name = "Development"
profile = "development"
theme = "cyberpunk"
```

The profile supplies defaults while workspace-specific settings can override them according to configuration precedence.

## Workspace Templates

Templates can simplify workspace creation.

Possible templates include:

* Empty
* Development
* SSH
* Monitoring
* Security
* Container
* Custom

Templates are data resources and can be discovered automatically.

## Workspace UI

The GUI workspace interface should provide:

* Workspace selector
* Workspace list
* Create
* Rename
* Duplicate
* Save
* Restore
* Delete
* Import
* Export

The sidebar should provide quick workspace navigation.

## CLI Integration

Examples:

```text
conduit workspace list
conduit workspace create <name>
conduit workspace open <name>
conduit workspace save <name>
conduit workspace rename <old> <new>
conduit workspace delete <name>
conduit workspace export <name>
conduit workspace import <file>
```

## TUI Integration

The TUI should provide equivalent functionality using keyboard-driven controls.

Workspace selection should also be available through the command palette.

## Live Workspace Changes

Changes to workspace resource files should be detected automatically.

Conduit should determine whether a change can be:

* Applied immediately
* Applied to future sessions
* Applied after a component reload
* Applied after workspace recreation

Existing running sessions should not be unnecessarily terminated.

## Workspace Events

Important events include:

* WorkspaceCreated
* WorkspaceLoaded
* WorkspaceSaved
* WorkspaceChanged
* WorkspaceSwitched
* WorkspaceRestored
* WorkspaceClosed
* WorkspaceDeleted
* WorkspaceError

## Security

Workspace definitions should not silently execute arbitrary commands merely because they were loaded.

If a workspace contains executable or privileged actions, those actions should be subject to the normal command and security policies.

Workspace files should therefore be treated as configuration, not automatically trusted executable code.

## Design Goal

The workspace system should allow Conduit to become a reusable working environment rather than merely a terminal window.

The guiding principle is:

> A workspace is a saved flow of tools, terminals, layouts, and context.

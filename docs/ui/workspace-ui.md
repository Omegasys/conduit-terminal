# Conduit Workspace UI

## Overview

The Workspace UI provides a unified interface for managing Conduit workspaces, sessions, tabs, panes, profiles, themes, and associated resources.

A workspace represents a higher-level environment containing one or more terminal sessions and their layout.

## Workspace Model

A workspace can contain:

* Tabs
* Panes
* Terminal sessions
* Profiles
* Themes
* Working directories
* Environment configuration
* Remote connections
* Layout information

Workspaces can be persistent or temporary.

## Workspace Selector

The workspace selector allows users to:

* View workspaces
* Switch workspaces
* Create workspaces
* Rename workspaces
* Duplicate workspaces
* Delete workspaces
* Restore workspaces

The selector should update automatically when workspace resources are added or removed.

## Workspace Sidebar

The sidebar can contain a dedicated workspace panel.

Each workspace entry can display:

* Workspace name
* Active session count
* Active tab count
* Connection status
* Activity state
* Notifications

The exact amount of information should be configurable.

## Workspace Layout

A workspace can remember its terminal layout.

For example:

`Workspace`

* `Tab 1`

  * `Pane A`
  * `Pane B`
* `Tab 2`

  * `Pane A`

Restoring a workspace should recreate the configured layout whenever possible.

## Tabs

Tabs belong to a workspace.

A tab can contain one or more panes.

Tab UI can display:

* Title
* Activity indicator
* Process state
* Session state
* Close button

Tabs can be reordered through drag-and-drop or keyboard commands.

## Panes

Panes divide a tab into multiple terminal sessions or views.

Supported operations include:

* Split horizontal
* Split vertical
* Resize
* Focus
* Move
* Close
* Maximize
* Restore

Pane layouts should remain independent from the underlying terminal process whenever possible.

## Sessions

A session represents an active terminal environment.

A session can contain:

* PTY
* Shell
* Environment
* Working directory
* Profile
* Security context
* Remote connection

The workspace UI should make the relationship between sessions, tabs, and panes clear.

## Profiles

A workspace can assign profiles to sessions.

This allows users to maintain different environments.

Examples:

* Local shell
* Development environment
* Remote SSH environment
* Restricted environment
* Administrative environment

Profile changes should be validated before being applied.

## Themes

A workspace can optionally specify a theme.

If no workspace theme is selected, Conduit can inherit the global theme.

Theme inheritance should be visible to the user.

## Workspace Configuration

Workspace configuration can be stored as a resource managed by the resource system.

Workspace resources should support:

* Discovery
* Validation
* Registration
* Live updates
* Deletion detection

## Live Workspace Updates

Workspace changes should be reflected immediately when possible.

Examples:

* A new workspace resource appears
* A workspace is renamed
* A theme changes
* A profile changes
* A layout changes
* A session closes

The UI should update without requiring a full application restart.

## Workspace Persistence

Users can choose whether Conduit remembers workspace state.

Possible persistence levels include:

* None
* Layout only
* Sessions and layout
* Full workspace state

Restoration behavior should respect security policies.

## Startup

Conduit can support several startup behaviors.

Examples:

* Start with an empty workspace
* Restore the previous workspace
* Open a selected workspace
* Open a default workspace
* Open workspaces from the command line

## Workspace Switching

Switching workspaces should preserve each workspace's independent state.

The currently active workspace determines:

* Visible tabs
* Visible panes
* Workspace theme
* Workspace profile defaults
* Workspace-specific configuration

## Workspace Commands

Workspace actions should use the central command registry.

Examples:

* `workspace.create`
* `workspace.close`
* `workspace.rename`
* `workspace.switch`
* `workspace.duplicate`
* `workspace.save`
* `workspace.restore`

This keeps GUI, TUI, and CLI behavior consistent.

## Workspace Context Menu

Workspace entries can expose actions such as:

* Open
* Rename
* Duplicate
* Save
* Close
* Delete
* Open settings
* View diagnostics

Security-sensitive actions should require appropriate confirmation.

## Drag and Drop

The GUI can support drag-and-drop operations for:

* Reordering workspaces
* Moving tabs
* Moving panes
* Moving sessions
* Importing workspace resources

Drag-and-drop should have keyboard alternatives.

## Remote Workspaces

Workspaces can contain remote sessions.

Remote workspace information should clearly distinguish:

* Local sessions
* Remote sessions
* Connection state
* Remote host information

Sensitive remote information should follow security and privacy policies.

## Workspace Recovery

If a workspace cannot be restored completely, Conduit should recover as much state as safely possible.

For example:

* Restore the layout
* Restore available profiles
* Recreate local sessions
* Report unavailable remote sessions

Recovery errors should appear in diagnostics.

## Accessibility

Workspace UI should support:

* Keyboard navigation
* Screen readers
* Focus indicators
* High contrast
* Reduced motion
* Accessible labels
* Non-drag alternatives

## Design Principles

Workspace UI should be:

* Hierarchical
* Flexible
* Persistent when requested
* Recoverable
* Live
* Accessible
* Consistent across GUI, TUI, and CLI

A workspace should feel like a complete working environment rather than simply a collection of terminal tabs.

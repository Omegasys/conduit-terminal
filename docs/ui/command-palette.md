# Conduit Command Palette

## Overview

The Command Palette provides a searchable interface to Conduit's commands and actions.

It is designed for users who prefer keyboard-driven workflows and for advanced functionality that may not fit naturally into menus or toolbars.

A default shortcut may be:

```text
Ctrl+Shift+P
```

The shortcut should be configurable.

## Goals

The Command Palette should:

* Make commands discoverable.
* Provide keyboard-first access.
* Search dynamically.
* Respect current context.
* Expose the same commands as menus.
* Integrate with plugins.
* Provide access to configuration and resources.
* Avoid requiring users to memorize every shortcut.

## Architecture

The Command Palette should query the command registry.

```text
User input
    ↓
Command Palette
    ↓
Command registry
    ↓
Search / ranking
    ↓
Available commands
    ↓
Command dispatcher
```

The palette should not contain independent implementations of commands.

## Search

Search should support:

* Fuzzy matching.
* Prefix matching.
* Exact matching.
* Command identifiers.
* Menu names.
* Keyboard shortcuts.
* Resource names.
* Contextual keywords.

For example, searching:

```text
split
```

may find:

* Split Pane Horizontally.
* Split Pane Vertically.
* Split Current Pane.
* Equalize Panes.

## Context Awareness

Commands should be filtered according to the current context.

Context may include:

* Active window.
* Active tab.
* Active pane.
* Focused UI element.
* Session state.
* Connection type.
* Security profile.
* Available plugins.
* Current selection.

Unavailable commands should normally be hidden or clearly marked as unavailable.

## Command Information

Each command may expose:

* Name.
* Description.
* Command identifier.
* Category.
* Keyboard shortcut.
* Menu location.
* Required permissions.
* Current availability.

## Categories

The palette may organize commands into categories such as:

* File
* Edit
* View
* Terminal
* Tabs
* Panes
* Sessions
* Workspaces
* Profiles
* Themes
* Security
* Plugins
* Tools
* Diagnostics

## Resource Search

The Command Palette can also provide resource actions.

Examples include:

* Open workspace.
* Switch profile.
* Apply theme.
* Open connection.
* Enable plugin.

Resource results should come from the resource registry.

## Recent Commands

Conduit may remember recently used commands.

Recent-command storage should be configurable.

Users should be able to disable command usage history.

## Command History

The Command Palette should remain distinct from shell command history.

Shell history belongs to the shell integration layer.

Conduit command history records interactions with the Conduit interface itself.

## Keyboard Navigation

The palette should support:

* Arrow keys.
* Page navigation.
* Enter.
* Escape.
* Tab where appropriate.
* Configurable shortcuts.

## Actions With Parameters

Some commands require parameters.

The palette may provide a second-stage input interface.

For example:

```text
Switch Workspace
    ↓
Workspace search
    ↓
Select workspace
```

This should remain within the command framework.

## Live Command Registration

Plugins may contribute commands.

When a plugin is enabled, its permitted commands should appear automatically.

When a plugin is disabled or removed, its commands should disappear.

## Security

The Command Palette should respect security policy.

A command requiring a denied capability should not become usable merely because it is visible in the palette.

Commands may display permission information where useful.

## Configuration Access

The palette should provide shortcuts to:

* Settings.
* Configuration Editor.
* Profiles.
* Workspaces.
* Themes.
* Keybindings.
* Security configuration.

## Diagnostics

Advanced commands may provide direct access to:

* Diagnostics.
* Event inspection.
* Flow View.
* Renderer information.
* Protocol information.
* Resource state.
* Live reload status.

## Live Updates

The Command Palette should update automatically when:

* Commands are registered.
* Plugins change.
* Resources change.
* Keybindings change.
* Security policies change.
* Workspace state changes.

## Accessibility

The palette should support:

* Screen readers.
* Keyboard-only operation.
* Clear focus.
* Adjustable text size.
* High-contrast themes.

## CLI and TUI Integration

The Command Palette should represent commands that are also available through other interfaces where appropriate.

The command architecture should therefore remain:

```text
                 Command Registry
                 /       |       \
              GUI      TUI       CLI
               ↓         ↓         ↓
                    Command Bus
                         ↓
                    Core Systems
```

## Design Principle

The Command Palette is the universal command discovery interface for Conduit.

If a feature is exposed as a command, users should be able to discover and invoke it through the palette when the current context and security policy permit it.

# Conduit Configuration Editor

## Overview

The Conduit Configuration Editor provides a graphical and terminal-based interface for viewing and modifying Conduit's configuration.

The editor operates on the same configuration model used by the configuration engine.

It should not implement a separate configuration system.

## Goals

The configuration editor should provide:

* Human-readable configuration
* Structured editing
* Search
* Validation
* Change previews
* Undo and redo
* Rollback
* Live application
* Configuration file integration
* Diagnostics
* GUI/TUI/CLI parity

## Configuration Model

The editor communicates with the configuration engine rather than directly modifying application components.

The basic flow is:

`Editor → Configuration Engine → Validation → Transaction → Application`

This keeps configuration changes consistent across all interfaces.

## Editing Modes

Conduit can provide several editing modes.

### Form Editor

The form editor presents settings as controls such as:

* Checkboxes
* Toggles
* Sliders
* Dropdowns
* Text fields
* Path selectors
* Color selectors
* Font selectors
* Lists
* Tables

This should be the primary mode for ordinary users.

### Structured Editor

The structured editor exposes configuration sections and keys directly.

This is useful for advanced users who want more control without manually editing the configuration file.

### Raw File Editor

Advanced users may open the actual configuration file.

For TOML configuration, Conduit should provide:

* Syntax highlighting
* Validation
* Error locations
* Completion
* Formatting
* Search
* Undo/redo

The raw editor must still pass changes through the configuration engine before they are applied.

## Configuration Preview

Before applying changes, the editor can display a change summary.

Examples:

* Added setting
* Removed setting
* Changed setting
* Affected component
* Required restart
* Security impact

## Validation

Validation occurs before configuration changes are committed.

The editor should detect:

* Invalid syntax
* Unknown keys
* Invalid values
* Missing required values
* Conflicting settings
* Invalid paths
* Unsupported features
* Security violations

## Apply

When the user applies changes, Conduit should:

1. Parse the edited configuration
2. Validate the configuration
3. Compare it with the active configuration
4. Determine affected components
5. Create a configuration transaction
6. Apply safe changes
7. Restart only affected components when required
8. Confirm the resulting state
9. Commit the configuration

## Rollback

If applying a configuration fails, Conduit should restore the previous known-good configuration.

Rollback should be automatic when necessary.

The editor should report:

* What failed
* What was rolled back
* Which component was affected

## Undo and Redo

The editor should maintain an undo/redo history.

Undo should operate on configuration changes rather than arbitrary UI actions whenever possible.

Examples:

* Change font
* Enable sidebar
* Modify keybinding
* Change security profile
* Add workspace
* Change renderer

## Search

Configuration search should support:

* Setting names
* Configuration keys
* Descriptions
* Categories
* Current values
* Plugin settings

Search results should navigate directly to the relevant configuration entry.

## Configuration Schema

The editor uses the Conduit configuration schema to understand valid settings.

The schema should describe:

* Key names
* Types
* Defaults
* Allowed values
* Ranges
* Descriptions
* Dependencies
* Deprecation information
* Security requirements

## Completion

Advanced configuration editing can provide completion suggestions.

Examples:

* Configuration keys
* Enum values
* Known profiles
* Known themes
* Workspace names
* Plugin names
* Paths

## External Changes

Conduit should monitor the configuration file for changes made outside the editor.

When an external change is detected, Conduit should:

1. Detect the file change
2. Parse the new configuration
3. Validate it
4. Compare it with the current configuration
5. Apply safe changes
6. Preserve the previous configuration if validation fails

If the editor currently contains unsaved changes, Conduit should detect the conflict rather than silently overwriting either version.

## Configuration Conflicts

Conflicts can occur when:

* The editor has unsaved changes
* Another process modifies the configuration
* A configuration resource changes
* A plugin changes its configuration schema

The editor should provide a clear conflict-resolution interface.

Possible actions include:

* Keep editor version
* Keep external version
* Merge changes
* Cancel

## Profiles

The configuration editor should support profile-specific settings.

Users should be able to:

* Create profiles
* Edit profiles
* Duplicate profiles
* Rename profiles
* Delete profiles
* Compare profiles
* Activate profiles

## Workspaces

Workspace configuration can be edited through the same system.

Workspace configuration can include:

* Layout
* Sessions
* Profiles
* Themes
* Startup behavior
* Pane configuration
* Environment settings

## Themes and Resources

The editor should recognize dynamically discovered resources.

For example, when a new valid theme appears in the theme directory, the editor can display it without requiring a restart.

Invalid resources should produce diagnostics instead of replacing valid resources.

## Security

Configuration changes that affect security should receive additional validation.

Examples include:

* Disabling sandboxing
* Expanding plugin permissions
* Allowing additional URI schemes
* Changing clipboard restrictions
* Changing remote-session policies

Security-sensitive changes should be clearly identified.

## Live Application

Changes should be applied live whenever safe.

The configuration engine determines whether a change requires:

* No action
* UI refresh
* Component reload
* Component restart
* Application restart

The editor should display this information when known.

## CLI and TUI Integration

The same configuration engine should support CLI and TUI configuration.

Examples:

`conduit config get terminal.scrollback`

`conduit config set terminal.scrollback 10000`

`conduit config validate`

`conduit config diff`

`conduit config reload`

These commands should produce results consistent with the GUI editor.

## Diagnostics Integration

Configuration diagnostics should integrate with the Conduit diagnostics system.

Errors can be surfaced through:

* Configuration editor
* Status bar
* Notifications
* Diagnostics panel
* Command palette

## Design Principles

The configuration editor should be:

* Structured
* Discoverable
* Reversible
* Validated
* Live-aware
* Security-aware
* Accessible
* Consistent across GUI, TUI, and CLI

The editor should make advanced configuration easier without hiding the underlying configuration model from experienced users.

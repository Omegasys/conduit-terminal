# Conduit Toolbar

## Overview

The Conduit toolbar provides quick access to frequently used commands.

It is optional and should remain less important than the terminal itself.

## Default Actions

A default toolbar may contain:

* New Terminal
* New Tab
* Split Pane
* Close
* Search
* Command Palette
* Workspace
* Profile
* Security
* Settings

The exact default layout may evolve with the application.

## Toolbar Customization

Users should be able to:

* Enable or disable the toolbar.
* Add commands.
* Remove commands.
* Reorder commands.
* Insert separators.
* Choose icon-only mode.
* Choose icon-and-label mode.
* Reset to defaults.

## Command-Based Architecture

Toolbar buttons should invoke commands rather than implement behavior directly.

```text
Toolbar button
    ↓
Command ID
    ↓
Command dispatcher
    ↓
Command handler
```

This ensures consistent behavior across the application.

## Dynamic State

Toolbar actions should reflect current state.

For example:

* Recording button changes state while recording.
* Security indicator reflects the active security profile.
* Workspace selector reflects the active workspace.
* Split commands reflect the current pane state.

## Workspace Integration

Workspaces may provide toolbar preferences.

A workspace may define:

* Visible toolbar commands.
* Toolbar mode.
* Custom shortcuts.
* Contextual tools.

Global security policy should still override unsafe actions.

## Live Updates

Toolbar configuration should support live changes.

Changing toolbar settings should normally not require restarting Conduit.

## Themes

Toolbar appearance should use the active Conduit theme.

Theme changes should update the toolbar without requiring a restart where possible.

## Accessibility

Toolbar controls should expose:

* Accessible names.
* Keyboard focus.
* Tooltips.
* State information.
* Keyboard alternatives.

Icon-only buttons should always have accessible labels.

## Compact Mode

Conduit may provide a compact toolbar mode for users who want more terminal space.

The toolbar may also automatically collapse into a smaller control set when window dimensions become constrained.

## Security Indicator

The toolbar may include an optional security indicator showing:

* Active security profile.
* Restricted capabilities.
* Safe Mode.
* Pending security warnings.

The indicator should not expose sensitive information.

## Design Principle

The toolbar is a convenience layer.

Every toolbar action should remain available through the command system even when the toolbar itself is disabled.

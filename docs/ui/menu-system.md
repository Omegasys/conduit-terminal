# Conduit Menu System

## Overview

The Conduit menu system provides traditional desktop application menus while exposing the same command architecture used by the Command Palette, keyboard shortcuts, TUI, and CLI.

Menus are therefore views of the command system rather than independent implementations of functionality.

## Default Menus

Conduit provides the following top-level menus:

```text
File
Edit
View
Terminal
Tabs
Panes
Session
Tools
Help
```

Users may customize menu contents where supported.

## File

The File menu may contain:

* New Window
* New Terminal
* Open Configuration
* Open Configuration Directory
* Open Workspace
* Save Workspace
* Import
* Export
* Close Window
* Exit

## Edit

The Edit menu may contain:

* Undo
* Redo
* Cut
* Copy
* Paste
* Paste Safely
* Select All
* Search
* Command History

## View

The View menu may contain:

* Toggle Sidebar
* Toggle Toolbar
* Toggle Status Bar
* Toggle Fullscreen
* Zoom In
* Zoom Out
* Reset Zoom
* Flow View
* Diagnostics
* Command Palette

## Terminal

The Terminal menu may contain:

* New Terminal
* Restart Shell
* Clear Screen
* Clear Scrollback
* Reset Terminal
* Send Signal
* Change Shell
* Toggle Alternate Screen
* Record Session
* Stop Recording

## Tabs

The Tabs menu may contain:

* New Tab
* Close Tab
* Reopen Tab
* Next Tab
* Previous Tab
* Move Tab
* Rename Tab
* Duplicate Tab
* Detach Tab

## Panes

The Panes menu may contain:

* Split Horizontally
* Split Vertically
* Close Pane
* Focus Next Pane
* Focus Previous Pane
* Resize Pane
* Collapse Pane
* Expand Pane
* Equalize Panes

## Session

The Session menu may contain:

* Session Information
* Rename Session
* Save Session
* Restore Session
* Reconnect
* Disconnect
* Change Profile
* Change Security Profile
* Session Diagnostics

## Tools

The Tools menu may contain:

* Command Palette
* Settings
* Configuration Editor
* Workspace Manager
* Profile Manager
* Theme Manager
* Plugin Manager
* Connection Manager
* History
* Diagnostics
* Flow View

## Help

The Help menu may contain:

* Documentation
* Keyboard Shortcuts
* Configuration Reference
* Protocol Support
* Plugin Documentation
* Diagnostics
* About Conduit

## Command Integration

Every menu action should map to a command identifier.

```text
Menu item
    ↓
Command ID
    ↓
Command dispatcher
    ↓
Command handler
```

This means a menu action and a keyboard shortcut can execute exactly the same operation.

## Command State

Menu items should dynamically reflect application state.

For example:

* Close Tab may be disabled when no tab exists.
* Reconnect may be available only for reconnectable sessions.
* Paste may be disabled when no clipboard data is available.
* Stop Recording may be available only while recording.
* Undo may be unavailable when there is no undoable action.

## Dynamic Menus

Some menus may contain dynamically generated content.

Examples include:

* Recent workspaces.
* Recent sessions.
* Available shells.
* Installed themes.
* Profiles.
* Connections.
* Plugins.
* Recently used commands.

Dynamic entries should be generated from the same registries used elsewhere in Conduit.

## Live Menu Updates

Menus should update automatically when resources change.

For example, adding:

```text
~/.config/conduit/themes/new-theme.toml
```

should eventually make the theme available in the appropriate UI without requiring a full application restart.

## Contextual Menus

Context menus should use the same command system.

The available commands can depend on:

* Current UI element.
* Current session.
* Current terminal mode.
* Selection.
* Security profile.
* Connection type.
* Plugin availability.

## Custom Menus

Advanced users may configure menu layouts.

Customization should support:

* Adding commands.
* Removing commands.
* Reordering items.
* Separators.
* Submenus.
* Conditional visibility.

## Keybindings

Menu commands should expose their associated keybindings where appropriate.

This makes menus useful as a discoverability mechanism.

## Accessibility

Menus should support:

* Keyboard navigation.
* Mnemonics where supported.
* Screen readers.
* Clear focus indicators.
* Consistent ordering.

## Design Principle

Menus are a presentation layer for Conduit's command system.

No important functionality should exist only inside the GUI menu implementation.

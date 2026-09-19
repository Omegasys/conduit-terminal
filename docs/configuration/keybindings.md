# Conduit Keybindings

## Overview

Conduit uses a configurable keybinding system shared across the GUI, TUI, command palette, terminal interface, and other supported components.

Keybindings map user input to Conduit commands.

The system is designed so that commands and keybindings remain separate.

```text
Keyboard Input
      ↓
Input System
      ↓
Keybinding Resolver
      ↓
Command
      ↓
Command Dispatcher
      ↓
Conduit Component
```

## Keybinding Architecture

Keybinding functionality is implemented through:

```text
src/input/
├── keyboard.rs
├── shortcuts.rs
└── keybindings.rs

src/command/
├── palette.rs
├── execution.rs
└── ...
```

## Command-Based Design

A keybinding should invoke a command rather than directly modifying application state.

For example:

```text
Ctrl+Shift+P
      ↓
Open Command Palette
```

This allows the same command to be triggered by:

* Keyboard
* Menu
* Toolbar
* Command palette
* TUI
* CLI
* Plugin

## Configuration

Keybindings can be configured through TOML.

Example:

```toml
[keybindings]
"ctrl+shift+p" = "command_palette.open"
"ctrl+shift+t" = "tab.new"
"ctrl+shift+w" = "tab.close"
"ctrl+shift+n" = "window.new"
```

The exact command names are defined by the command registry.

## Key Representation

Conduit should normalize keyboard input before resolving a binding.

A binding may contain:

* Modifier keys
* Main key
* Function keys
* Navigation keys
* Keypad keys
* Platform-specific keys

Supported modifiers can include:

* Ctrl
* Alt
* Shift
* Super
* Meta

## Key Chords

Conduit may support multi-step key sequences.

Example:

```text
Ctrl+K → Ctrl+S
```

Key chords should have timeout and cancellation behavior.

## Contexts

Keybindings can operate within contexts.

Examples include:

* Global
* Terminal
* Text selection
* Command palette
* Settings
* Configuration editor
* Sidebar
* Flow View
* Search
* TUI

A context-specific binding can override a broader binding when appropriate.

## Priority

Binding resolution should consider context and priority.

A simplified process is:

```text
Specific Context
      ↓
Component Context
      ↓
Workspace
      ↓
Global
```

The exact precedence is controlled by the input system.

## Terminal Input

Terminal applications need access to keyboard input.

Conduit must distinguish between:

```text
Input intended for Conduit
```

and:

```text
Input intended for the terminal process
```

Not every key should be intercepted by Conduit.

Users should be able to configure this behavior.

## Conflicts

The keybinding system should detect conflicts.

Examples:

* Two commands use the same global shortcut.
* A workspace binding conflicts with a global binding.
* A terminal binding conflicts with a UI command.

The configuration editor and Settings application should report these conflicts.

## Built-In Shortcuts

Conduit can provide default shortcuts for common operations such as:

* New tab
* Close tab
* New window
* Close window
* Split pane
* Focus pane
* Resize pane
* Switch workspace
* Open settings
* Open command palette
* Search terminal
* Copy
* Paste

Defaults should remain configurable.

## Command Palette Integration

The command palette should display available keybindings next to commands.

This makes shortcuts discoverable without requiring users to memorize them.

## Menu Integration

Menus should display associated shortcuts.

For example:

```text
Edit
├── Copy                 Ctrl+Shift+C
├── Paste                Ctrl+Shift+V
└── Select All
```

The actual shortcuts should come from the keybinding registry rather than being hard-coded into menu definitions.

## Workspace Keybindings

A workspace may define specialized bindings.

For example, a development workspace might assign shortcuts to frequently used development commands.

Workspace bindings should be removable without modifying global configuration.

## Profile Keybindings

Profiles can provide predefined keybinding sets.

This allows environments such as:

* Default
* Vim-like
* Emacs-like
* Minimal
* Development

to use different shortcut arrangements.

## Import and Export

Users should be able to export keybindings independently from the rest of the configuration.

This allows shortcuts to be moved between systems.

## Live Reload

Keybinding changes should normally be applied immediately.

The process is:

```text
Keybinding File Change
       ↓
Parse
       ↓
Validate
       ↓
Conflict Detection
       ↓
Update Registry
       ↓
Input System Refresh
```

No terminal session restart should normally be required.

## Invalid Keybindings

An invalid binding should not replace a valid existing binding.

Conduit should report:

* Invalid key syntax
* Unknown command
* Conflict
* Invalid context
* Unsupported key

## Accessibility

Keybindings should integrate with accessibility settings.

Users should be able to configure alternatives for important actions.

The system should also avoid making essential functionality accessible only through complex shortcuts.

## Plugins

Plugins can register commands and optionally provide default keybindings.

Plugin keybindings must pass through the same conflict detection and permission systems as built-in commands.

## CLI

Keybinding management can be exposed through the CLI.

Examples:

```text
conduit keybind list
conduit keybind find <command>
conduit keybind set <key> <command>
conduit keybind remove <key>
conduit keybind conflicts
```

## Security

Keybindings should not automatically bypass security controls.

A shortcut that invokes a privileged or potentially destructive command should still be subject to the command system and relevant confirmation or permission requirements.

## Design Goal

The keybinding system should make Conduit highly customizable without fragmenting command behavior.

The principle is:

> Keys trigger commands; commands define behavior.

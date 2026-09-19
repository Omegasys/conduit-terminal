# Conduit Resources

## Overview

Conduit uses a resource system for discovering, loading, validating, and managing data-driven components.

Resources allow users to extend and customize Conduit without modifying the application source code.

Examples include:

* Themes
* Profiles
* Workspaces
* Workspace templates
* Configuration resources
* Other supported data files

Resources are intentionally different from plugins.

A resource contains data.

A plugin can contain executable code and therefore requires additional security controls.

## Resource Architecture

The resource system is implemented under:

```text
src/resources/
├── mod.rs
├── manager.rs
├── discovery.rs
├── registry.rs
├── paths.rs
├── resource.rs
├── theme.rs
├── profile.rs
├── workspace.rs
├── manifest.rs
└── validation.rs
```

The resource system integrates with:

```text
src/config_engine/
src/live/
src/themes/
src/workspaces/
src/events/
```

## Resource Lifecycle

A resource follows a controlled lifecycle:

```text
Discover
   ↓
Identify
   ↓
Parse
   ↓
Validate
   ↓
Register
   ↓
Load
   ↓
Activate
   ↓
Watch for Changes
   ↓
Reload / Remove
```

A resource should not become active until validation succeeds.

## Resource Types

### Themes

Themes define visual appearance.

Examples include:

* Colors
* Fonts
* UI appearance
* Terminal appearance
* Flow View appearance

### Profiles

Profiles contain reusable configuration settings.

Examples include:

* Development
* SSH
* Minimal
* Security
* Hardened

### Workspaces

Workspaces describe complete working environments.

They may contain:

* Layout
* Tabs
* Panes
* Sessions
* Profiles
* Themes
* Working directories

### Templates

Templates provide starting configurations for new resources.

Examples include:

* Development workspace
* SSH workspace
* Minimal profile
* Custom theme

## Resource Registry

The registry maintains the currently known resources.

A registry entry can contain:

* Resource identifier
* Resource type
* Display name
* Version
* Source path
* Validation state
* Loaded state
* Active state
* Dependencies
* Metadata

The registry is shared with the GUI, TUI, CLI, command palette, and diagnostics system.

## Resource Manager

The resource manager coordinates:

* Discovery
* Loading
* Validation
* Registration
* Activation
* Reloading
* Removal

Individual managers such as the Theme Manager can build specialized behavior on top of the general resource system.

## Resource Paths

Typical user resource paths include:

```text
~/.config/conduit/
├── themes/
├── profiles/
└── workspaces/
```

System resources may be installed under:

```text
/usr/share/conduit/
├── themes/
├── profiles/
└── workspaces/
```

Additional user data resources may use:

```text
~/.local/share/conduit/
```

Exact paths should remain configurable where practical.

## Precedence

When multiple resources use the same identifier, Conduit resolves them according to resource precedence.

A typical model is:

```text
System Resources
      ↓
User Resources
      ↓
Workspace / Runtime Overrides
```

The exact precedence is controlled by the configuration engine and resource policy.

## Resource Identity

Each resource should have a stable identifier.

The identifier should be suitable for:

* Configuration references
* CLI commands
* Workspace files
* Plugin APIs
* Internal registry lookups

A display name can be separate from the identifier.

## Manifests

Some resource types may optionally provide metadata through a manifest.

Example:

```toml
[resource]
id = "example-theme"
type = "theme"
version = "1.0"
name = "Example Theme"
description = "Example Conduit theme"
```

Simple resources should not require unnecessary metadata.

## Validation

Resources are validated before activation.

Validation can check:

* Syntax
* Schema
* Required fields
* Data types
* Allowed values
* References
* Dependencies
* Resource-specific constraints

## Invalid Resources

An invalid resource should not replace the last valid version.

For example:

```text
Valid Theme
     ↓
File Modified
     ↓
Invalid TOML
     ↓
Reject New Version
     ↓
Keep Previous Theme
     ↓
Report Diagnostic
```

This is particularly important for live-reloaded resources.

## Live Resource Changes

Resource files can be changed while Conduit is running.

The resource watcher detects:

* Creation
* Modification
* Rename
* Deletion

The resource manager then determines what action is appropriate.

## Resource Events

Important resource events include:

* ResourceDiscovered
* ResourceValidated
* ResourceRegistered
* ResourceActivated
* ResourceChanged
* ResourceReloaded
* ResourceRemoved
* ResourceInvalid
* ResourceError

These events are published through the event bus.

## Resource Security

Resources are data and should not automatically execute arbitrary code.

For example, a theme should not be capable of executing a shell command merely because it contains a particular configuration value.

Resource parsers should treat resource files as untrusted data.

## Resource and Plugin Separation

The distinction is intentional:

```text
Resource
   ↓
Data
   ↓
Parse / Validate
   ↓
Activate

Plugin
   ↓
Executable Code
   ↓
Permissions
   ↓
Sandbox / Isolation
   ↓
Load
```

This keeps ordinary customization simpler and safer than executable extensions.

## GUI Integration

The GUI should provide resource managers for:

* Themes
* Profiles
* Workspaces
* Templates

Users should be able to see:

* Available resources
* Active resources
* Invalid resources
* Resource paths
* Versions
* Metadata

## TUI Integration

The TUI should provide keyboard-driven resource management.

## CLI Integration

Examples:

```text
conduit resource list
conduit resource list themes
conduit resource list profiles
conduit resource list workspaces
conduit resource validate <file>
conduit resource reload <id>
```

Specialized commands such as `conduit theme list` can provide a friendlier interface.

## Resource Import and Export

Supported resources can be imported and exported independently.

This allows users to share:

* Themes
* Profiles
* Workspaces
* Templates

Import must validate resources before activation.

## Resource Diagnostics

The diagnostics system should expose:

* Invalid resources
* Failed loads
* Missing dependencies
* Duplicate identifiers
* Permission problems
* Reload failures

## Design Goal

The resource system makes Conduit extensible through data rather than executable code.

The principle is:

> Customize Conduit by adding resources, not by modifying the application.

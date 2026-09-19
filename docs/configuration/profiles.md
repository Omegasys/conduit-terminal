# Conduit Profiles

## Overview

Profiles are reusable collections of Conduit settings.

A profile allows users to create a consistent environment without duplicating an entire configuration file.

Examples include:

* Default
* Minimal
* Development
* Security
* Hardened
* SSH
* Custom

## Profile Architecture

Profile functionality is integrated with:

```text
src/config_engine/profiles.rs
src/resources/profile.rs
src/resources/manager.rs
```

Profiles are treated as configuration resources and can be automatically discovered.

## Profile Locations

User profiles can be stored under:

```text
~/.config/conduit/profiles/
```

System profiles may be installed under:

```text
/usr/share/conduit/profiles/
```

User profiles should take precedence over system profiles when their identifiers conflict.

## Example Profile

```toml
[profile]
name = "Development"
description = "Development-focused terminal environment"

[terminal]
scrollback_lines = 50000

[appearance]
theme = "cyberpunk"
font_size = 13.0

[behavior]
confirm_close = true
```

## Profile Discovery

Conduit automatically searches supported profile directories.

A new valid profile can therefore be added without modifying the Conduit executable.

For example:

```text
~/.config/conduit/profiles/
├── development.toml
├── server.toml
└── personal.toml
```

## Live Profile Discovery

The resource watcher monitors profile directories.

When a profile is:

* Added
* Modified
* Renamed
* Removed

Conduit updates its profile registry.

Invalid profiles should be rejected without disrupting active configuration.

## Profile Selection

Profiles can be selected through:

* GUI Settings
* Profile manager
* TUI
* CLI
* Command palette
* Workspace configuration
* Session configuration

## Profile Precedence

A profile supplies configuration values but does not necessarily override every other configuration source.

The configuration engine resolves the final state using its defined precedence rules.

## Profile Composition

Conduit may support profile inheritance or composition.

For example:

```text
Default
   ↓
Development
   ↓
Rust Development
```

A child profile can override selected values.

Composition should detect circular dependencies.

## Security Profiles

Security-oriented profiles may configure:

* Plugin permissions
* Clipboard restrictions
* Hyperlink handling
* Graphics policies
* Recording behavior
* External integrations
* Sandbox settings

Profiles should expose the individual settings they modify rather than presenting a profile name as a security guarantee.

## Temporary Profiles

A profile can be selected for a single invocation.

Example:

```text
conduit --profile minimal
```

This should not permanently modify the user's default configuration.

## Session Profiles

A session can use a profile independently from the global default.

For example:

```text
Local Shell → Default
SSH Server  → SSH
Development → Development
```

## Workspace Profiles

Workspaces can select profiles.

This allows each workspace to establish a consistent environment.

## Profile Manager

The GUI profile manager should provide:

* Profile list
* Search
* Create
* Edit
* Duplicate
* Enable/select
* Rename
* Delete
* Import
* Export
* Validation status

## CLI

Example commands:

```text
conduit profile list
conduit profile show <name>
conduit profile create <name>
conduit profile edit <name>
conduit profile delete <name>
conduit profile validate <name>
```

## Profile Validation

Profiles must be validated before activation.

Validation should check:

* TOML syntax
* Schema
* Value types
* Allowed values
* Dependencies
* Security constraints

## Live Profile Changes

If an active profile is modified, Conduit should calculate the resulting configuration difference.

Changes should be:

* Applied immediately when safe.
* Deferred when necessary.
* Applied after component reload when required.
* Rejected when invalid.

## Profile Rollback

If applying a modified profile fails, Conduit should retain the previous valid state.

The user should receive diagnostics explaining which setting caused the failure.

## Profile Templates

Conduit can provide built-in templates for common environments.

Templates may include:

* Default
* Minimal
* Development
* SSH
* Security
* Hardened

Users can create custom profiles from these templates.

## Design Goal

Profiles provide reusable configuration without forcing users to maintain multiple complete configuration files.

The principle is:

> A profile describes how Conduit should behave in a particular environment.

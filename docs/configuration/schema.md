# Conduit Configuration Schema

## Overview

The Conduit configuration schema defines the structure, types, constraints, and relationships of configuration data.

The schema is used by:

* Configuration loading
* Validation
* Settings
* Configuration editor
* Autocomplete
* Profiles
* Workspaces
* Plugins
* Diagnostics
* CLI
* TUI
* Live reload

The schema is a shared source of truth.

## Schema Location

The repository contains the primary schema definition under:

```text
config/schema.toml
```

The Rust configuration engine consumes the schema through:

```text
src/config_engine/schema.rs
```

## Schema Responsibilities

The schema defines:

* Configuration sections
* Configuration keys
* Types
* Defaults
* Allowed values
* Ranges
* Required fields
* Deprecated values
* Security classifications
* Reload behavior
* Documentation

## Example

A simplified schema entry might conceptually describe:

```toml
[terminal.scrollback_lines]
type = "integer"
default = 10000
minimum = 0
maximum = 10000000
reload = "live"
description = "Maximum number of scrollback lines."
```

The exact schema format may evolve independently from the user-facing configuration format.

## Configuration Types

Supported configuration types can include:

* String
* Integer
* Floating point
* Boolean
* Array
* Table
* Enumerated value
* Path
* Duration
* Color
* Keybinding
* Resource reference

## Defaults

The schema can define defaults where appropriate.

Defaults provide predictable behavior when a user does not specify a value.

Built-in defaults are maintained separately from user configuration.

## Required Values

Some configuration objects may require specific fields.

Validation should identify missing required values before activation.

## Enumerations

Settings with limited choices should define valid values.

For example:

```text
cursor.style
├── block
├── beam
└── underline
```

The configuration editor can use these values to provide autocomplete and selection controls.

## Numeric Constraints

Numeric settings can define:

* Minimum
* Maximum
* Inclusive or exclusive bounds
* Recommended values

For example:

```text
font.size
minimum = 1
maximum = 200
```

## String Constraints

String settings may define:

* Maximum length
* Pattern
* Allowed schemes
* Path requirements
* Identifier rules

## Resource References

The schema can define references to resources.

Examples include:

```text
theme = "nord"
profile = "development"
workspace = "server"
```

The configuration engine can validate that referenced resources exist or determine whether unresolved references are permitted.

## Keybinding Schema

Keybinding definitions can specify:

* Key syntax
* Command
* Context
* Chord behavior

The keybinding subsystem performs additional conflict validation.

## Security Classification

Schema entries can identify security-sensitive settings.

For example:

```text
Public
Sensitive
SecuritySensitive
Privileged
```

This allows Settings and the configuration editor to provide appropriate warnings and access controls.

## Reload Classification

Each setting can identify how it can be changed.

Possible values include:

```text
live
component_refresh
component_restart
window_recreate
application_restart
startup_only
```

This information feeds the live reload dependency system.

## Dependency Information

Some settings affect other components.

For example:

```text
appearance.font
    ↓
font manager
    ↓
text layout
    ↓
renderer
```

Schema metadata can describe these relationships.

## Validation Pipeline

Schema validation occurs after parsing.

```text
TOML
 ↓
Parser
 ↓
Schema Validation
 ↓
Resource / Configuration Validation
 ↓
Security Validation
 ↓
Candidate State
```

## Unknown Keys

Unknown configuration keys should normally produce a diagnostic.

Conduit should avoid silently accepting misspelled configuration names.

Compatibility modes may allow selected unknown values where necessary.

## Deprecated Settings

The schema can identify deprecated settings.

Diagnostics should explain:

* Why the setting is deprecated.
* What replaces it.
* Whether automatic migration is available.

## Schema Versions

The schema should have a version.

This allows Conduit to distinguish between:

* Current configuration
* Older configuration
* Future/incompatible configuration

## Migration

When the schema changes, the migration system can transform older configuration.

```text
Old Configuration
      ↓
Detect Version
      ↓
Migration
      ↓
Current Schema
      ↓
Validation
```

## Profiles and Schema

Profiles must conform to the appropriate portions of the configuration schema.

A profile should not be able to introduce arbitrary configuration keys.

## Workspaces and Schema

Workspace resources can use workspace-specific sections defined by the schema.

References to profiles, themes, and sessions should also be validated.

## Themes and Schema

Themes have their own resource schema.

The theme schema defines:

* Colors
* Fonts
* Appearance
* Metadata
* Theme-specific properties

## Plugin Configuration

Plugins may define their own configuration schema within the boundaries of the plugin API.

Plugin configuration should be namespaced to prevent collisions.

For example:

```text
plugin.git.*
plugin.ssh.*
plugin.docker.*
```

## Configuration Editor

The schema provides the configuration editor with:

* Autocomplete
* Type information
* Documentation
* Validation
* Allowed values
* Reload behavior

## CLI Integration

The CLI can expose schema information.

Examples:

```text
conduit config schema
conduit config describe terminal.scrollback_lines
conduit config validate
```

## TUI Integration

The TUI can use schema metadata to build interactive settings and validation interfaces.

## Generated Documentation

Schema metadata can potentially generate portions of:

* Configuration documentation
* Settings descriptions
* CLI help
* Configuration editor documentation

This reduces duplication between implementation and documentation.

## Testing

The schema should be tested for:

* Invalid types
* Missing required fields
* Invalid ranges
* Unknown keys
* Deprecated settings
* Resource references
* Security classifications
* Reload classifications

## Compatibility

Schema changes should be considered carefully because configuration files can remain in use for many years.

Backward compatibility should be preferred where practical.

## Security

Schema validation is not a complete security boundary.

A value being valid according to the schema does not automatically mean it is safe.

Security-sensitive values must also pass the security policy.

## Design Goal

The schema provides a common language for every part of Conduit's configuration system.

The principle is:

> One schema should teach Conduit what is valid, how it behaves, and how it can change.

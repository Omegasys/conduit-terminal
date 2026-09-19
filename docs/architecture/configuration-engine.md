# Conduit Configuration Engine

The Conduit configuration engine is responsible for loading, validating, applying, migrating, and persisting configuration.

It provides a single configuration system shared by the GUI, TUI, CLI, resource system, and live reload system.

## Configuration File

The primary user configuration file is:

```text
~/.config/conduit/config.toml
```

The configuration file uses TOML.

Users can edit it directly or modify settings through Conduit's graphical Settings application.

## Shared Configuration Model

The GUI, TUI, CLI, and direct file editing all ultimately interact with the same configuration model.

The general flow is:

Configuration Source
→ Loader
→ Parser
→ Schema
→ Validator
→ Configuration State
→ Diff
→ Transaction
→ Event Bus
→ Components

This prevents each interface from maintaining a separate configuration implementation.

## Configuration Sources

Configuration can originate from:

* Built-in defaults
* System configuration
* User configuration
* Profiles
* Workspaces
* Command-line overrides
* Environment-specific settings

The configuration engine combines these sources according to their defined precedence.

## Configuration Precedence

A typical precedence model is:

Built-in Defaults
→ System Configuration
→ User Configuration
→ Profile
→ Workspace
→ Session Overrides
→ Command-Line Overrides

More specific configuration can override less specific configuration.

The exact precedence rules should remain deterministic and documented.

## Parsing

The parser converts TOML into Conduit's internal configuration representation.

Parsing errors should identify:

* File
* Section
* Key
* Location when available
* Error type
* Human-readable explanation

Invalid configuration should not silently replace a valid active configuration.

## Schema

The configuration schema defines:

* Valid sections
* Valid keys
* Value types
* Allowed values
* Defaults
* Deprecation information
* Documentation

The schema is also used by the built-in configuration editor.

## Validation

Validation occurs after parsing.

Validation can check:

* Data types
* Required values
* Allowed values
* Numeric ranges
* Path validity
* Feature dependencies
* Conflicting settings

For example, a renderer backend may require a corresponding platform or graphics capability.

## Configuration State

The active configuration is represented internally as configuration state.

Components should consume this state rather than repeatedly reading `config.toml` themselves.

This allows configuration to be changed without requiring every component to implement its own file watcher.

## Configuration Diffs

When a new configuration is loaded, Conduit compares it against the active configuration.

The result identifies:

* Added values
* Removed values
* Changed values
* Unaffected values

For example:

```text
font.size
theme
renderer.backend
scrollback_lines
```

may change independently.

## Dependency Analysis

Configuration changes are evaluated against component dependencies.

For example:

```text
theme
→ appearance
→ renderer
```

while:

```text
scrollback_lines
→ scrollback
→ terminal session
```

This allows Conduit to determine what actually needs to change.

## Live Application

The preferred configuration path is:

Configuration Change
→ Parse
→ Validate
→ Diff
→ Dependency Analysis
→ Live Apply

If a component supports live changes, the component receives the new configuration without restarting.

## Component Restart

Some changes may not be safely applicable while a component is running.

In those cases:

Configuration Change
→ Dependency Analysis
→ Restart Policy
→ Smallest Affected Component
→ Restart
→ Restore State

The entire application should not be restarted unless necessary.

## Transactions

Configuration changes should be treated as transactions.

A transaction can contain:

* Previous state
* Proposed state
* Validation result
* Affected components
* Apply status
* Rollback state

This prevents partially applied configuration changes from leaving Conduit in an inconsistent state.

## Rollback

If a configuration change fails, Conduit should attempt to restore the previous valid state.

For example:

Valid Configuration
→ New Configuration
→ Validation
→ Apply
→ Component Failure
→ Rollback
→ Previous Configuration

The previous configuration should remain usable whenever possible.

## Configuration Editor

The GUI configuration editor uses the same parser, schema, and validator as normal configuration loading.

This allows the editor to provide:

* Syntax highlighting
* Autocomplete
* Schema information
* Validation
* Documentation
* Diagnostics
* Preview
* Apply
* Revert

The editor should not maintain a second incompatible configuration parser.

## Profiles

Profiles provide reusable configuration groups.

For example:

```text
default
development
minimal
hardened
ssh
custom
```

Profiles can be stored as TOML resources.

User profiles can be placed in:

```text
~/.config/conduit/profiles/
```

## Workspaces

Workspaces can contain configuration in addition to layout information.

Workspace settings can override general settings for a particular working environment.

For example, a development workspace may select:

* Development profile
* Specific theme
* Specific working directories
* Specific shell
* Specific remote connections

## Resource Integration

The configuration engine integrates with the resource system.

Themes, profiles, and workspaces can be discovered and registered without hardcoding them into the application.

## CLI Integration

The CLI can use the configuration engine for commands such as:

```bash
conduit config show
conduit config validate
conduit config set
conduit profile list
conduit profile activate
```

The CLI should use the same configuration state as the GUI.

## TUI Integration

The TUI can provide configuration editing and management without implementing another configuration engine.

## GUI Integration

The GUI Settings application modifies configuration through the configuration engine.

A setting changed in the GUI should therefore behave the same as the equivalent change made in `config.toml`.

## Migration

Configuration formats may change between Conduit versions.

The migration system can:

* Detect older versions
* Convert deprecated settings
* Rename configuration keys
* Update resource formats
* Preserve compatible values

Migration should avoid destroying user configuration.

## Errors

Configuration errors should be visible and actionable.

A useful error should identify:

* What failed
* Where it failed
* Why it failed
* Whether the old configuration remains active
* How the user can correct the problem

## Security

Configuration files can influence security-sensitive behavior.

The configuration engine should therefore distinguish between ordinary settings and security-sensitive settings.

Security policies should not be accidentally weakened by malformed or ambiguous configuration.

## Configuration Philosophy

The configuration engine follows several principles:

* One configuration model
* One parser
* One schema
* One validation system
* Live updates when possible
* Transactions for changes
* Rollback on failure
* Clear errors
* No unnecessary restarts
* User-editable configuration

The goal is for users to have complete control over their configuration without requiring them to restart Conduit every time they change a setting.

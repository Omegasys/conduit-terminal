# Conduit Configuration

This directory contains Conduit's built-in configuration defaults, configuration schema, and example profiles.

The configuration system is designed around a single configuration model shared by the GUI, TUI, CLI, and configuration files.

## Directory Structure

* `defaults.toml` — Built-in default configuration
* `schema.toml` — Configuration schema and validation metadata
* `profiles/` — Built-in configuration profiles

## Profiles

Available profiles include:

* `default.toml` — Normal everyday configuration
* `minimal.toml` — Minimal interface and resource usage
* `development.toml` — Development and debugging configuration
* `security.toml` — Security-focused configuration
* `hardened.toml` — More restrictive security configuration
* `ssh.toml` — Remote SSH-oriented configuration
* `custom.toml` — Starting point for user customization

## User Configuration

The primary user configuration file is:

`~/.config/conduit/config.toml`

Users can also place resources in the Conduit configuration directories.

Common resource directories include:

`~/.config/conduit/themes/`

`~/.config/conduit/profiles/`

`~/.config/conduit/workspaces/`

System-wide resources can be installed under:

`/usr/share/conduit/`

User resources take precedence over system resources according to the configuration and resource precedence rules.

## Configuration Precedence

Conduit can combine configuration from several sources.

The general precedence order is:

1. Built-in defaults
2. System configuration
3. User configuration
4. Profile configuration
5. Workspace configuration
6. Session configuration
7. Command-line overrides

More specific configuration overrides less specific configuration when the setting permits overriding.

Security restrictions may impose additional precedence rules.

## Profiles

Profiles are reusable configuration sets.

A profile can modify:

* Terminal behavior
* Appearance
* Rendering
* Shell behavior
* Security
* Remote-session behavior
* Keybindings
* Notifications
* Performance options

Profiles should avoid duplicating unrelated configuration when inheritance or defaults can be used.

## Live Configuration

Conduit monitors supported configuration and resource files for changes.

When a configuration file changes, Conduit should:

1. Detect the change
2. Parse the configuration
3. Validate the configuration
4. Calculate the configuration difference
5. Determine affected components
6. Apply safe changes
7. Reload affected components
8. Roll back failed changes

A complete application restart should only be required when a change cannot safely be applied otherwise.

## Validation

Configuration is validated against `schema.toml` and the internal configuration model.

Invalid configuration should not replace the currently active valid configuration.

## Security

Configuration files should be treated as user-controlled input.

Conduit should validate:

* Paths
* Resource references
* Plugin configuration
* External commands
* URI schemes
* Security settings
* Resource limits

Security-sensitive settings should receive additional validation.

## Editing Configuration

Users can edit configuration through:

* GUI settings
* Configuration editor
* TUI configuration interface
* CLI commands
* Direct TOML editing

All interfaces should ultimately use the same configuration engine.

## Example CLI Commands

`conduit config get terminal.scrollback`

`conduit config set terminal.scrollback 10000`

`conduit config validate`

`conduit config diff`

`conduit config reload`

## Configuration Files and Shell History

Conduit configuration is separate from shell history.

For example, Bash normally maintains:

`~/.bash_history`

Zsh and Fish maintain their own history systems.

Conduit can integrate with shell history while maintaining optional metadata under:

`~/.local/share/conduit/history/`

Conduit should not silently replace the shell's native history mechanism.

## Custom Profile

`profiles/custom.toml` provides a starting point for creating a customized profile.

Users may copy and modify it rather than changing the built-in profiles.

## Important Note

Files in this directory represent repository-managed configuration defaults and examples.

User configuration should normally be stored under:

`~/.config/conduit/`

Local modifications to built-in files may be overwritten during upgrades.

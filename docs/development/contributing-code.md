# Conduit Code Contributions

## Overview

Conduit is designed as a modular system.

Contributions should preserve clear boundaries between the terminal core, interface layers, configuration system, resource system, security system, plugins, and platform integrations.

## Before Writing Code

Contributors should understand the relevant architecture before modifying a subsystem.

Useful documents include:

* Architecture overview
* Terminal core documentation
* Configuration documentation
* Security documentation
* UI documentation
* Protocol documentation
* Development documentation

## Design Principles

Contributed code should generally follow these principles:

* Clear ownership
* Small components
* Explicit interfaces
* Minimal coupling
* Testability
* Error handling
* Security by default
* GUI/TUI/CLI consistency

## Shared Core

Important application behavior should live in the shared core rather than being implemented separately for each interface.

For example, a command should normally be implemented through the command system rather than separately in:

* GUI code
* TUI code
* CLI code

The interfaces should invoke the same underlying command.

## Command Registry

New user-facing operations should generally be represented as commands.

A command should define:

* Stable command ID
* Description
* Parameters
* Availability
* Execution behavior
* Permission requirements when applicable

Commands can then be exposed through:

* Menus
* Toolbar
* Keybindings
* Command palette
* TUI
* CLI
* Plugins

## Event Bus

Components should use the event bus for appropriate cross-component communication.

Direct dependencies should be avoided when an event-based relationship is more appropriate.

Events should have clear ownership and semantics.

## Configuration

New configurable behavior should use the configuration engine.

Contributors should avoid creating independent configuration systems for individual components.

Configuration should include:

* Schema
* Defaults
* Validation
* Serialization
* Documentation

## Live Reload

When adding configurable behavior, contributors should determine whether it can safely support live updates.

If a setting can be reloaded, the component should expose an appropriate reload mechanism.

If it cannot, the reason should be documented.

## Resources

Themes, profiles, workspaces, and similar user-editable resources should use the resource system.

A new resource type should define:

* Resource identity
* File format
* Manifest
* Parser
* Validator
* Registration behavior
* Precedence
* Live reload behavior

## Security

Security-sensitive functionality must integrate with the central security model.

Contributors should consider:

* Trust boundaries
* Input validation
* Permissions
* Sandboxing
* Resource limits
* Untrusted output
* Sensitive data

Security checks should not be duplicated inconsistently across interfaces.

## Error Handling

Errors should provide enough context to diagnose failures.

Avoid silently ignoring errors unless the behavior is intentional and documented.

Errors should be classified appropriately.

Possible categories include:

* User input
* Configuration
* Resource
* Protocol
* IO
* Platform
* Security
* Plugin
* Internal

## Testing Requirements

New functionality should include appropriate tests.

Depending on the feature, this may include:

* Unit tests
* Integration tests
* Regression tests
* Security tests
* Protocol tests
* UI tests
* Performance tests

## Documentation

User-facing behavior should be documented.

Configuration options should be added to the appropriate configuration documentation.

New commands should be documented.

Security-sensitive behavior should be documented in the security documentation.

## Code Style

Contributors should follow the project's Rust formatting and linting configuration.

Code should prioritize readability and maintainability.

Avoid clever implementations when a straightforward implementation is clearer.

## Dependencies

New dependencies should have a clear justification.

Contributors should consider:

* Maintenance status
* License compatibility
* Security history
* Platform support
* Build complexity
* Runtime overhead

## Platform-Specific Code

Platform-specific functionality should remain isolated behind appropriate interfaces.

Linux-specific behavior should not unnecessarily leak into platform-independent terminal code.

## Plugins

Functionality that belongs naturally outside the core can be considered for the plugin system.

However, security-sensitive or performance-critical functionality may need to remain within trusted core components.

## Experimental Features

Experimental functionality should be clearly separated from stable functionality.

Experimental features should not silently alter stable behavior.

## Pull Requests

A code contribution should explain:

* What changed
* Why it changed
* Which components are affected
* Configuration changes
* Security implications
* Testing performed
* Performance implications when relevant

## Review

Code review should examine:

* Correctness
* Architecture
* Security
* Performance
* Testing
* Documentation
* Maintainability

## Small Changes

Small, focused contributions are preferred when practical.

A contribution that changes several unrelated subsystems should consider being divided into multiple changes.

## Backward Compatibility

Changes to public configuration keys, commands, plugin APIs, or resource formats should consider compatibility.

Deprecated interfaces should have a documented migration path when practical.

## Design Principles for Contributors

Contributors should aim to make Conduit:

* More modular
* More understandable
* More testable
* More secure
* More accessible
* More performant
* More maintainable

The goal is not simply to add features, but to preserve the architecture that allows Conduit to continue evolving.

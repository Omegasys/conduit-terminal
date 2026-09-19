# Conduit Debugging

## Overview

Conduit includes a layered debugging system for identifying problems in the terminal core, interface, configuration system, resource system, rendering pipeline, plugins, sessions, and platform integrations.

Debugging should provide useful information without exposing sensitive terminal or user data.

## Debugging Goals

The debugging system should make it possible to determine:

* What failed
* Where it failed
* What caused the failure
* Which component was affected
* What configuration was active
* Whether recovery occurred
* Whether a restart was required

## Debugging Layers

Conduit debugging is divided into several layers.

### Application

General application lifecycle and state.

### Interface

GUI, TUI, and CLI behavior.

### Configuration

Configuration parsing, validation, transactions, and reloads.

### Resources

Themes, profiles, workspaces, and other dynamically discovered resources.

### Terminal Core

PTYs, terminal state, cursor state, scrollback, and sessions.

### Protocols

ANSI, VT, xterm, OSC, graphics, mouse, and related protocol processing.

### Rendering

GPU, software rendering, fonts, images, scaling, and frame generation.

### Input

Keyboard, mouse, focus, and input translation.

### Plugins

Plugin loading, permissions, sandboxing, and API calls.

### Platform

Wayland, X11, Linux APIs, filesystem integration, and other platform-specific functionality.

## Logging

Conduit should provide structured logging.

Each log entry can contain:

* Timestamp
* Log level
* Component
* Event type
* Session ID
* Workspace ID
* Correlation ID
* Message
* Diagnostic metadata

Sensitive values should be redacted.

## Log Levels

Suggested levels include:

* `trace`
* `debug`
* `info`
* `warn`
* `error`

The default logging level should avoid excessive output.

## Debug Mode

Conduit can provide a debug mode that enables additional diagnostics.

Debug mode may expose:

* Component state
* Event routing
* Configuration transactions
* Resource discovery
* Protocol processing
* Renderer statistics
* Plugin lifecycle
* Session lifecycle

Debug mode must not disable security controls.

## Diagnostics Panel

The diagnostics panel provides a centralized view of application problems.

It can display:

* Errors
* Warnings
* Configuration problems
* Resource validation failures
* Plugin failures
* Renderer problems
* Session failures
* Performance warnings

## Error Correlation

Related events should share a correlation identifier.

This allows a user or developer to trace an operation across multiple components.

For example:

`Configuration Change → Validation → Transaction → Renderer Reload`

can be inspected as one operation.

## Flow View Integration

Debugging integrates with Flow View.

Flow View can display:

* Event paths
* Component transitions
* Processing failures
* Reload operations
* Security decisions
* Restart operations

## Terminal Output Debugging

Terminal output should be inspectable without unnecessarily exposing it in logs.

Useful debugging information includes:

* Parser state
* Sequence type
* Sequence length
* Control type
* Processing result
* Security decision

Raw sensitive terminal output should not automatically be logged.

## Protocol Debugging

Protocol debugging can identify:

* Unsupported sequences
* Malformed sequences
* Invalid parameters
* Unknown commands
* Parser state transitions
* Graphics protocol errors

Protocol debugging should enforce the same input limits and security boundaries as normal operation.

## Configuration Debugging

Configuration diagnostics should identify:

* File location
* Configuration source
* Key
* Value type
* Validation result
* Dependency
* Affected component

Sensitive configuration values should be redacted.

## Resource Debugging

Resource debugging can show:

* Resource path
* Resource type
* Discovery status
* Validation status
* Registration status
* Active state
* Reload status

This is particularly useful when troubleshooting themes, profiles, and workspaces.

## Plugin Debugging

Plugin diagnostics should include:

* Plugin identifier
* Version
* Load state
* Permission state
* Sandbox state
* API errors
* Resource usage

Plugin output should remain isolated from trusted application logs where appropriate.

## Crash Handling

When Conduit crashes, it should attempt to provide useful diagnostics.

Possible information includes:

* Application version
* Build information
* Platform information
* Component state
* Recent diagnostic events
* Crash location
* Active configuration identifiers

Sensitive terminal content should not automatically be included.

## Safe Recovery

Debugging must not prevent Conduit from recovering from failures.

Possible recovery actions include:

* Reload component
* Restart component
* Disable problematic resource
* Disable third-party plugin
* Switch renderer
* Enter safe mode

## Debugging Commands

Example CLI commands include:

`conduit diagnostics`

`conduit diagnostics --verbose`

`conduit logs`

`conduit config validate`

`conduit resources diagnose`

`conduit plugins diagnose`

`conduit flow trace`

## Debugging Information Bundles

Conduit can provide an optional diagnostic bundle containing relevant logs and system information.

Users should be able to review the bundle before sharing it.

Sensitive information should be redacted automatically where possible.

## Design Principles

Debugging should be:

* Structured
* Actionable
* Privacy-aware
* Security-aware
* Component-oriented
* Correlatable
* Accessible
* Useful across GUI, TUI, and CLI

Debugging should help explain Conduit's behavior without becoming a source of unnecessary information leakage.

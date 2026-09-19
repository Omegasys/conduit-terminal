# Conduit Event Bus

The Conduit event bus provides communication between independent components.

It allows subsystems to communicate through events instead of requiring direct dependencies between every component.

## Location

The event system is located under:

```text
src/events/
```

Primary components include:

```text
bus.rs
event.rs
command.rs
dispatcher.rs
subscriptions.rs
filters.rs
priorities.rs
```

## Purpose

The event bus exists to reduce coupling.

Instead of:

Theme Manager
→ Directly modifies GUI
→ Directly modifies Renderer
→ Directly modifies Diagnostics
→ Directly modifies Settings

Conduit can use:

Theme Manager
→ Theme Changed Event
→ Event Bus
→ Interested Components

This makes components easier to modify, test, and replace.

## Commands and Events

Commands and events have different purposes.

A command represents a request.

Examples:

* Create tab
* Close pane
* Change theme
* Start recording
* Switch workspace

An event represents something that happened.

Examples:

* Tab created
* Pane closed
* Theme changed
* Recording started
* Workspace switched

The general relationship is:

Command
→ Action
→ State Change
→ Event

## Command Dispatcher

Commands are routed through the command system.

For example:

```text
GUI
→ Create Tab Command
→ Command Dispatcher
→ Tab Manager
→ New Tab
→ Tab Created Event
→ Event Bus
```

The CLI and TUI can perform the same operation through the same command path.

## Event Types

Events can represent changes throughout Conduit.

Examples include:

### Application Events

* Application started
* Application ready
* Application shutting down
* Application state changed

### Window Events

* Window created
* Window closed
* Window resized
* Window moved
* Window state changed

### Tab Events

* Tab created
* Tab closed
* Tab selected
* Tab renamed
* Tab reordered

### Pane Events

* Pane created
* Pane closed
* Pane resized
* Pane focused
* Pane layout changed

### Session Events

* Session created
* Session started
* Session stopped
* Session exited
* Session restored

### Terminal Events

* Output received
* Screen changed
* Cursor changed
* Scrollback changed
* Terminal resized

### Configuration Events

* Configuration loaded
* Configuration changed
* Configuration validation failed
* Configuration rolled back

### Resource Events

* Resource discovered
* Resource changed
* Resource removed
* Resource validation failed
* Resource reloaded

### Plugin Events

* Plugin discovered
* Plugin loaded
* Plugin unloaded
* Plugin permission changed
* Plugin failed

### Security Events

* Security policy changed
* Permission requested
* Permission granted
* Permission denied
* Security warning

### Diagnostic Events

* Diagnostic state changed
* Performance warning
* Renderer state changed
* Resource error

## Event Payloads

Events should contain enough information for subscribers to understand what changed without requiring them to directly inspect unrelated components.

For example, a theme-changed event can include:

* Theme identifier
* Previous theme
* New theme
* Source path
* Validation state

## Subscriptions

Components subscribe to events they care about.

For example:

Renderer
→ Subscribe to Theme Changed

Diagnostics
→ Subscribe to Renderer Changed

GUI
→ Subscribe to Tab Changed

History
→ Subscribe to Command Completed

This keeps each component focused on its own responsibilities.

## Filters

Subscriptions can use filters to reduce unnecessary event processing.

A component can subscribe to:

* Specific event types
* Specific resource identifiers
* Specific sessions
* Specific workspaces
* Specific priority levels

## Priorities

Some events may require prioritized processing.

For example:

* Security events
* Session termination
* Application shutdown
* Critical configuration changes

Priority handling should remain deterministic.

## Event Ordering

Events that affect the same piece of state should be processed in a predictable order.

The event system should avoid situations where dependent state changes are observed in an impossible order.

## Asynchronous Events

Some events originate from asynchronous operations.

Examples include:

* PTY output
* Remote connections
* File watching
* Resource discovery
* Plugin operations
* Diagnostics

The event bus can deliver these events without blocking the UI.

## Event Ownership

The component that detects a state change should normally publish the corresponding event.

For example:

Resource Manager
→ Detects theme change
→ Publishes Theme Changed

The GUI should not need to poll the resource manager repeatedly.

## Configuration and Events

Configuration changes are a major use of the event bus.

The process is:

Configuration File
→ Configuration Engine
→ Validate
→ Diff
→ Apply
→ Event Bus
→ Affected Components

For example:

```text
theme = "nord"
```

can produce:

```text
ThemeChanged("nord")
```

The renderer and GUI can then update themselves.

## Resource and Events

Resource discovery uses the same system.

For example:

```text
New Theme File
→ Resource Watcher
→ Resource Manager
→ Theme Registered
→ Event Bus
→ Theme UI Updated
```

This is what allows newly added themes to appear without restarting Conduit.

## Terminal Output

Terminal output can generate events when appropriate.

The PTY produces data:

```text
PTY
→ Terminal Parser
→ Terminal State
→ Terminal Changed Event
```

The renderer can then update the visible terminal.

Not every byte necessarily needs to become an individual high-level event.

The architecture should allow batching for performance.

## Event Batching

High-frequency operations such as terminal output may produce very large numbers of updates.

The event system should therefore support batching where appropriate.

For example:

```text
PTY Output
→ Parse Large Output Block
→ Update Terminal State
→ One Batched Terminal Update
→ Renderer
```

This reduces unnecessary overhead.

## Event Bus and Flow View

Flow View can observe selected events from the event bus.

This allows users to visualize operations such as:

```text
Configuration
→ Config Engine
→ Event Bus
→ Renderer
```

or:

```text
Shell
→ PTY
→ Terminal
→ Pane
→ Renderer
```

Flow View should be able to filter high-frequency events to remain usable.

## Event Logging

Diagnostics can optionally record event information.

This can help diagnose:

* Configuration failures
* Resource reload problems
* Plugin failures
* Session problems
* Renderer problems

Sensitive event data should not be logged unnecessarily.

## Security

The event bus itself should not bypass security boundaries.

A plugin receiving an event should only receive information that its permissions allow it to access.

Events may therefore require filtering or capability checks before being delivered to restricted components.

## Plugin Events

Plugins can publish and subscribe to supported events through the plugin API.

Plugins should not receive unrestricted access to internal event types.

The plugin system should expose a controlled event interface.

## Error Handling

Event handlers should not be allowed to destabilize the entire application.

A handler failure should be isolated where practical.

For example:

Plugin Event Handler
→ Error
→ Plugin Error State

rather than:

Plugin Event Handler
→ Application Crash

## Shutdown

The event system should support orderly shutdown.

A typical shutdown sequence is:

Shutdown Command
→ Application Shutdown Event
→ Stop New Work
→ Close Sessions
→ Stop Plugins
→ Stop Watchers
→ Stop Event Processing
→ Exit

## Design Principles

The event bus follows these principles:

* Loose coupling
* Explicit event types
* Predictable ordering
* Controlled subscriptions
* Batched high-frequency updates
* Safe asynchronous processing
* Security-aware delivery
* Observable behavior
* Shared use across GUI, TUI, CLI, and plugins

The event bus is one of the central pieces that allows Conduit to remain modular while still behaving as a single integrated application.

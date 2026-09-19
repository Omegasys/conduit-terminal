# Conduit Flow System

## Overview

The Flow System represents how information, processes, terminal sessions, events, and rendering components move through Conduit.

The system provides the architectural foundation for Flow View while also giving developers a way to reason about Conduit's internal data paths.

The concept is inspired by the same fundamental idea as the Unix pipe:

> Information flows through connected components.

## Architecture

The Flow System is implemented under:

```text
src/flow/
├── engine.rs
├── node.rs
├── edge.rs
├── process.rs
├── pty.rs
├── terminal.rs
├── pane.rs
├── event.rs
├── graph.rs
└── filters.rs
```

## Flow Graph

Conduit represents relevant components as nodes connected by edges.

A simplified terminal flow is:

```text
Process
   ↓
PTY
   ↓
Terminal Protocol Parser
   ↓
Terminal State
   ↓
Pane
   ↓
Renderer
   ↓
Display
```

Input flows in the opposite direction:

```text
Keyboard / Mouse
       ↓
Input System
       ↓
Pane
       ↓
PTY
       ↓
Process
```

## Nodes

A node represents a component or entity participating in a flow.

Possible node types include:

* Process
* PTY
* Session
* Terminal
* Pane
* Tab
* Workspace
* Renderer
* Event Bus
* Plugin
* Connection
* Configuration
* Resource
* Command

## Edges

Edges represent relationships or data flow between nodes.

Examples include:

```text
Process → PTY
PTY → Terminal
Terminal → Renderer
Configuration → Component
Resource → Registry
Event → Subscriber
Command → Dispatcher
```

## Flow Engine

The flow engine maintains the runtime representation of relevant flows.

It should provide:

* Node registration
* Node removal
* Edge creation
* Edge removal
* State updates
* Graph queries
* Filtering
* Event integration

The flow engine should not replace the actual event bus or process manager.

It represents those systems rather than becoming responsible for their underlying work.

## Event Integration

The Flow System receives information from the central event bus.

For example:

```text
SessionCreated
      ↓
Flow Engine
      ↓
Create Session Node
```

And:

```text
SessionExited
      ↓
Flow Engine
      ↓
Update Session Node
```

## Flow View

Flow View is the graphical representation of the Flow System.

It can visualize:

* Active sessions
* Processes
* PTYs
* Panes
* Tabs
* Workspaces
* Connections
* Plugins
* Events
* Rendering paths
* Configuration changes

## Interactive Flow View

Flow View should be more than a static diagram.

Users may be able to:

* Select nodes
* Inspect details
* Follow connections
* Filter node types
* Show/hide events
* Highlight active paths
* Inspect errors
* View resource dependencies
* Open related diagnostics
* Navigate to the corresponding session or pane

## Filtering

The flow system supports filters.

Examples:

```text
Show only sessions
Show only processes
Show only remote connections
Show only rendering components
Show only configuration flows
Show only errors
Show only active flows
```

## Flow State

Nodes can expose state information.

Examples include:

* Running
* Idle
* Waiting
* Error
* Disconnected
* Reloading
* Restarting
* Disabled

The exact visual representation belongs to the UI layer.

## Configuration Flow

Configuration changes can also be represented.

Example:

```text
config.toml
     ↓
Configuration Engine
     ↓
Validation
     ↓
Configuration State
     ↓
Dependency Graph
     ↓
Affected Components
     ↓
Live Reload
```

## Resource Flow

Resource discovery can be visualized as:

```text
Theme File
    ↓
Resource Discovery
    ↓
Validation
    ↓
Resource Registry
    ↓
Theme Manager
    ↓
Active Theme
    ↓
Renderer
```

## Plugin Flow

Plugins can be represented as:

```text
Plugin File
    ↓
Plugin Discovery
    ↓
Manifest
    ↓
Validation
    ↓
Permissions
    ↓
Plugin Loader
    ↓
Plugin API
    ↓
Conduit
```

## Remote Flow

Remote connections can be represented as:

```text
User
 ↓
Connection Manager
 ↓
SSH / Serial / Container
 ↓
Remote Process
 ↓
PTY / Transport
 ↓
Terminal
 ↓
Renderer
```

## Flow History

The Flow System may optionally maintain short-lived flow history for diagnostics.

This should not automatically become a permanent activity log.

Users should be able to configure retention and privacy behavior.

## Performance

Flow tracking should not significantly affect terminal performance.

The system should support:

* Event batching
* Lightweight node state
* Incremental updates
* Configurable history
* Lazy graph expansion
* Filtering before rendering

## Security

Flow View must avoid exposing sensitive information unnecessarily.

Potentially sensitive information includes:

* Command arguments
* Environment variables
* Hostnames
* File paths
* Authentication information
* Plugin data

Sensitive values should be redacted according to security policy.

## Debugging

Flow View can assist developers in diagnosing:

* Broken event paths
* Stalled sessions
* Renderer failures
* Configuration reload failures
* Plugin problems
* Remote connection issues
* Unexpected component restarts

## Design Goal

The Flow System makes Conduit's internal architecture observable.

The principle is:

> If information flows through Conduit, developers should have a way to understand that flow.

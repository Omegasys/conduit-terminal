# Conduit Terminal Core

The terminal core is the central runtime responsible for managing terminal sessions, processes, PTYs, terminal state, input, output, scrollback, and shell interaction.

The core is intentionally independent from the graphical interface.

## Responsibilities

The terminal core is responsible for:

* Terminal sessions
* Process management
* PTY management
* Terminal screen state
* Cursor state
* Scrollback
* Environment state
* Signals
* Session lifecycle
* Terminal input
* Terminal output

The core should not be responsible for drawing GUI widgets or deciding how a terminal is presented visually.

## Core Directory

The implementation is located under:

```text
src/core/
```

Primary components include:

```text
terminal.rs
session.rs
process.rs
pty.rs
screen.rs
cursor.rs
scrollback.rs
state.rs
environment.rs
signals.rs
lifecycle.rs
```

## Terminal Session

A terminal session represents an active terminal environment.

A session generally contains:

* PTY
* Process
* Shell
* Terminal state
* Environment
* Working directory
* Dimensions
* Scrollback
* Cursor state
* Connection information

A session can exist independently from a specific GUI widget.

This allows sessions to survive UI changes and potentially be moved between panes or restored later.

## PTY

The PTY layer provides the communication channel between Conduit and terminal processes.

The general flow is:

Conduit
→ PTY
→ Shell
→ Child Processes

Output follows the reverse direction:

Child Process
→ Shell
→ PTY
→ Conduit

The PTY subsystem is responsible for:

* Creating PTYs
* Resizing PTYs
* Reading output
* Writing input
* Handling process state
* Closing PTYs
* Sending signals

## Process Management

The process subsystem manages the processes associated with terminal sessions.

Responsibilities include:

* Starting processes
* Tracking process IDs
* Tracking exit status
* Sending signals
* Detecting process termination
* Cleaning up resources

Conduit should distinguish between the terminal session and the process running inside it.

A session may remain available for state restoration even after its process exits.

## Shells

Shell integration is implemented separately from the core process system.

Supported shells include:

* Bash
* Zsh
* Fish
* PowerShell

The shell integration layer can provide additional information such as:

* Current working directory
* Command boundaries
* Command duration
* Exit status
* Prompt state

The core should continue functioning even when shell integration is unavailable.

## Terminal State

The terminal state represents the logical contents of the terminal.

It can include:

* Screen cells
* Rows
* Columns
* Cursor position
* Cursor visibility
* Cursor style
* Active character attributes
* Alternate screen state
* Scroll regions
* Selection state

The terminal state should be independent from the renderer.

## Screen Buffer

The screen buffer stores the currently visible terminal contents.

Terminal applications can modify this state through escape sequences.

For example, an application may:

* Move the cursor
* Clear a region
* Write text
* Change colors
* Change attributes
* Switch screens

The parser updates terminal state rather than directly manipulating the renderer.

## Scrollback

Scrollback stores terminal output that has moved beyond the visible screen.

The scrollback system supports:

* Configurable line counts
* Searching
* Selection
* History navigation
* Session restoration where supported

Scrollback should be optimized for long-running sessions.

## Cursor

Cursor state includes:

* Position
* Visibility
* Shape
* Blink state
* Current attributes

Cursor rendering is handled by the renderer, while cursor state belongs to the terminal core.

## Terminal Protocol Processing

Terminal protocol handling is located under:

```text
src/terminal_protocols/
```

The parser converts terminal byte streams into structured operations.

The conceptual pipeline is:

PTY Output
→ Byte Stream
→ Protocol Parser
→ Terminal Operations
→ Terminal State

This separation allows protocol support to expand without rewriting the terminal state system.

## Input

User input follows the opposite direction:

Keyboard / Mouse
→ Input System
→ Terminal Session
→ PTY
→ Process

The terminal core should not need to know whether input originated from the GUI, TUI, automation, or another interface.

## Terminal Dimensions

The core tracks terminal dimensions in rows and columns.

When a pane is resized:

Pane
→ Terminal Resize
→ PTY Resize
→ Shell / Process Notification

This ensures terminal applications receive accurate terminal dimensions.

## Signals

The signal subsystem manages process signals.

Common operations include:

* Interrupt
* Terminate
* Hang up
* Suspend
* Continue
* Resize notification

The exact behavior depends on the operating system and process.

## Environment

Each session can have its own environment.

Environment information may include:

* PATH
* HOME
* SHELL
* TERM
* Working directory
* Locale
* User-defined variables

Environment state should remain associated with the session rather than with the GUI window.

## Session Lifecycle

A session generally follows:

Create
→ Initialize
→ Spawn PTY
→ Spawn Process
→ Running
→ Process Exit
→ Cleanup
→ Closed

The lifecycle subsystem ensures resources are released correctly.

## Session Restoration

When session restoration is enabled, Conduit can preserve metadata needed to restore a previous workspace.

Restoration should not pretend that a terminated process is still running.

Instead, Conduit can restore the surrounding workspace and optionally start a new process.

## Multiple Sessions

A single Conduit process can manage many sessions.

For example:

Window
→ Tab
→ Pane
→ Session

Different panes can contain:

* Bash
* Zsh
* SSH
* Serial
* Docker
* Podman
* Other supported sessions

## Remote Sessions

Remote sessions use the same terminal abstraction.

For example:

SSH Connection
→ Remote PTY
→ Terminal Session
→ Pane

This means the renderer and UI do not need separate terminal implementations for local and remote sessions.

## Terminal and Renderer Separation

The terminal core should never directly depend on a particular rendering backend.

The renderer receives terminal state and produces visual output.

This allows:

* GPU rendering
* Software rendering
* OpenGL
* Vulkan
* Wayland
* X11

to coexist without changing the terminal state model.

## Threading and Concurrency

Terminal sessions can generate output continuously.

The architecture should therefore avoid blocking the UI while processing terminal output.

Long-running operations should be handled asynchronously where appropriate.

Examples include:

* PTY reads
* Remote connections
* Plugin operations
* File operations
* Resource discovery
* Diagnostics

The event bus can be used to communicate state changes between asynchronous components.

## Security

The terminal core must treat terminal output as untrusted input.

Terminal applications can emit escape sequences containing:

* Hyperlinks
* Clipboard operations
* Title changes
* Color changes
* Graphics
* Other control operations

Security-sensitive operations should pass through the security subsystem before being performed.

## Core Independence

The core should be usable without the full GUI.

This supports:

* TUI
* CLI
* Testing
* Headless operation
* Automated environments
* Remote management

The core therefore forms the foundation of the entire Conduit architecture.

## Design Principle

The terminal core should answer:

"What is happening in the terminal?"

It should not answer:

"How should the user interface display it?"

That distinction keeps Conduit modular and allows the UI and renderer to evolve independently from terminal behavior.

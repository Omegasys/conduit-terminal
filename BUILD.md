# Conduit Build Guide

This document describes how to build Conduit from source.

## Build Philosophy

Conduit is designed as a modular Rust application.

The GUI, TUI, and CLI use the same underlying Conduit core rather than implementing separate terminal engines.

The architecture is approximately:

GUI
→ Command API
→ Event Bus
→ Conduit Core
→ Sessions / PTYs / Shells
→ Renderer

TUI and CLI use the same core interfaces.

Configuration changes follow a similar path:

Configuration File or GUI
→ Configuration Engine
→ Validation
→ Diff
→ Event Bus
→ Affected Component

When a configuration change requires restarting something, Conduit should restart the smallest affected component instead of restarting the entire application whenever possible.

## Build Requirements

Recommended development environment:

* Linux
* Rust
* Cargo
* Git
* C compiler
* System development libraries required by the selected GUI and rendering backends

Additional development dependencies may be required for:

* Wayland
* X11
* OpenGL
* Vulkan
* GPU acceleration
* Desktop notifications
* Image rendering
* Sixel
* Kitty graphics
* iTerm2 graphics
* SSH
* Serial communication
* Container integrations

## Rust Toolchain

The repository contains:

```text
rust-toolchain.toml
```

Use the repository's configured Rust toolchain.

Check the installed toolchain:

```bash
rustc --version
cargo --version
```

## Clone the Repository

```bash
git clone https://github.com/your-username/conduit.git
cd conduit
```

## Check the Project

Run:

```bash
cargo check
```

This checks the project without producing a final optimized executable.

## Development Build

Build the normal development version:

```bash
cargo build
```

The binary will normally be located at:

```text
target/debug/conduit
```

Run it:

```bash
cargo run
```

## Release Build

Build an optimized release version:

```bash
cargo build --release
```

The resulting binary will normally be:

```text
target/release/conduit
```

## Run Tests

Run the complete test suite:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

## Formatting

Format the project:

```bash
cargo fmt
```

Check formatting without modifying files:

```bash
cargo fmt -- --check
```

## Linting

Run Clippy:

```bash
cargo clippy
```

For stricter CI checking:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

## Full Development Check

A typical development verification sequence is:

```bash
cargo fmt -- --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Feature Flags

Conduit is designed to support optional functionality through feature flags.

Possible feature groups include:

* GUI
* TUI
* CLI
* Wayland
* X11
* OpenGL
* Vulkan
* GPU rendering
* Sixel
* Kitty graphics
* iTerm2 graphics
* SSH
* Serial
* Docker
* Podman
* Kubernetes
* Plugins
* Recording
* Accessibility
* Experimental features

Available features should be documented in `Cargo.toml`.

List available Cargo commands with:

```bash
cargo metadata
```

## Building Without Optional Components

Conduit should support minimal builds where practical.

A minimal build may disable components such as:

* GUI integrations
* optional graphics protocols
* plugins
* remote connection integrations
* experimental functionality

The exact feature set is controlled by `Cargo.toml`.

## Development Configuration

For development, configuration can be stored at:

```text
~/.config/conduit/config.toml
```

Example configurations are available under:

```text
examples/config/
```

## Resource Development

Conduit resources are designed to be discovered dynamically.

Themes:

```text
themes/
```

Profiles:

```text
profiles/
```

Workspaces:

```text
workspaces/
```

During development, user resource directories can be used to test discovery and live reload.

For example:

```text
~/.config/conduit/themes/test-theme.toml
```

Creating or modifying the resource should trigger the resource watcher.

## Live Reload Testing

Conduit should detect changes to supported resources without restarting the entire application.

Important test cases include:

* Creating a theme
* Modifying a theme
* Deleting a theme
* Creating a profile
* Modifying a profile
* Creating a workspace
* Modifying a workspace
* Modifying `config.toml`
* Introducing an invalid configuration
* Restoring a previously valid configuration

When a reload cannot safely occur live, the reload manager should determine the smallest component that must be restarted.

## Debug Builds

Debug builds are recommended during development:

```bash
cargo build
```

They provide additional debugging information and are generally faster to compile than release builds.

## Performance Builds

For performance testing:

```bash
cargo build --release
```

Benchmarks are stored under:

```text
benchmarks/
```

## Running Benchmarks

Run all available benchmarks:

```bash
cargo bench
```

Important benchmark areas include:

* Renderer performance
* Scrollback
* Input processing
* Startup
* Memory usage
* Configuration processing
* Resource discovery
* Event bus performance
* History processing
* Large terminal sessions

## Packaging

Packaging configurations are stored under:

```text
packaging/
```

Supported packaging targets include:

* Debian
* RPM
* Arch Linux
* Flatpak
* Snap
* AppImage

Packaging scripts should use the same release artifacts produced by the main build system whenever possible.

## Development Scripts

Helper scripts are located in:

```text
scripts/
```

Examples:

```bash
./scripts/build.sh
./scripts/test.sh
./scripts/format.sh
./scripts/lint.sh
./scripts/check.sh
./scripts/package.sh
```

## Clean Build

Remove build artifacts:

```bash
cargo clean
```

Then rebuild:

```bash
cargo build
```

## CI Expectations

Before submitting a change, developers should normally run:

```bash
cargo fmt -- --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Changes involving configuration, resources, live reload, terminal protocols, security, or plugins should also include appropriate tests.

## Architecture Changes

Changes that alter major subsystem boundaries should be reflected in:

```text
ARCHITECTURE.md
```

Changes that introduce user-visible functionality should also be reflected in:

```text
FEATURES.md
```

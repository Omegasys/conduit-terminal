# Conduit Installation

Conduit is a modular Linux terminal emulator with GUI, TUI, and CLI interfaces, live configuration, resource discovery, profiles, themes, workspaces, plugins, remote connections, terminal recording, and security controls.

## Requirements

Conduit currently targets Linux systems.

Recommended requirements:

* Linux
* Rust toolchain
* Cargo
* A working system compiler toolchain
* GTK or the GUI toolkit selected by the current Conduit build
* Wayland or X11
* A POSIX-compatible shell
* Git

Optional dependencies may be required for specific features such as:

* Wayland integration
* X11 integration
* GPU rendering
* Sixel graphics
* Kitty graphics
* Image previews
* SSH
* Serial connections
* Docker
* Podman
* Kubernetes
* Desktop notifications

## Install From Source

Clone the repository:

```bash
git clone https://github.com/your-username/conduit.git
cd conduit
```

Build Conduit:

```bash
cargo build --release
```

Install the resulting binary:

```bash
install -Dm755 target/release/conduit ~/.local/bin/conduit
```

Make sure `~/.local/bin` is in your `PATH`.

Check the installation:

```bash
conduit --version
```

## First Launch

Start Conduit:

```bash
conduit
```

Conduit will create its user configuration directories as needed.

The primary configuration file is:

```text
~/.config/conduit/config.toml
```

User resources are stored in:

```text
~/.config/conduit/
├── config.toml
├── themes/
├── profiles/
└── workspaces/
```

Conduit data is stored separately:

```text
~/.local/share/conduit/
├── history/
├── recordings/
├── sessions/
├── diagnostics/
└── state/
```

## Shell History

Conduit does not replace the history system of the shell running inside it.

For Bash, the normal history file remains:

```text
~/.bash_history
```

Zsh, Fish, and PowerShell retain their respective history systems.

Conduit can integrate with these histories and provide additional search, metadata, filtering, and history-management features without taking ownership of the shell's native history file.

## Themes

Conduit automatically discovers themes from supported resource directories.

A user theme can be placed in:

```text
~/.config/conduit/themes/
```

For example:

```text
~/.config/conduit/themes/my-theme.toml
```

After the file is created, Conduit can detect it automatically.

Theme changes can be applied without restarting Conduit when the affected configuration supports live reloading.

Invalid themes are rejected without replacing the currently active valid theme.

## Profiles

User profiles can be stored in:

```text
~/.config/conduit/profiles/
```

Example:

```text
~/.config/conduit/profiles/development.toml
```

Profiles can define groups of settings for different environments.

## Workspaces

User workspaces can be stored in:

```text
~/.config/conduit/workspaces/
```

A workspace can describe terminal layouts, panes, tabs, sessions, working directories, connections, and other workspace state.

## System-Wide Resources

System installations may provide resources in locations such as:

```text
/usr/share/conduit/themes/
/usr/share/conduit/profiles/
/usr/share/conduit/workspaces/
```

User resources take precedence over system resources when both provide the same resource identifier.

## Configuration

Conduit uses TOML for configuration.

The main configuration file is:

```text
~/.config/conduit/config.toml
```

An example configuration is included in:

```text
config.example.toml
```

You can copy it as a starting point:

```bash
mkdir -p ~/.config/conduit
cp config.example.toml ~/.config/conduit/config.toml
```

## GUI Settings

Most settings can be changed through the Conduit Settings application.

Changes made through the GUI use the same configuration engine as changes made directly to `config.toml`.

Where possible, configuration changes are applied immediately.

## TUI

Launch the terminal user interface with:

```bash
conduit --tui
```

The TUI provides access to core Conduit functionality without requiring the full graphical interface.

## CLI

Display available commands:

```bash
conduit --help
```

Examples:

```bash
conduit config show
conduit theme list
conduit profile list
conduit workspace list
conduit diagnostics
```

## Uninstall

If Conduit was installed manually into `~/.local/bin`:

```bash
rm ~/.local/bin/conduit
```

User configuration and data can be removed separately.

Configuration:

```text
~/.config/conduit/
```

Application data:

```text
~/.local/share/conduit/
```

Remove these directories only if you also want to remove your Conduit settings, resources, recordings, sessions, history metadata, and other stored application data.

## Troubleshooting

Display diagnostic information:

```bash
conduit diagnostics
```

Check the installed version:

```bash
conduit --version
```

Run Conduit in a minimal mode:

```bash
conduit --minimal
```

Run safe mode:

```bash
conduit --safe-mode
```

Safe mode can be used to diagnose problems involving plugins, custom resources, configuration, or other optional components.

## Development Installation

For development, clone the repository and use Cargo directly:

```bash
git clone https://github.com/your-username/conduit.git
cd conduit
cargo check
cargo test
cargo run
```

See `BUILD.md` for the complete development build process.

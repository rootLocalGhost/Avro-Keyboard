# Avro Keyboard on Arch Linux & Hyprland

This guide details how to install and configure Avro Keyboard with the Quickshell floating topbar on Arch Linux and the Hyprland Wayland compositor.

## Installation

You can install the package using `makepkg`:

```bash
cd pkg
makepkg -si
```

This will build the Avro core library and install the python daemon, CLI, Quickshell configuration, and systemd user unit.

## Enable Systemd Service

Enable and start the daemon so that it runs in the background and listens for IPC calls on `/tmp/avro.sock`.

```bash
systemctl --user enable --now avro.service
```

## Hyprland Configuration

Add the following snippets to your `~/.config/hyprland/hyprland.conf`:

### Auto-start
Ensure both the daemon and the Quickshell UI auto-start:

```ini
# Start the topbar UI
exec-once = qs -c /usr/share/avro-keyboard/quickshell/avro-topbar/shell.qml
```
*(Note: avro-daemon is started automatically by systemd, so there is no need to launch it here).*

### Keybinds
Bind a key (e.g., F12) to globally toggle the Avro input mode:

```ini
bind = , F12, exec, avro-cli toggle
```

### Window & Layer Rules
Configure Wayland LayerShell rules for the Quickshell floating bar.

```ini
layerrule = blur, avro-bar
layerrule = noanim, avro-bar
```

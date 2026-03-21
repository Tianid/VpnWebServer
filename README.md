# haven

A synchronous Rust TCP/WebSocket server for the **Raspberry Pi 5** that provides a browser-based control panel for [AdGuard VPN CLI](https://adguard-vpn.com/en/blog/adguard-vpn-cli.html).

> **No authentication, no TLS** — intended for use on a trusted local network only.

---

## Features

- Live VPN status indicator with animated states (connecting / disconnecting / reconnecting)
- Dynamic location list sorted by ping — select a city or connect to the fastest
- Reconnect Wi-Fi and system restart buttons
- Live server log panel with runtime log-level control
- English / Russian UI toggle (persisted per session)
- WebSocket auto-reconnect with exponential backoff

---

## Requirements (Raspberry Pi)

- Raspberry Pi 5 running Pi OS / Debian (glibc)
- [`adguardvpn-cli`](https://adguard-vpn.com/en/blog/adguard-vpn-cli.html) installed and authenticated
- `nmcli` (NetworkManager CLI) available
- The OS user running the binary must have permission to call `adguardvpn-cli`, `nmcli`, and `shutdown -r now`

---

## Quick Start (download release)

1. Download the latest `.tar.gz` from the [Releases](../../releases) page.
2. Extract and enter the directory:
   ```sh
   tar -xzf haven-v*.tar.gz
   cd haven-v*-aarch64-linux-gnu/
   ```
3. Run:
   ```sh
   ./haven -a 0.0.0.0 -p 9000
   ```
4. Open `http://<raspberry-pi-ip>:9000` in a browser on the same network.

The `resources/` directory **must remain alongside the binary**.

---

## CLI Options

| Flag | Short | Default | Description |
|---|---|---|---|
| `--address` | `-a` | `127.0.0.1` | Bind address |
| `--port` | `-p` | `9000` | Bind port |
| `--log-level` | `-l` | `info` | `trace` \| `debug` \| `info` \| `warn` \| `error` \| `off` |
| `--setup-autostart` | `-A` | — | Create XDG autostart entries, then exit; add `-a` to also start the server |
| `--remove-autostart` | `-R` | — | Remove XDG autostart entries, then exit; add `-a` to also start the server |
| `--help` | `-h` | — | Print usage and exit |
| `--version` | `-V` | — | Print version and exit |

The `-A` and `-R` flags can be combined with `-a` to configure autostart and immediately start the server in the same invocation.

The log level can also be changed **at runtime** without restarting — use the dropdown in the browser log panel or send:

```sh
curl -X POST http://<host>:9000/api/config \
     -H 'Content-Type: application/json' \
     -d '{"log_level":"debug"}'
```

---

## Autostart (Raspberry Pi)

Run once to create XDG autostart entries so haven and the VPN connect automatically at login.

> **Run this from the directory that contains `resources/`** — the generated startup script captures the current working directory and the server depends on it to locate resource files.

From an **extracted release directory**:
```sh
./haven -A -a 0.0.0.0 -p 9000
```

From a **source checkout** (workspace root):
```sh
./target/release/haven -A -a 0.0.0.0 -p 9000
```

This creates four files:

| File | Purpose |
|---|---|
| `~/.local/bin/haven-autostart.sh` | Waits up to 60 s for network, then starts the haven server |
| `~/.local/bin/haven-vpn-connect.sh` | Waits up to 90 s for internet, then runs `adguardvpn-cli connect` |
| `~/.config/autostart/haven-server.desktop` | XDG entry — opens a terminal running the server script at login |
| `~/.config/autostart/haven-vpn.desktop` | XDG entry — opens a terminal running the VPN connect script at login |

To remove:

```sh
./haven -R                        # extracted release directory
./target/release/haven -R         # source checkout
```

> Requires a desktop environment with XDG autostart support (e.g. LXDE on Pi OS Desktop).
> A terminal emulator must be installed — detected in order: `x-terminal-emulator`, `lxterminal`, `xfce4-terminal`, `gnome-terminal`, `mate-terminal`, `konsole`, `xterm`.

---

## Building from Source

### Host (debug)

```sh
cargo build
./target/debug/haven -a 0.0.0.0 -p 9000
```

### Cross-compile for Raspberry Pi 5 (aarch64-unknown-linux-gnu)

```sh
# Install cross-linker (Ubuntu / Debian host)
sudo apt-get install gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu libc6-dev-arm64-cross

# Add Rust target
rustup target add aarch64-unknown-linux-gnu

# Build
cargo build --release --target=aarch64-unknown-linux-gnu
```

The linker is pre-configured in `.cargo/config.toml` — no environment variables needed.

Package for deployment:

```sh
mkdir -p haven-deploy
cp target/aarch64-unknown-linux-gnu/release/haven haven-deploy/
cp -r resources haven-deploy/
tar -czf haven-deploy.tar.gz haven-deploy/
```

### Run tests

```sh
cargo test
```

---

## CI/CD

| Workflow | Trigger | Output |
|---|---|---|
| `CI Build` | Pull request | `.tar.gz` artifact (90 days) |
| `Release Build` | Push `v*.*.*` tag | GitHub Release with `.tar.gz` |
| `Dev Build` | Push to `main` | Rolling `dev-latest` pre-release |
| `Manual Build` | `workflow_dispatch` | Artifact + optional pre-release |

### Release procedure

1. Update `version` in `Cargo.toml`.
2. `git commit -m "chore: bump version to 1.2.3"`
3. `git tag v1.2.3 && git push origin v1.2.3`
4. GitHub Actions builds and publishes the release automatically.

---

## Architecture

See [`AGENTS.md`](AGENTS.md) for a full machine-readable architecture reference
including module descriptions, WebSocket protocol, and development notes.

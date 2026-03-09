# AGENTS.md — haven

> Machine-readable architecture guide for AI agents and contributors.  
> Updated after full redesign on 2026-03-09 (Phases 0–8 complete; Session 2 refactoring complete; Session 3 flexible page routing complete; Session 4 project renamed to `haven` + code ordering convention applied).  
> Update this file whenever the architecture changes.

---

## 1. Project Purpose

haven is a **synchronous Rust TCP/WebSocket server** that runs on a
**Raspberry Pi 5** and acts as a remote control panel for the
[AdGuard VPN CLI](https://adguard-vpn.com/en/blog/adguard-vpn-cli.html)
(`adguardvpn-cli`) and `nmcli` (NetworkManager CLI).

A browser-based single-page UI connects to the server over a plain WebSocket and
can:
- Query the current VPN connection state and connected location
- Browse and select from a dynamic location list (sorted by ping)
- Connect to a chosen location or the fastest available
- Disconnect from the VPN
- Reconnect the Raspberry Pi to the currently-active Wi-Fi SSID
- Trigger a full system restart (`shutdown -r now`)
- View a live server log panel with runtime log-level control
- Toggle UI language (English / Russian)

There is **no authentication** and **no TLS** — the server is intended for use
on a trusted local network only.

---

## 2. Repository Layout

```
haven/
├── Cargo.toml                        # Package manifest + dependencies
├── Cargo.lock
├── resources/
│   └── web_resources/
│       ├── html_pages/
│       │   └── index.html            # Single-page UI (served at GET /)
│       └── page_scripts/
│           └── client.js             # Browser WebSocket client
├── src/
│   ├── main.rs                       # Entry point
│   ├── config/                       # CLI argument parsing
│   │   ├── mod.rs
│   │   └── args.rs                   # ServerConfig, parse_args()
│   ├── core/                         # VPN business logic (CLI invocations)
│   │   ├── mod.rs                    # Public API + CoreState/CoreError/CoreResult
│   │   ├── commands.rs               # std::process::Command wrappers
│   │   ├── location.rs               # Location struct
│   │   ├── location_cache.rs         # Arc<RwLock<>> background-refresh cache
│   │   ├── location_parser.rs        # Parse adguardvpn-cli list-locations output
│   │   └── state_parser.rs           # Parse adguardvpn-cli status output
│   ├── logger/                       # Levelled logging + broadcast channel
│   │   ├── mod.rs                    # LogLevel, emit, log_*! macros
│   │   └── broadcast.rs              # MPMC log-line broadcast to WS subscribers
│   ├── server/                       # TCP listener + HTTP/WS dispatch
│   │   ├── mod.rs                    # start(), connection thread loop
│   │   ├── connection_state.rs       # enum ConnectionState { KeepAlive, Close }
│   │   ├── request_handler.rs        # trait RequestHandler
│   │   ├── reader.rs                 # read_stream()
│   │   ├── http_handler.rs           # struct HttpHandler (holds cache)
│   │   ├── pages.rs                  # enum Page — URL→HTML file path mapping
│   │   ├── router.rs                 # route(): GET/POST dispatch, /api/config
│   │   ├── sender.rs                 # send(stream, ResponseBuilder)
│   │   └── ws/
│   │       ├── mod.rs
│   │       ├── handler.rs            # WS upgrade + run loop + log broadcast
│   │       └── messages.rs           # ClientMessage / ServerMessage enums
│   ├── requests/                     # HTTP request parsing
│   ├── responses/                    # HTTP response builders
│   └── utils/                        # Misc utilities (file I/O)
├── .cargo/
│   └── config.toml                   # Cross-linker config per target
├── .github/
│   └── workflows/
│       ├── build_for_pull_request.yaml   # CI: PR test + cross-compile build → artifact
│       ├── build_and_deploy_release.yaml # Release: v* tag → GitHub Release
│       ├── dev_build.yaml               # Dev: push to main → rolling dev-latest
│       └── manual_build.yaml            # Manual: workflow_dispatch, any branch
└── .fabricator/                      # AI-session artifacts (gitignored)
```

---

## 3. Build System

There is **no build script** (`build.rs` was removed). The project compiles directly with `cargo build`.

> **Important:** All file paths are **relative to the working directory** at
> runtime.  The binary must be launched from the workspace root
> (`/home/limd/Documents/Projects/VpnWebServer`) so that `resources/…` resolves
> correctly.
>
> HTML page paths are managed by `server/pages.rs` (`enum Page`) and resolved
> inside `router::route()` — no runtime flag is needed.

### Build command

```sh
cargo build --release
```

### Test command

```sh
cargo test
```

### Run command

```sh
# From workspace root:
./target/release/haven --address 0.0.0.0 --port 9000
# or with all options:
./target/release/haven -a 0.0.0.0 -p 9000 -l debug
```

| Flag | Short | Default | Description |
|---|---|---|---|
| `--address` | `-a` | `127.0.0.1` | Bind address |
| `--port` | `-p` | `9000` | Bind port |
| `--log-level` | `-l` | `info` | Log level: `trace`\|`debug`\|`info`\|`warn`\|`error`\|`off` |
| `--help` | `-h` | — | Print usage and exit |
| `--version` | `-V` | — | Print version and exit |

---

## 4. Dependencies

| Crate | Version | Purpose |
|---|---|---|
| `tungstenite` | 0.21 | WebSocket handshake + frame encoding/decoding |
| `serde` | 1.0 (derive) | Derive `Serialize`/`Deserialize` for WS messages |
| `serde_json` | 1.0 | JSON encode/decode for WS message protocol |
| `time` | 0.3.41 (formatting, local-offset) | Timestamps in logger output |
| `regex` | 1 | Parse `adguardvpn-cli list-locations` tabular output |
| `base64` | 0.22.1 | Encode WebSocket `Sec-WebSocket-Accept` key (legacy path) |
| `sha1` | 0.10.6 | SHA-1 hash for WebSocket handshake (legacy path) |

No async runtime — the server uses OS threads exclusively.

---

## 5. Module Reference

### 5.1 `main.rs`

Startup sequence:

```rust
logger::init_time_offset();
let cfg = config::args::parse_args(std::env::args());
logger::set_level(cfg.log_level);
let cache = core::LocationCache::new();
cache.refresh_in_background();
server::start(cfg, cache);
```

---

### 5.2 `config/args.rs`

```rust
pub struct ServerConfig { pub address: String, pub port: u16, pub log_level: LogLevel }
pub fn parse_args<I: IntoIterator<Item=String>>(args: I) -> ServerConfig
```

- Skips the binary name (first non-`-` token).
- Supports `--key value`, `-k value`, and `--key=value` forms.
- Invalid values are logged to stderr and the default is kept.
- `--help`/`-h` prints usage and exits; `--version`/`-V` prints version and exits.

---

### 5.3 `logger/`

#### `mod.rs`

| Symbol | Description |
|---|---|
| `enum LogLevel` | `Trace=0, Debug=1, Info=2, Warn=3, Error=4, Off=5` — `#[repr(u8)]`, `PartialOrd` |
| `LogLevel::from_str(s)` | Case-insensitive parse; returns `Option<LogLevel>` |
| `GLOBAL_LOG_LEVEL` | `AtomicU8` — runtime-mutable global filter |
| `set_level(level)` | Stores to `GLOBAL_LOG_LEVEL`, emits an `Info` log line |
| `current_level()` | Loads from `GLOBAL_LOG_LEVEL` |
| `init_time_offset()` | Caches local UTC offset in `OnceLock` (call once at startup) |
| `trace / debug / info / warn / error` | Free functions → `emit()` |
| `log_trace! … log_error!` | Macro wrappers for the free functions |
| Level icons | Trace 🟫  Debug 🟩  Info 🟦  Warn 🟧  Error 🟥 |

`emit()` filters by `current_level()`, prints to stdout, then calls
`broadcast::broadcast(&line)` so WebSocket subscribers receive every log line.

#### `broadcast.rs`

```rust
pub struct LogLine { pub timestamp, level, tag: String, pub pid: u32, pub tid: u64, pub message: String }
pub fn subscribe() -> Receiver<LogLine>    // adds Sender to SUBSCRIBERS (capacity 1024)
pub fn broadcast(line: &LogLine)          // fans out to all subscribers
```

`SUBSCRIBERS` is a `Mutex<Vec<Sender<LogLine>>>`.  Dead senders are pruned
on every broadcast.

---

### 5.4 `core/`

All public functions are **synchronous and blocking**.

#### `mod.rs` — public API

| Function | Command(s) invoked |
|---|---|
| `connect(location: Option<String>)` | `adguardvpn-cli connect -l <city>` (or fastest if `None`) |
| `disconnect()` | `adguardvpn-cli disconnect` |
| `restart()` | `shutdown -r now` |
| `status() → CoreResult<CoreState>` | `adguardvpn-cli status` |
| `list_locations() → CoreResult<Vec<Location>>` | `adguardvpn-cli list-locations 100` |
| `reconnect_wifi() → CoreResult<()>` | `nmcli` SSID detection + `nmcli connection up id <ssid>` |

`reconnect_wifi()` polls `get_ssid()` for up to 10 s (500 ms interval) to
confirm the SSID is back after issuing the reconnect command.

```rust
pub enum CoreState { Connected, Disconnected, Reconnecting }
pub enum CoreError { CommandFailed { cmd, stderr }, ParseError { context, raw }, IoError(io::Error) }
pub type CoreResult<T> = Result<T, CoreError>;
```

#### `location.rs`

```rust
pub struct Location { pub iso: String, pub city: String, pub country: String, pub ping_ms: i32 }
```

`iso` serialises as `"id"` in JSON (`#[serde(rename = "id")]`).

No methods — fields are accessed directly.

#### `location_parser.rs`

`parse_locations(output: &str) → Vec<Location>` — splits each line on `\s{2,}`
(regex), expects 4 columns (`ISO, COUNTRY, CITY, PING`), skips header and
malformed lines, parses ping as `i32` (fallback `-1`).

#### `location_cache.rs`

`LocationCache` wraps `Arc<RwLock<Vec<Location>>>` + `Arc<RwLock<Option<Instant>>>`.

| Method | Description |
|---|---|
| `new()` | Returns empty cache |
| `refresh_in_background()` | Spawns a thread: calls `core::list_locations()`, writes result, loops |
| `get()` | Returns a snapshot clone (non-blocking read) |

`LocationCache` is `Clone` — the `Arc` is cloned, not the data.

#### `state_parser.rs`

`parse_status(output: &str) → CoreState` — lowercases, checks substrings in
priority order: `"reconnecting"` → `Reconnecting`; `"disconnected"` →
`Disconnected`; `"connected"` → `Connected`; else → `Disconnected`.

---

### 5.5 `server/`

#### Connection lifecycle

```
TcpListener::incoming()
  └─ thread::spawn
       └─ listen_stream(&mut TcpStream, LocationCache)
            HttpHandler { cache }
            loop:
              read_stream(1024, HttpHandler)
              → parse HTTP bytes → HttpRequest
              → router::route(stream, req, cache)
                    GET /            → serve Page::Index.path()
                    GET /resources/* → serve static file
                    GET /ws          → ws::handler::handle()   ← WebSocket loop
                    POST /api/config → set logger level
                    *                → 404
              → ConnectionState::Close → break
```

#### `request_handler.rs`

```rust
pub trait RequestHandler {
    fn handle(&self, stream: &mut TcpStream, size: usize, data: &[u8]) -> ConnectionState;
}
```

#### `reader.rs`

`read_stream<T: RequestHandler>(buffer_size, stream, handler) → ConnectionState`

Reads up to `buffer_size` bytes, calls `handler.handle()`, or returns `Close` on EOF/error.

#### `http_handler.rs`

`HttpHandler { cache: LocationCache }` — implements `RequestHandler`.
Parses raw bytes into `HttpRequest`, delegates to `router::route()`.

#### `pages.rs`

```rust
pub enum Page { Index }
impl Page { pub fn path(&self) -> &'static str }
```

Single source of truth for URL→HTML file path mapping.
Add a variant + one match arm here to serve a new HTML page — no other files need to change.

#### `router.rs`

`route(stream, req, cache) → ConnectionState`

- `GET /` → `Page::Index.path()` (resolved via `pages::Page`)
- `GET /resources/*` → serve file at path (strips leading `/`), content-type from extension
- `GET /ws` → `ws::handler::handle()`
- `POST /api/config` → JSON body `{ "log_level": "debug" }` → `logger::set_level()`
- All others → `404 Not Found`

#### `ws/handler.rs`

`handle(stream, req, cache) → ConnectionState`

1. Performs tungstenite WS handshake.
2. Sends initial `StatusUpdate`, `LocationList`, `LogLevelChanged` to the new client.
3. Calls `logger::broadcast::subscribe()` and enters the run loop.
4. Run loop: select over tungstenite messages and log-broadcast channel;
   dispatches `ClientMessage` variants to `core::*`; sends `ServerMessage` back.

#### `ws/messages.rs`

See §6 for the full protocol.  Both enums use `#[serde(tag = "type")]`.

---

### 5.6 `requests/`

| File | Role |
|---|---|
| `http_method.rs` | `enum HttpMethod { GET, POST, DELETE, … }` + `FromStr` impl |
| `http_request.rs` | `HttpRequest::new(&str) → Option<HttpRequest>` — parses request line, headers, body |

---

### 5.7 `responses/`

| File | Role |
|---|---|
| `response_builder.rs` | `trait ResponseBuilder { fn build(self) → String }` |
| `http_response_builder.rs` | `HTTP/1.1 <code> OK\r\nContent-Type: …\r\nContent-Length: …\r\n\r\n<body>` |

---

### 5.8 `utils/`

| File | Role |
|---|---|
| `resource_provider.rs` | `read_content(path: &str) → Option<String>` — thin wrapper around `fs::read_to_string` |

---

## 6. WebSocket Protocol

All messages are JSON objects with a `"type"` discriminant (`#[serde(tag = "type")]`).

### Client → Server (`ClientMessage`)

| type | Extra fields | Description |
|---|---|---|
| `Status` | — | Request current VPN state |
| `Connect` | `location?: string` | Connect; omit or null for fastest |
| `Disconnect` | — | Disconnect from VPN |
| `ReconnectWifi` | — | Reconnect to current Wi-Fi SSID |
| `Restart` | — | `shutdown -r now` |
| `RefreshLocations` | — | Re-run `list-locations` and broadcast result |
| `SetLogLevel` | `level: string` | Change runtime log level |

### Server → Client (`ServerMessage`)

| type | Fields | Description |
|---|---|---|
| `StatusUpdate` | `state: CoreState, location: string\|null` | VPN state change or response to `Status` |
| `LocationList` | `locations: Location[]` | Full location list (sorted by ping) |
| `LogLine` | `timestamp, level, tag: string; pid: u32; tid: u64; message: string` | Live log line |
| `LogLevelChanged` | `level: string` | Confirms a level change |
| `Error` | `code, message: string` | Operation error |

### WS framing

Handled entirely by **tungstenite 0.21** — no manual frame parsing.

---

## 7. Frontend (Web UI)

**Files:** `resources/web_resources/html_pages/index.html`,
`resources/web_resources/page_scripts/client.js`

Served as static files over HTTP. No build step — plain HTML + vanilla JavaScript.

### UI elements

| Element | ID | Purpose |
|---|---|---|
| Language toggle | `langToggle` | EN ↔ RU; persisted in `sessionStorage` |
| Refresh button | `refreshBtn` | Sends `RefreshLocations` |
| Error banner | `errorBanner` | Shown on WS disconnect, hidden on reconnect |
| Status circle | `statusIndicator` | CSS class controls colour + pulse animation |
| Status text | `statusText` | Human-readable state + connected location |
| Location search | `locationSearch` | Filters `locationBody` rows live |
| Location table | `locationBody` | `<tbody>` rows: ISO / city+country / ping |
| Toggle button | `toggleBtn` | Connect (optional city) / Disconnect / `…` disabled |
| Wi-Fi button | `wifiBtn` | Sends `ReconnectWifi` |
| Restart button | `restartBtn` | Sends `Restart` |
| Log panel | `logPanel` | `<details>` — collapsible |
| Log level select | `logLevelSelect` | Sends `SetLogLevel` on change |
| Log output | `logOutput` | Monospace; max 500 lines; auto-scroll |

### Status colours

| State | CSS class | Colour | Animation |
|---|---|---|---|
| Connected | `status-connected` | Green | — |
| Disconnected | `status-disconnected` | Red | — |
| Connecting | `status-connecting` | Orange | `pulse` 1 s |
| Disconnecting | `status-disconnecting` | Orange | `pulse` 1 s |
| Reconnecting | `status-reconnecting` | Yellow | `pulse` 1 s |
| WS disconnected | `status-unknown` | Grey dashed | — |

### WS reconnect

Exponential backoff: `[1, 2, 4, 8, 16]` seconds, then stays at 16 s.  
On `close` event: show error banner, reset to `status-unknown`.
On `open` event: hide banner, reset attempt counter.

### i18n

`sessionStorage` key `"lang"` persists the choice; falls back to
`navigator.language` (Russian if starts with `"ru"`, else English).
All translatable strings are in the `i18n` object in `client.js`.

---

## 8. System Commands Reference

All commands are run as the OS user that launched the haven binary.
That user must have the necessary permissions for `adguardvpn-cli`, `nmcli`,
and `shutdown`.

| Command | Trigger |
|---|---|
| `adguardvpn-cli connect -l <city>` | `ClientMessage::Connect { location: Some(city) }` |
| `adguardvpn-cli connect` (fastest) | `ClientMessage::Connect { location: None }` |
| `adguardvpn-cli disconnect` | `ClientMessage::Disconnect` |
| `adguardvpn-cli status` | `ClientMessage::Status` + after `Connect`/`Disconnect` |
| `adguardvpn-cli list-locations 100` | `ClientMessage::RefreshLocations` + background cache refresh |
| `shutdown -r now` | `ClientMessage::Restart` |
| `sh -c "nmcli -t -f active,ssid dev wifi \| grep '^yes:' \| cut -d':' -f2-"` | `ClientMessage::ReconnectWifi` (step 1: get SSID) |
| `nmcli connection up id <ssid>` | `ClientMessage::ReconnectWifi` (step 2: reconnect) |

---

## 9. Known Issues & Technical Debt

| # | Location | Issue |
|---|---|---|
| 1 | `server/connection_state.rs` | `KeepAlive` variant is defined and matched but never returned by any current code path (HTTP responses always close); future keep-alive support would use it |
| 2 | `server/ws/handler.rs` | `send_status()` always sends `location: None` — connected location is not retrieved from `adguardvpn-cli status` output |
| 3 | General | Blocking `std::process::Command` calls run on the connection thread — long VPN commands stall WS message processing for that connection |

---

## 10. Development Notes for AI Agents

- **Adding a new WebSocket command:** (1) Add variant to `ClientMessage` in `ws/messages.rs`;
  (2) handle it in `ws/handler.rs`; (3) add the `core::*` call;
  (4) add outgoing sender in `client.js`.
- **Adding a new server-push message:** (1) Add variant to `ServerMessage`;
  (2) serialize and send from `ws/handler.rs`; (3) handle in `client.js::handleMessage()`.
- **Changing the runtime log level via HTTP:** `POST /api/config` with body
  `{"log_level":"debug"}` — no restart needed.
- **Adding a new cross-compilation target:** Add one entry to the matrix in each
  workflow YAML; add the corresponding linker to `.cargo/config.toml`.
- **Running tests:** `cargo test` — 69 unit tests across 9 modules.
- **Adding a new HTML page at a new GET route:** (1) add a variant to `enum Page` in `server/pages.rs` with its `path()` match arm; (2) add one match arm in `router::route()` in `server/router.rs` — no other files need to change.
- **Cross-compiling for Raspberry Pi:**
  ```sh
  sudo apt-get install gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu libc6-dev-arm64-cross
  cargo build --release --target=aarch64-unknown-linux-gnu
  ```

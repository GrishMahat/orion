# System Architecture

## Core Components

### Background Service (background/)
- `hotkey.rs` → Global hotkey detection and management
- `process.rs` → Process lifecycle and IPC handling
- `main.rs` → Service initialization and event loop

### Popup Interface (popup_ui/)
- `ui.rs` → Search interface and result display
- `commands.rs` → Command parsing and execution
- `state.rs` → UI state management
- `main.rs` → Window management and IPC client

### Settings Application (settings_app/)
- `ui.rs` → Configuration interface
- `state.rs` → Settings state management
- `profiles.rs` → Profile management
- `main.rs` → Settings window

### Shared Library (shared/)
- `config.rs` → Configuration management
- `models.rs` → Common data structures
- `ipc.rs` → Inter-process communication
- `logging.rs` →  orion log
## Communication Architecture

### IPC Implementation
- Background ↔ Popup: Unix domain sockets for low-latency communication
- Background ↔ Settings: TCP localhost for configuration updates
- Shared message types and protocols defined in `shared/ipc.rs`

### Configuration Management
- TOML-based configuration files
- Automatic configuration reloading
- Profile-based settings support
- Configuration validation and migration

## System Workflow

1. System Initialization
   - Background service starts at system boot
   - Loads configuration and initializes IPC
   - Registers global hotkey

2. Hotkey Detection
   - Monitors system-wide keyboard events
   - Matches against configured hotkey combinations
   - Triggers popup interface

3. Popup Interface
   - Launches or focuses existing window
   - Handles search input and command execution
   - Manages command history and suggestions

4. Settings Management
   - Provides configuration interface
   - Supports multiple profiles
   - Live configuration updates
   - Configuration backup and restore

## Error Handling
- Graceful degradation on IPC failures
- Automatic service recovery
- User-friendly error reporting
- Configuration validation and repair
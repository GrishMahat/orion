# 🔍 Orion - Technical Overview & Tasks

## 📁 Project Structure

```
orion/
├── background/        # Background service application
│   ├── src/
│   │   ├── hotkey.rs  # Hotkey detection and handling
│   │   ├── process.rs # Process lifecycle management
│   │   ├── setup.rs   # Configuration setup
│   │   └── main.rs    # Application entry point
├── popup_ui/          # Search and command UI interface
├── settings_app/      # GUI settings application
├── shared/            # Shared code library
│   ├── src/
│   │   ├── config.rs  # Configuration structures and functions
│   │   ├── ipc.rs     # Inter-process communication
│   │   ├── models.rs  # Data models
│   │   └── logging.rs # Logging functionality
└── workflow/          # Project documentation
```

## 🧩 Technical Overview

Orion is built as a Rust workspace with multiple crates:

### 📚 shared

A library crate containing common code used by all applications:

| Module | Purpose |
|--------|---------|
| **config.rs** | Configuration structures and validation |
| **ipc.rs** | Inter-Process Communication between components |
| **models.rs** | Common data models |
| **logging.rs** | Logging functionality |

### 🔄 background

The main service that runs in the background:

| Module | Purpose |
|--------|---------|
| **hotkey.rs** | Monitors for hotkey combinations |
| **process.rs** | Manages process lifecycle and IPC |
| **setup.rs** | Handles configuration initialization |
| **main.rs** | Service initialization and event loop |

### 🔍 popup_ui

The search and command interface:

| Module | Purpose |
|--------|---------|
| **ui.rs** | Displays search interface with real-time results |
| **commands.rs** | Handles command parsing and execution |
| **state.rs** | Manages UI state and window behavior |
| **main.rs** | Window management and IPC client |

### ⚙️ settings_app

A GUI application for configuring Orion:

| Module | Purpose |
|--------|---------|
| **ui.rs** | Configuration interface with Iced GUI framework |
| **state.rs** | Settings state management |
| **profiles.rs** | Profile management |
| **main.rs** | Settings window and application entry point |

## 🔧 Implementation Details

### Configuration (`shared/src/config.rs`)
- ✅ Uses TOML format for configuration files
- ✅ Includes validation for settings values
- ✅ Supports multiple profiles with commands
- ✅ Handles command prefixes for direct command access

### Command Prefixes (Bang Commands)
- Similar to DuckDuckGo's bang syntax (e.g., `!g` for Google search)
- Defined in the configuration as CommandPrefix objects
- Examples:
  - `!g` - Search Google
  - `!yt` - Search YouTube
  - `!w` - Search Wikipedia
  - `!gh` - Search GitHub
- Features:
  - Instant redirection to specific search engines or sites
  - Custom URL templates for each prefix
  - Support for URL encoding of search terms
  - User-configurable through settings_app
  - Profile-specific prefix configurations

### IPC (`shared/src/ipc.rs`)
- ✅ Uses Unix sockets for Linux/macOS for low-latency communication
- ✅ TCP localhost sockets for configuration updates
- ✅ Provides both synchronous and asynchronous interfaces
- ✅ Includes timeout handling and message size limits
- ✅ Handles serialization/deserialization of messages

### Popup UI (`popup_ui/`)
```
┌───────────────────────┐
│ ┌────────────────────┐│
│ │ Search Bar          ││
│ └────────────────────┘│
│                       │
│ ┌────────────────────┐│
│ │ Result Item 1      ││
│ └────────────────────┘│
│ ┌────────────────────┐│
│ │ Result Item 2      ││
│ └────────────────────┘│
│ ┌────────────────────┐│
│ │ Result Item 3      ││
│ └────────────────────┘│
└───────────────────────┘
```

- **ui.rs**: Implements the search interface and result display
  - Minimal, keyboard-focused UI design
  - Real-time search result updates
  - Command suggestion and autocompletion
  - Keyboard navigation through results

- **commands.rs**: Handles command parsing and execution
  - Parses user input to identify commands
  - Supports command prefixes for specialized searches
  - Handles command execution with proper error handling
  - Manages command history for quick access

- **state.rs**: Manages UI state
  - Tracks current search query and results
  - Manages selection state and navigation
  - Handles UI transitions and animations
  - Maintains session history

- **main.rs**: Window management and IPC client
  - Creates and manages the popup window
  - Handles window focus and positioning
  - Establishes IPC connection with background service
  - Processes system events and keyboard input

### Settings UI (`settings_app/`)
```
┌───────────────────────────────────────────────┐
│ ┌────────┐                                    │
│ │ General│                                    │
│ ├────────┤ ┌────────────────────────────────┐ │
│ │ Hotkeys│ │                                │ │
│ ├────────┤ │                                │ │
│ │ Appear.│ │         Settings Panel         │ │
│ ├────────┤ │                                │ │
│ │ Adv.   │ │                                │ │
│ └────────┘ └────────────────────────────────┘ │
└───────────────────────────────────────────────┘
```
- Built with Iced, a Rust GUI framework
- Provides a clean interface for editing configurations
- Updates configuration file on save
- Shows profiles, commands, and allows editing settings

## 🔄 System Workflow

### 1. System Initialization
```
┌───────────┐     ┌─────────────┐     ┌───────────┐
│ Start     │────►│ Load        │────►│ Register  │
│ Background│     │ Config      │     │ Hotkey    │
└───────────┘     └─────────────┘     └───────────┘
```

- Background service starts at system boot
- Loads configuration and initializes IPC
- Registers global hotkey

### 2. Hotkey Detection
```
┌───────────┐     ┌─────────────┐     ┌───────────┐
│ Detect    │────►│ Match       │────►│ Trigger   │
│ Key Event │     │ Hotkey      │     │ Popup     │
└───────────┘     └─────────────┘     └───────────┘
```

- Background service monitors system-wide keyboard events
- Matches against configured hotkey combinations
- Triggers popup interface via IPC

### 3. Popup Interface
```
┌───────────┐     ┌─────────────┐     ┌───────────┐
│ Show      │────►│ Process     │────►│ Execute   │
│ Popup     │     │ Input       │     │ Command   │
└───────────┘     └─────────────┘     └───────────┘
```

- Launches or focuses existing window
- Handles search input and displays results in real-time
- Executes selected commands
- Manages command history and suggestions

### 4. Bang Command Workflow
```
┌───────────┐     ┌─────────────┐     ┌───────────┐
│ Input:    │────►│ Recognize   │────►│ Redirect  │
│ !g rust   │     │ Bang Prefix │     │ to Google │
└───────────┘     └─────────────┘     └───────────┘
```

- User activates Orion with hotkey
- Types a bang command (e.g., "!g rust programming")
- Orion recognizes the prefix and processes it
- Redirects to the appropriate service with the search query
- Example: "!g rust programming" opens Google search for "rust programming"

### 5. Settings Management
```
┌───────────┐     ┌─────────────┐     ┌───────────┐
│ Open      │────►│ Edit        │────►│ Save      │
│ Settings  │     │ Config      │     │ Changes   │
└───────────┘     └─────────────┘     └───────────┘
```

- Provides user interface for configuration changes
- Communicates changes to background service
- Supports profile management and customization

## ✅ Current Tasks

### Completed
- ✅ Basic configuration structure
- ✅ IPC communication between components
- ✅ Settings app UI framework
- ✅ Command prefix support
- ✅ Profile management
- ✅ Configuration validation
- ✅ Basic command prefix (bang) implementation
- ✅ Default Config implementation
- ✅ Project workspace structure

### In Progress
- 🔄 Fix validation and UI issues in settings app
  - ✅ Remove validator derive macros causing type mismatches
  - ✅ Implement manual validation functions
  - ✅ Fix unused imports in config.rs and ipc.rs
  - ⏳ Add proper error handling for settings UI
  - ⏳ Fix text input binding for profile addition
  - ⏳ Fix PickList component for profile selection
- 🔄 Implement profile switching
  - ✅ Basic profile data model
  - ⏳ UI for profile selection and switching
  - ⏳ Real-time profile updates
  - ⏳ Profile-specific settings persistence
- 🔄 Connect background service to settings app via IPC
  - ✅ Basic IPC message structure
  - ✅ Unix socket communication
  - ⏳ Configuration update notifications
  - ⏳ Live settings updates

### Short-term Goals
- ⏳ Complete settings app functionality
  - ⏳ Fix all remaining compilation errors
  - ⏳ Implement profile management UI
  - ⏳ Add command prefix configuration interface
  - ⏳ Add hotkey configuration UI
- ⏳ Create basic popup UI
  - ⏳ Setup window with search input
  - ⏳ Implement focus and blur handling
  - ⏳ Add basic styling and theming
- ⏳ Implement bang command execution
  - ⏳ Add URL parameter substitution
  - ⏳ Implement URL encoding/decoding
  - ⏳ Add browser launch functionality
  - ⏳ Support command history

### Medium-term Goals
- ⏳ Implement hotkey detection
  - ⏳ Cross-platform hotkey registration
  - ⏳ User-configurable key combinations
  - ⏳ Modifier key support (Alt, Ctrl, Shift)
- ⏳ Create popup UI search interface
  - ⏳ Real-time search results
  - ⏳ Keyboard navigation
  - ⏳ Result highlighting
- ⏳ Command execution handling
  - ⏳ Process spawning for external commands
  - ⏳ Result capture and display
  - ⏳ Error handling

### Long-term Goals
- ⏳ Plugin system for extensions
- ⏳ Custom command scripting
- ⏳ Theme support
- ⏳ Cloud sync for settings
- ⏳ AI-powered search suggestions 
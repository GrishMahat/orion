# Orion - Technical Overview & Tasks

## Project Structure

```
orion/
├── background/        # Background service application
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

## Technical Overview

Orion is built as a Rust workspace with multiple crates:

1. **shared**: A library crate containing common code used by both applications:
   - Configuration structures and validation
   - IPC (Inter-Process Communication) between the components
   - Common data models
   - Logging functionality

2. **background**: The main service that runs in the background:
   - Monitors for hotkey combinations
   - Manages process lifecycle and IPC
   - Handles service initialization and event loop
   - Triggers popup UI when hotkey is detected

3. **popup_ui**: The search and command interface:
   - Displays when triggered by the background service
   - Provides search functionality with real-time results
   - Handles command parsing and execution
   - Manages UI state and window behavior
   - Communicates with background service via IPC

4. **settings_app**: A GUI application for configuring Orion:
   - Built with the Iced GUI framework
   - Allows editing profiles, hotkeys, and other settings
   - Communicates with the background service via IPC

## Implementation Details

### Configuration (shared/src/config.rs)
- Uses TOML format for configuration files
- Includes validation for settings values
- Supports multiple profiles with commands
- Handles command prefixes for direct command access

### Command Prefixes (Bang Commands)
- Similar to DuckDuckGo's bang syntax (e.g., !g for Google search)
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

### IPC (shared/src/ipc.rs)
- Uses Unix sockets for Linux/macOS for low-latency communication
- TCP localhost sockets for configuration updates
- Provides both synchronous and asynchronous interfaces
- Includes timeout handling and message size limits
- Handles serialization/deserialization of messages

### Popup UI (popup_ui/)
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

### Settings UI (settings_app/)
- Built with Iced, a Rust GUI framework
- Provides a clean interface for editing configurations
- Updates configuration file on save
- Shows profiles, commands, and allows editing settings

## System Workflow

1. **System Initialization**
   - Background service starts at system boot
   - Loads configuration and initializes IPC
   - Registers global hotkey

2. **Hotkey Detection**
   - Background service monitors system-wide keyboard events
   - Matches against configured hotkey combinations
   - Triggers popup interface via IPC

3. **Popup Interface**
   - Launches or focuses existing window
   - Handles search input and displays results in real-time
   - Executes selected commands
   - Manages command history and suggestions

4. **Bang Command Workflow**
   - User activates Orion with hotkey
   - Types a bang command (e.g., "!g rust programming")
   - Orion recognizes the prefix and processes it
   - Redirects to the appropriate service with the search query
   - Example: "!g rust programming" opens Google search for "rust programming"

5. **Settings Management**
   - Provides user interface for configuration changes
   - Communicates changes to background service
   - Supports profile management and customization

## Current Tasks

### Completed
- [x] Basic configuration structure
- [x] IPC communication between components
- [x] Settings app UI framework
- [x] Command prefix support
- [x] Profile management
- [x] Configuration validation
- [x] Basic command prefix (bang) implementation
- [x] Default Config implementation
- [x] Project workspace structure

### In Progress
- [ ] Fix validation and UI issues in settings app
  - [x] Remove validator derive macros causing type mismatches
  - [x] Implement manual validation functions
  - [x] Fix unused imports in config.rs and ipc.rs
  - [ ] Add proper error handling for settings UI
  - [ ] Fix text input binding for profile addition
  - [ ] Fix PickList component for profile selection
- [ ] Implement profile switching
  - [x] Basic profile data model
  - [ ] UI for profile selection and switching
  - [ ] Real-time profile updates
  - [ ] Profile-specific settings persistence
- [ ] Connect background service to settings app via IPC
  - [x] Basic IPC message structure
  - [x] Unix socket communication
  - [ ] Configuration update notifications
  - [ ] Live settings updates

### Short-term Goals
- [ ] Complete settings app functionality
  - [ ] Fix all remaining compilation errors
  - [ ] Implement profile management UI
  - [ ] Add command prefix configuration interface
  - [ ] Add hotkey configuration UI
- [ ] Create basic popup UI
  - [ ] Setup window with search input
  - [ ] Implement focus and blur handling
  - [ ] Add basic styling and theming
- [ ] Implement bang command execution
  - [ ] Add URL parameter substitution
  - [ ] Implement URL encoding/decoding
  - [ ] Add browser launch functionality
  - [ ] Support command history

### Medium-term Goals
- [ ] Implement hotkey detection
  - [ ] Cross-platform hotkey registration
  - [ ] User-configurable key combinations
  - [ ] Modifier key support (Alt, Ctrl, Shift)
- [ ] Create popup UI search interface
  - [ ] Real-time search results
  - [ ] Keyboard navigation
  - [ ] Result highlighting
- [ ] Command execution handling
  - [ ] Process spawning for external commands
  - [ ] Result capture and display
  - [ ] Error handling

### Long-term Goals
- [ ] Add keyboard shortcuts in settings
- [ ] Implement search algorithm with ranking
- [ ] Add theming support
- [ ] Implement window positioning and focus management for popup_ui
- [ ] Add command history and suggestions
- [ ] Expand bang command prefixes with more services
- [ ] Add custom user-defined bang commands
- [ ] Create plugin system for extensibility
- [ ] Add internationalization support
- [ ] Implement auto-update functionality

## Development Guidelines

### Code Style
- Follow Rust idioms and naming conventions
- Use meaningful error messages with context
- Validate all user inputs
- Use Result for error handling with anyhow for context

### Testing
- Unit tests for validation logic
- Integration tests for IPC
- End-to-end tests for configuration loading/saving
- UI component tests for popup and settings interfaces

### Performance Considerations
- Minimize latency for hotkey activation
- Optimize search for sub-100ms response times
- Keep memory usage low for background service
- Efficiently manage window creation/destruction for popup UI 
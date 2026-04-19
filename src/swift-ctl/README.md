# swift-ctl

The command-line controller for the Swift Desktop Environment. It allows users to easily view, modify, and monitor system configuration using terminal.

## Usage

The tool follows a simple `<command> <section> <key>` structure.

**Get a setting**
Retrieve a value from a specific section:
```bash
swift-ctl get appearance theme
```
**Set a setting**

Update or create a new setting (changes are automatically saved to ~/.config/swift/settings.toml):
```Bash
swift-ctl set appearance theme light
```
**Monitor mode**
Listen for real-time configuration changes emitted by the daemon:
```Bash
swift-ctl monitor
```
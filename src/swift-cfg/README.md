# swift-cfg

The configuration daemon for the Swift Desktop Environment. It manages settings across different sections and provides a DBus interface for real-time updates.

## Configuration Files

The daemon performs a "deep merge" of the following files:
1. **/usr/share/swift/defaults.toml** - System-wide default settings.
2. **~/.config/swift/settings.toml** - User-specific overrides.

### File Format Example

```toml
[appearance]
theme = "dark"
accent-color = "blue"

[panel]
enable = "1"
```

### DBus Interface

- Service: `org.swift.Config`

- Object Path: `/org/swift/Config`

- Interface: `org.swift.Config`

**Get value**
```bash
busctl --user call org.swift.Config /org/swift/Config org.swift.Config GetValue ss "section" "key"
```

**Set value**
```bash
busctl --user call org.swift.Config /org/swift/Config org.swift.Config SetValue sss "section" "key" "value"
```

**Monitor changes**
```bash
busctl --user monitor org.swift.Config
```
### Systemd
```bash
systemctl --user status swift-cfg
```
### Dedicated tools
You can also manually manage configuration through a dedicated tools. See `swift-ctl` for CLI or Swift Settings for GUI.
# appInstalleR

A simple and efficient command-line tool to create desktop entries for AppImage applications on Linux systems.

## Description

appInstalleR automatically generates and installs `.desktop` files for AppImage applications, making them accessible from your system's application menu. It places the desktop entries in the user's local applications directory (`~/.local/share/applications/`).

## Features

- Creates desktop entries for AppImage files
- Automatically sets correct file permissions
- Places desktop files in the correct user directory
- Simple command-line interface
- Minimal dependencies

## Dependencies

- Rust 2021 edition or later
- Dependencies (automatically managed by Cargo):
  - clap (^4.5.23)
  - dirs (^5.0)

## Installation

0. Using crates.io:

```bash
cargo install appInstalleR
```

1. Clone the repository:

```bash
git clone https://github.com/sponkurtus2/appInstalleR.git
```

2. Build the project:
```bash
cargo build --release
```

3. Run the project:
```bash
cargo run --release
```

## Usage

```bash
appInstalleR --app-file <path_to_app_image>
```

## Support me 🤝

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/sponkurtus2)

## How It Works

1. The tool validates that the specified file exists and has the `.AppImage` extension
2. Creates a desktop entry file with the appropriate metadata
3. Places the file in `~/.local/share/applications/`
4. Sets the correct file permissions (executable)

## Building from Source

The project uses several optimization flags for release builds:
- Link Time Optimization (LTO) enabled
- Binary size optimization
- Symbol stripping
- Panic abort optimization

## Notes

- This tool is designed for Linux systems
- Requires appropriate permissions to create files in the user's application directory
- Desktop entries are created for the current user only (not system-wide)
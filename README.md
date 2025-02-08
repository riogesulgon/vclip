# vclip

A terminal-based video clip extraction tool written in Rust. This application provides a simple TUI (Terminal User Interface) for extracting clips from video files using FFmpeg.

## Features

- Interactive terminal user interface
- Easy time-based video clip extraction
- Real-time progress feedback
- Copy stream (fast extraction without re-encoding)
- Simple keyboard navigation

## Prerequisites

Before building and running vclip, you need to install some system dependencies. See [DEPENDENCIES.md](DEPENDENCIES.md) for detailed installation instructions.

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/vclip.git
cd vclip
```

2. Build the project:
```bash
cargo build --release
```

The compiled binary will be available at `target/release/vclip`

## Usage

1. Run the application:
```bash
./target/release/vclip
```

2. Navigate through the interface:
- Use `Tab` to switch between input fields
- Type the required information:
  - Input file path
  - Start time (HH:MM:SS format)
  - End time (HH:MM:SS format)
  - Output file path
- Press `Enter` when all fields are filled to start the extraction
- Press `q` to quit the application

## Dependencies

- [ratatui](https://crates.io/crates/ratatui) - TUI framework
- [crossterm](https://crates.io/crates/crossterm) - Terminal manipulation
- [ffmpeg-next](https://crates.io/crates/ffmpeg-next) - FFmpeg bindings for Rust
- [anyhow](https://crates.io/crates/anyhow) - Error handling

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. 
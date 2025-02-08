# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Auto-scrolling functionality for FFmpeg output
- Scrollbar widget for output window
- Visual scroll position indicator

### Changed
- Improved FFmpeg output display with real-time updates
- Enhanced UI responsiveness during processing
- Updated message area to show more content

### Fixed
- Fixed memory usage by removing unused variables
- Improved error handling in FFmpeg process
- Fixed UI refresh rate during processing

## [0.1.0] - 2024-03-20

### Added
- Initial release
- Basic TUI interface with input fields
- FFmpeg integration for video clip extraction
- Real-time process output
- Keyboard navigation (Tab, Enter, Ctrl+Q)
- Error handling and user feedback
- Support for time-based clip extraction

### Dependencies Added
- ratatui v0.24.0 - TUI framework
- crossterm v0.27.0 - Terminal manipulation
- ffmpeg-next v6.1.0 - FFmpeg bindings for Rust
- anyhow v1.0.75 - Error handling

### System Dependencies
Please see DEPENDENCIES.md for system-level dependencies that need to be installed. 
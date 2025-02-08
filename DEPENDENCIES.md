# System Dependencies

To build and run this application, you need to install the following system dependencies:

## FFmpeg Development Libraries
```bash
sudo apt-get install -y libavcodec-dev libavformat-dev libavutil-dev libavfilter-dev libavdevice-dev libpostproc-dev libswresample-dev libswscale-dev
```

## Build Tools and Development Dependencies
```bash
sudo apt-get install -y build-essential clang pkg-config
```

## Runtime Dependencies
- FFmpeg command-line tools:
```bash
sudo apt-get install -y ffmpeg
```

## Verification
You can verify the installation of FFmpeg and its development libraries with:
```bash
ffmpeg -version
pkg-config --libs --cflags libavutil libavformat libavcodec
```

## Note
These installation commands are for Ubuntu/Debian-based systems. For other distributions, use the appropriate package manager and package names. 
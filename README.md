# 🎬 YouTube Downloader (Rust) 🦀

A simple yet efficient YouTube video downloader written in Rust.

![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue)

## ✨ Features

- 📥 Download YouTube videos in various formats and quality options
- 🎵 Extract audio-only streams
- 📝 Custom output filename and directory
- 📊 Progress bar for download tracking
- ⚡ Lightweight and fast

## 📋 Prerequisites

- 🦀 Rust (latest stable version recommended)
- � FFmpeg (required for format conversions and audio extraction)

## 🛠️ Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/youtube-downloader-rust.git
   cd youtube-downloader-rust/src/
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## 🚀 Usage

Run the program with a YouTube URL:
```bash
cargo run --release -- <YOUTUBE_URL>
```

### 🎛️ Options:
- `-o, --output <PATH>`: Set custom output path/filename
- `-f, --format <FORMAT>`: Specify format (mp4, webm, mp3, etc.)
- `-q, --quality <QUALITY>`: Set video quality (highest, lowest, 720p, etc.)
- `--audio-only`: Download audio only

### 💡 Examples:
```bash
# Download highest quality video
cargo run --release -- https://www.youtube.com/watch?v=dQw4w9WgXcQ

# Download as MP3
cargo run --release -- https://www.youtube.com/watch?v=dQw4w9WgXcQ -f mp3

# Custom output filename
cargo run --release -- https://www.youtube.com/watch?v=dQw4w9WgXcQ -o "my_video.mp4"
```

## 📦 Dependencies

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) (via Rust bindings)
- [FFmpeg](https://ffmpeg.org/) (for format conversion)

## 🤝 Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## 📜 License

MIT License - see [LICENSE](LICENSE) file for details

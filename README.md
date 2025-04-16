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
   git clone https://github.com/your-username/YT-Downloader-Rust.git
   cd YT-Downloader-Rust/src/
   cargo build
   cargo run
   ```
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

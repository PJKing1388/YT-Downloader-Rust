# 🎬 YouTube Downloader (Rust) 🦀

یک دانلودر ساده و کارآمد برای ویدیوهای یوتیوب نوشته شده با Rust

![Rust Version](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue)

## ✨ ویژگی‌های کلیدی
- دانلود ویدیو با کیفیت‌های مختلف
- امکان استخراج صوت (MP3)
- پشتیبانی از نام و مسیر دلخواه برای فایل خروجی
- نمایش نوار پیشرفت هنگام دانلود
- سرعت بالا و مصرف منابع بهینه

## 📥 نصب و راه‌اندازی

### پیش‌نیازها
- نصب [Rust](https://www.rust-lang.org/tools/install) (نسخه پایدار)
- نصب [FFmpeg](https://ffmpeg.org/) (برای تبدیل فرمت)

### مراحل نصب
```bash
git clone https://github.com/your-username/youtube-downloader-rust.git
cd youtube-downloader-rust/
cargo build --release
cargo run --release
```
### 💡 تگ های کاربردی

```
# دانلود با بهترین کیفیت
cargo run --release -- https://youtu.be/example

# تبدیل به MP3
cargo run --release -- https://youtu.be/example -f mp3

# ذخیره با نام دلخواه
cargo run --release -- https://youtu.be/example -o "ویدیوی_من.mp4"
```
📦 وابستگی‌ها

    yt-dlp (از طریق بایندینگ‌های راست)

    FFmpeg (برای تبدیل فرمت)

🤝 مشارکت

مشارکت‌ها مورد استقبال هستند! لطفاً issue جدید باز کنید یا pull request ارسال کنید.
📜 مجوز

مجوز MIT - برای جزئیات به فایل LICENSE مراجعه کنید

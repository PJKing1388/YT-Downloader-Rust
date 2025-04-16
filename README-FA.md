🎬 دانلودر یوتیوب (راست) 🦀

یک دانلودر ساده اما کارآمد برای ویدیوهای یوتیوب نوشته شده با راست.

راست
مجوز
✨ ویژگی‌ها

    📥 دانلود ویدیوهای یوتیوب در فرمت‌ها و کیفیت‌های مختلف

    🎵 استخراج فقط صوت

    📝 نام و مسیر دلخواه برای خروجی

    📊 نوار پیشرفت برای نمایش روند دانلود

    ⚡ سبک‌وزن و سریع

📋 پیش‌نیازها

    🦀 راست (نسخه پایدار جدید توصیه می‌شود)

    � FFmpeg (برای تبدیل فرمت و استخراج صوت لازم است)

🛠️ نصب

    کلون کردن مخزن:
    bash
    Copy

    git clone https://github.com/your-username/youtube-downloader-rust.git  
    cd youtube-downloader-rust/src/  

    ساخت پروژه:
    bash
    Copy

    cargo build --release  

🚀 نحوه استفاده

اجرا با لینک یوتیوب:
bash
Copy

cargo run --release -- <لینک_یوتیوب>  

🎛️ گزینه‌ها:

    -o, --output <مسیر>: تنظیم مسیر/نام دلخواه برای فایل خروجی

    -f, --format <فرمت>: مشخص کردن فرمت (mp4, webm, mp3 و غیره)

    -q, --quality <کیفیت>: تنظیم کیفیت ویدیو (highest, lowest, 720p و غیره)

    --audio-only: فقط دانلود صوت

💡 مثال‌ها:
bash
Copy

# دانلود با بالاترین کیفیت  
cargo run --release -- https://www.youtube.com/watch?v=dQw4w9WgXcQ  

# دانلود به صورت MP3  
cargo run --release -- https://www.youtube.com/watch?v=dQw4w9WgXcQ -f mp3  

# نام دلخواه برای فایل خروجی  
cargo run --release -- https://www.youtube.com/watch?v=dQw4w9WgXcQ -o "ویدیوی_من.mp4"  

📦 وابستگی‌ها

    yt-dlp (از طریق بایندینگ‌های راست)

    FFmpeg (برای تبدیل فرمت)

🤝 مشارکت

مشارکت‌ها مورد استقبال هستند! لطفاً issue جدید باز کنید یا pull request ارسال کنید.
📜 مجوز

مجوز MIT - برای جزئیات به فایل LICENSE مراجعه کنید

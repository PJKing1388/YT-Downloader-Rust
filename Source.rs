use reqwest::blocking::get;
use std::fs::File;
use std::io;
use std::error::Error;
use dialoguer::{Select, Input};
use youtube_dl::{YoutubeDl, YoutubeDlOutput};

fn main() -> Result<(), Box<dyn Error>> {
    let lang = Select::new()
        .with_prompt("زبان را انتخاب کنید / Select language")
        .items(&["فارسی", "English"])
        .default(0)
        .interact()?;
    let prompt = if lang == 0 { "لینک ویدیو را وارد کنید" } else { "Enter video URL" };
    let url: String = Input::new()
        .with_prompt(prompt)
        .interact_text()?;
    let output = YoutubeDl::new(&url)
        .socket_timeout("15")
        .run()?;
    let title = match output {
        YoutubeDlOutput::SingleVideo(video) => video.title.unwrap_or_else(|| "video".to_string()),
        YoutubeDlOutput::Playlist(playlist) => playlist.entries
            .and_then(|entries| entries.into_iter().next())
            .and_then(|video| video.title)
            .unwrap_or_else(|| "video".to_string()),
    };
    let sanitized_title = title.chars()
        .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { '_' })
        .collect::<String>();
    let filename = format!("{}.mp4", sanitized_title);
    let download_prompt = if lang == 0 { "در حال دانلود..." } else { "Downloading..." };
    println!("{}", download_prompt);
    let response = get(&url)?;
    let mut file = File::create(&filename)?;
    io::copy(&mut response.bytes()?.as_ref(), &mut file)?;
    let done_msg = if lang == 0 { "دانلود کامل شد!" } else { "Download complete!" };
    println!("{}", done_msg);
    Ok(())
}
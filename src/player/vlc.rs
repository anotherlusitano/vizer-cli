use std::process::Command;

use crate::TRANSLATION;

pub fn open_vlc(video_url: &str) {
    println!("{}", TRANSLATION.vlc_start_misc_text);
    let output = Command::new("vlc")
        .args(["--fullscreen", "--play-and-exit", video_url])
        .spawn();

    match output {
        Ok(mut child) => match child.wait() {
            Ok(status) => {
                if status.success() {
                    print!("\x1B[2J\x1B[1;1H");
                    println!("{}", TRANSLATION.vlc_exit_misc_text);
                } else {
                    println!("{} {:?}", TRANSLATION.vlc_exit_with_err, status.code());
                }
            }
            Err(err) => {
                println!("{} {}", TRANSLATION.vlc_wait_err, err);
            }
        },
        Err(err) => {
            println!("{} {}", TRANSLATION.vlc_start_err, err);
        }
    }
}

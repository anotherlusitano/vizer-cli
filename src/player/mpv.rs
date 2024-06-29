use std::process::Command;

use crate::TRANSLATION;

pub fn open_mpv(video_url: &str) {
    let language = TRANSLATION.get().unwrap();

    println!("{}", language.players_start_misc_text);
    let output = Command::new("mpv")
        .args(["--fs", "--really-quiet", video_url])
        .spawn();

    match output {
        Ok(mut child) => match child.wait() {
            Ok(status) => {
                if status.success() {
                    print!("\x1B[2J\x1B[1;1H");
                } else {
                    println!("{} {:?}", language.mpv_exit_with_err, status.code());
                }
            }
            Err(err) => {
                println!("{} {}", language.mpv_wait_err, err);
            }
        },
        Err(err) => {
            println!("{} {}", language.mpv_start_err, err);
        }
    }
}

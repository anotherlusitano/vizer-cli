use std::process::Command;

pub fn open_vlc(video_url: &str) {
    println!("Starting the player");

    let output = Command::new("vlc")
        .args(["--fullscreen", "--play-and-exit", video_url])
        .spawn();

    match output {
        Ok(mut child) => match child.wait() {
            Ok(status) => {
                if status.success() {
                    print!("\x1B[2J\x1B[1;1H");
                    println!("Bye bye (^_^)/");
                } else {
                    println!("VLC exited with an error: {:?}", status.code());
                }
            }
            Err(err) => {
                println!("Failed to wait for VLC: {}", err);
            }
        },
        Err(err) => {
            println!("Failed to start VLC: {}", err);
        }
    }
}

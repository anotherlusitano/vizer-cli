use crate::player::{mpv::open_mpv, vlc::open_vlc};
use crate::USE_MPV;

pub fn play_video(video_url: &str) {
    let use_mpv = USE_MPV.get().unwrap();

    if *use_mpv {
        open_mpv(video_url);
    } else {
        open_vlc(video_url);
    }
}

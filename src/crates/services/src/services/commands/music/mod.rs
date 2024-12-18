use std::fmt::Display;

use shared::{shell, traits::Beautify};

#[derive(Debug, PartialEq, Clone)]
pub struct TrackInfo {
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    // platform: String,
}

impl Display for TrackInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatting_query = vec![];
        if let Some(artist_value) = &self.artist {
            formatting_query.push(format!("artist: {}", artist_value))
        }
        if let Some(album_value) = &self.album {
            formatting_query.push(format!("album: {}", album_value))
        }
        if let Some(title_value) = &self.title {
            formatting_query.push(format!("title: {}", title_value))
        }
        let res = String::from_iter(formatting_query);
        write!(f, "{}", res)
    }
}

impl Beautify for TrackInfo {
    fn beautiful_out(&self) -> String {
        let res = match self.album.as_ref() {
            None => format!(
                "🎧 <b>Сейчас играет:</b>\n\n{} — {}",
                self.artist.clone().unwrap_or_default(),
                self.title.clone().unwrap_or_default()
            ),
            Some(album) => format!(
                "🎧 <b>Сейчас играет:</b>\n\n{} — {}\nАльбом: {}",
                self.artist.clone().unwrap_or_default(),
                self.title.clone().unwrap_or_default(),
                album
            ),
        };
        res
    }
}

#[derive(Debug)]
pub enum MediaPlayingStatus {
    /// Media is currently playing.
    Playing(TrackInfo),
    /// Media is currently paused.
    Paused(TrackInfo),
    /// Media is currently stopped.
    Stopped,
    /// Maybe media is currently destroyed.
    Unknown,
}

impl Display for MediaPlayingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            MediaPlayingStatus::Playing(track_info) | MediaPlayingStatus::Paused(track_info) => {
                write!(f, "{}", track_info)
            }
            _ => write!(f, ""),
        }
    }
}

#[cfg(target_family = "unix")]
pub fn play_pause() {
    shell::execute_command(vec!["playerctl", "play-pause"]);
}

#[cfg(target_family = "unix")]
pub fn get_status() -> MediaPlayingStatus {
    let status_opt = shell::execute_command(vec!["playerctl", "status"]);

    let track_info = TrackInfo {
        title: pctl_metadat_prop("title"),
        artist: pctl_metadat_prop("artist"),
        album: pctl_metadat_prop("album"),
    };

    match status_opt {
        Some(status) => match status.as_str().trim() {
            "Playing" => MediaPlayingStatus::Playing(track_info),
            "Paused" => MediaPlayingStatus::Paused(track_info),
            "Stopped" => MediaPlayingStatus::Stopped,
            _ => MediaPlayingStatus::Unknown,
        },
        None => todo!(),
    }
}

#[cfg(target_family = "unix")]
pub fn play_next() {
    shell::execute_command(vec!["playerctl", "next"]).expect("playerctl next caused error");
}

#[cfg(target_family = "unix")]
fn pctl_metadat_prop(prop: &str) -> Option<String> {
    let prop_formatted = format!("{{{{{}}}}}", prop);
    let query = vec!["playerctl", "metadata", "--format", prop_formatted.as_str()];
    let prop_res = shell::execute_command(query).unwrap();
    if prop_res.trim().is_empty() {
        None
    } else {
        Some(prop_res)
    }
}

#[cfg(target_family = "unix")]
pub fn play_prev() {
    shell::execute_command(vec!["playerctl", "previous"]).expect("playerctl prev caused error");
}

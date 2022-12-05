use rspotify::model::{FullTrack, TrackId};

pub struct FilterResult {
    pub state: bool,
    pub track_id: TrackId,
}

pub fn filter_by_condition(
    condition_name: &str,
    condition_value: &str,
    track: FullTrack,
) -> FilterResult {
    let condition_state = match condition_name {
        "old-hindi" => {
            let release_year = track
                .album
                .release_date
                .as_ref()
                .unwrap()
                .split('-')
                .next()
                .unwrap();
            let release_year_value = release_year.parse::<i32>().unwrap();
            release_year_value < 1990
                && track.available_markets.len() == 1
                && track.available_markets[0] == "IN"
        }
        "artist" => {
            let track_artists = track.artists;
            let mut artist_matched = false;
            for artist in track_artists {
                if artist.name.to_lowercase().contains(condition_value) {
                    artist_matched = true;
                    break;
                }
            }
            artist_matched
        }
        _ => false,
    };
    FilterResult {
        state: condition_state,
        track_id: track.id.unwrap(),
    }
}

pub fn filter_condition_to_playlist_name(condition_name: &str, condition_value: &str) -> String {
    match condition_name {
        "old-hindi" => "Old hindi".to_string(),
        "artist" => {
            let prefix = "Best of";
            format!("{} {}", prefix, condition_value)
        }
        _ => "".to_string(),
    }
}

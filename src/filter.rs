use rspotify::model::{FullTrack, TrackId};
use rspotify::prelude::*;
use rspotify::{model::SearchType, AuthCodeSpotify};

pub struct FilterResult<'a> {
    pub state: bool,
    pub track_id: TrackId<'a>,
}

pub struct ArtistSearchResult {
    pub id: String,
    pub name: String,
    pub followers: u32,
    pub genres: Vec<String>,
}

pub async fn search_artists(
    spotify: &AuthCodeSpotify,
    query: &str,
    limit: u32,
) -> Result<Vec<ArtistSearchResult>, Box<dyn std::error::Error>> {
    let result = spotify
        .search(query, SearchType::Artist, None, None, Some(limit), None)
        .await;

    match result {
        Ok(rspotify::model::SearchResult::Artists(page)) => Ok(page
            .items
            .into_iter()
            .map(|artist| ArtistSearchResult {
                id: artist.id.id().to_string(),
                name: artist.name,
                followers: artist.followers.total,
                genres: artist.genres,
            })
            .collect()),
        Ok(_) => Ok(Vec::new()),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn filter_by_condition<'a>(
    condition_name: &'a str,
    condition_value: &'a str,
    track: FullTrack,
) -> FilterResult<'a> {
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
                if let Some(artist_id) = &artist.id {
                    if artist_id.id() == condition_value {
                        artist_matched = true;
                        break;
                    }
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

pub fn filter_removed_songs_with_no_avaliable_market(track: &FullTrack) -> bool {
    track.is_playable.unwrap() == false
}

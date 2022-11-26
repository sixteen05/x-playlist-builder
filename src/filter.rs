// if year_val < 1990
//             && item.track.available_markets.len() == 1
//             && item.track.available_markets[0] == "IN"

pub fn filter_by_condition(
    condition_name: String, 
    release_year: i32,
    available_markets: Vec<String>,
) -> bool {
    let condition_state = match condition_name.as_str() {
        "old-hindi" => {
            if release_year < 1990
            && available_markets.len() == 1
            && available_markets[0] == "IN" {
                true
        } else {
            false
        }
        },
        _ => {
            false
        }
    };
    return condition_state
}

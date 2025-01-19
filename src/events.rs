pub const ANY_TILE_SELECTED: &str = "selected-tile";
pub const ADDED_CHARACTER: &str = "added-char";

pub fn to_events(events: Vec<impl Into<String>>) -> String {
    events
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<String>>()
        .join(", ")
}

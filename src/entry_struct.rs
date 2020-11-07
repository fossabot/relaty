#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RateEntry {
    name: String,
    wins: u32,
    votes: u32,
}

impl RateEntry {
    pub fn new(name: String, wins: u32, votes: u32) -> RateEntry {
        RateEntry { name, wins, votes }
    }
}

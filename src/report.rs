
#[derive(Serialize, Deserialize, Debug)]
pub struct Entries(pub Vec<DayEntry>);

#[derive(Serialize, Deserialize, Debug)]
pub struct DayEntry {
    pub day_entry: DayEntryFields
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DayEntryFields {
    pub id: u64,
    pub notes: String,
    pub spent_at: String,
    pub hours: u64,
    pub user_id: u64,
    pub project_id: u64,
    pub task_id: u64,
    created_at: String,
    updated_at: String,
    pub timer_started_at: Option<String>,
    pub is_closed: bool,
    pub is_billed: bool,
}


impl Entries {
    pub fn base_url(domain: &str, id: u64, from: &str, to: &str) -> String {
        format!("https://{}.harvestapp.com/people/{}/entries?from={}&to={}", domain, id, from, to)
    }
}
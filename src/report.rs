
#[derive(Deserialize, Debug)]
pub struct Entries(Vec<DayEntry>);

#[derive(Deserialize, Debug)]
pub struct DayEntry {
    day_entry: DayEntryFields
}

#[derive(Deserialize, Debug)]
pub struct DayEntryFields {
    id: u64,
    notes: String,
    spent_at: String,
    hours: u64,
    user_id: u64,
    project_id: u64,
    task_id: u64,
    created_at: String,
    updated_at: String,
//    adjustment_records: bool,
    timer_started_at: Option<String>,
    is_closed: bool,
    is_billed: bool,
}


impl Entries {
    pub fn base_url(domain: &str, id: u64, from: &str, to: &str) -> String {
        format!("https://{}.harvestapp.com/people/{}/entries?from={}&to={}", domain, id, from, to)
    }
}
use chrono::*;

#[derive(Deserialize, Debug)]
pub struct TimesheetEntries {
    pub for_day: String,
    pub day_entries: Vec<TimesheetEntryFields>
}

#[derive(Deserialize, Debug)]
pub struct TimesheetEntryFields {
    pub id: u64,
    user_id: u64,
    pub notes: String,
    pub hours: f64,
    pub spent_at: String,
    created_at: String,
    updated_at: String,
    pub project_id: u64,
    pub task_id: u64,
    pub project: String,
    pub task: String,
    pub client: String,
    pub hours_without_timer: f64,
    pub started_at: String,
    pub ended_at: String
}

impl TimesheetEntries {

    pub fn base_url_daily(domain: &str, uid: u64) -> String {
        format!("https://{}.harvestapp.com/daily?of_user={}&slim=1", domain, uid)
    }

    pub fn base_url_for_day(domain: &str, uid: u64, day: &str) -> Result<String, ParseError> {
        let datetime = format!("{} 00:00:00 +02:00", day);
        println!("{}", datetime);
        let dt = DateTime::parse_from_str(&datetime, "%Y%m%d %H:%M:%S %z")?;
        let day_of_year = dt.ordinal();
        let year = dt.year();

        Ok(format!("https://{}.harvestapp.com/daily/{}/{}?of_user={}&slim=1", domain, day_of_year, year, uid))
    }

    pub fn base_url_entry(domain: &str, uid: u64, id: u64) -> String {
        format!("https://{}.harvestapp.com/daily/show/{}?of_user={}", domain, id, uid)
    }
}
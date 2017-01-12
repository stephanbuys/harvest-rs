
#[derive(Deserialize, Debug)]
pub struct Tasks(pub Vec<Task>);

#[derive(Deserialize, Debug)]
pub struct Task {
    pub task: TaskFields
}

#[derive(Deserialize, Debug)]
pub struct TaskFields {
    pub id: u64,
    pub name: String,
    pub billable_by_default: bool,
    created_at: String,
    updated_at: String,
    pub is_default: bool,
    pub default_hourly_rate: u64,
    pub deactivated: bool
}


impl Tasks {
    pub fn base_url(domain: &str) -> String {
        format!("https://{}.harvestapp.com/tasks", domain)
    }

    pub fn base_url_task(domain: &str, id: u64) -> String {
        format!("https://{}.harvestapp.com/tasks/{}", domain, id)
    }
}
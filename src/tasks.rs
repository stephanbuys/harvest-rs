
#[derive(Deserialize, Debug)]
pub struct Tasks(Vec<Task>);

#[derive(Deserialize, Debug)]
pub struct Task {
    task: TaskFields
}

#[derive(Deserialize, Debug)]
pub struct TaskFields {
    id: u64,
    name: String,
    billable_by_default: bool,
    created_at: String,
    updated_at: String,
    is_default: bool,
    default_hourly_rate: u64,
    deactivated: bool
}


impl Tasks {
    pub fn base_url(domain: &str) -> String {
        format!("https://{}.harvestapp.com/tasks", domain)
    }

    pub fn base_url_task(domain: &str, id: u64) -> String {
        format!("https://{}.harvestapp.com/tasks/{}", domain, id)
    }
}
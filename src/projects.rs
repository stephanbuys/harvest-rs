
#[derive(Deserialize, Debug)]
pub struct Projects(Vec<Project>);

#[derive(Deserialize, Debug)]
pub struct Project {
    project: ProjectFields
}

#[derive(Deserialize, Debug)]
pub struct ProjectFields {
    id: u64,
    client_id: u64,
    name: String,
    code: String,
    active: bool,
    billable: bool,
    bill_by: String,
    hourly_rate: Option<u64>,
    budget: Option<u64>,
    budget_by: String,
    notify_when_over_budget: bool,
    over_budget_notification_percentage: u64,
    over_budget_notified_at: Option<String>,
    show_budget_to_all: bool,
    created_at: String,
    updated_at: String,
    starts_on: Option<String>,
    ends_on: String,
    estimate: Option<u64>,
    estimate_by: String,
    notes: String,
}


impl Projects {
    pub fn base_url(domain: &str) -> String {
        format!("https://{}.harvestapp.com/projects", domain)
    }

    pub fn base_url_project(domain: &str, id: u64) -> String {
        format!("https://{}.harvestapp.com/projects/{}", domain, id)
    }

    pub fn base_url_by_client(domain: &str, cid: u64) -> String {
        format!("https://{}.harvestapp.com/projects?client={}", domain, cid)
    }
}
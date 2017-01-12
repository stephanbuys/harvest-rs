
#[derive(Deserialize, Debug)]
pub struct Projects(pub Vec<Project>);

#[derive(Deserialize, Debug)]
pub struct Project {
    pub project: ProjectFields
}

#[derive(Deserialize, Debug)]
pub struct ProjectFields {
    pub id: u64,
    pub client_id: u64,
    pub name: String,
    pub code: String,
    pub active: bool,
    pub billable: bool,
    pub bill_by: String,
    pub hourly_rate: Option<u64>,
    pub budget: Option<u64>,
    pub budget_by: String,
    pub notify_when_over_budget: bool,
    pub over_budget_notification_percentage: u64,
    pub over_budget_notified_at: Option<String>,
    pub show_budget_to_all: bool,
    created_at: String,
    updated_at: String,
    pub starts_on: Option<String>,
    pub ends_on: String,
    pub estimate: Option<u64>,
    pub estimate_by: String,
    pub notes: String,
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
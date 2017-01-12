
#[derive(Deserialize, Debug)]
pub struct People(pub Vec<User>);

#[derive(Deserialize, Debug)]
pub struct User {
    pub user: UserFields
}

#[derive(Deserialize, Debug)]
pub struct UserFields {
    pub id: u64,
    pub email: String,
    pub is_admin: bool,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub default_hourly_rate: u64,
    pub department: String,
    pub cost_rate: Option<u64>,
}


impl People {
    pub fn base_url(domain: &str) -> String {
        format!("https://{}.harvestapp.com/people", domain)
    }

    pub fn base_url_user(domain: &str, id: u64) -> String {
        format!("https://{}.harvestapp.com/people/{}", domain, id)
    }
}
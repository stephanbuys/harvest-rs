
#[derive(Serialize, Deserialize, Debug)]
pub struct WhoAmICompany {
    pub base_uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhoAmIUser {
    pub timezone: String,
    pub timezone_identifier: String,
    pub id: u64,
    pub email: String,
    pub admin: bool,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhoAmI {
    pub company: WhoAmICompany,
    pub user: WhoAmIUser
}

impl WhoAmI {
    pub fn url(domain: &str) -> String {
        format!("https://{}.harvestapp.com/account/who_am_i", domain)
    }
}
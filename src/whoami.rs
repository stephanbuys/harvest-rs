
#[derive(Deserialize, Debug)]
pub struct WhoAmICompany {
    base_uri: String,
}

#[derive(Deserialize, Debug)]
pub struct WhoAmIUser {
    timezone: String,
    timezone_identifier: String,
    id: u64,
    email: String,
    admin: bool,
    first_name: String,
    last_name: String,

}

#[derive(Deserialize, Debug)]
pub struct WhoAmI {
    company: WhoAmICompany,
    user: WhoAmIUser
}


impl WhoAmI {
    pub fn url(domain: &str) -> String {
        format!("https://{}.harvestapp.com/account/who_am_i", domain)
    }
}
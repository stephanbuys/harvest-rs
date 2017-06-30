
#[derive(Serialize, Deserialize, Debug)]
pub struct Clients(pub Vec<Client>);

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    pub client: ClientFields,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientFields {
    pub id: u64,
    pub name: String,
    pub active: bool,
    pub currency: String,
    pub currency_symbol: String,
    pub details: String,
}

impl Clients {
    pub fn base_url(domain: &str) -> String {
        format!("https://{}.harvestapp.com/clients", domain)
    }

    pub fn base_url_client(domain: &str, id: u64) -> String {
        format!("https://{}.harvestapp.com/clients/{}", domain, id)
    }
}

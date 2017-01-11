
#[derive(Deserialize, Debug)]
pub struct Clients(Vec<Client>);

#[derive(Deserialize, Debug)]
pub struct Client {
    client: ClientFields
}

#[derive(Deserialize, Debug)]
pub struct ClientFields {
    id: u64,
    name: String,
    active: bool,
    currency: String,
    currency_symbol: String,
    details: String,
//    default_invoice_timeframe: null,
//    last_invoice_kind: null
}


impl Clients {
    pub fn base_url(domain: &str) -> String {
        format!("https://{}.harvestapp.com/clients", domain)
    }

    pub fn base_url_client(domain: &str, id: u64) -> String {
        format!("https://{}.harvestapp.com/clients/{}", domain, id)
    }
}
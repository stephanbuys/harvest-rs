#[macro_use]
extern crate clap;
extern crate harvest;

use clap::{Arg, App};
use std::collections::BTreeMap;
use std::env::{self};

fn get_api_client() -> harvest::Client {
    let username = env::var("HARVEST_USERNAME")
        .expect("environment variable HARVEST_USERNAME not set");
    let password = env::var("HARVEST_PASSWORD")
        .expect("environment variable HARVEST_PASSWORD not set");
    let harvest_domain = env::var("HARVEST_DOMAIN")
        .expect("environment variable HARVEST_DOMAIN not set");

    harvest::Client::new_with(harvest_domain, username, password)
}

fn main() {
    let matches = App::new("Hours for Harvest")
        .version(crate_version!())
        .author("Stephan Buys <stephan.buys@panoptix.co.za>")
        .about("Pulls the current time spent per active employee.")
        .arg(Arg::with_name("day")
            .short("d")
            .long("day")
            .value_name("DAY")
            .help("For a specific day (format: YYYYMMDD)")
            .takes_value(true))
        .get_matches();

    // day
    let day_of_search = matches.value_of("day");

    let mut summary = BTreeMap::new();
    let c = get_api_client();
    let users = c.people().unwrap();

    for u in users.0 {
        if !u.user.is_active {
            continue;
        }

        let name = format!("{} {}", u.user.first_name, u.user.last_name);

        let d = match day_of_search {
            None => c.day(u.user.id).unwrap(),
            Some(date) => c.day_for_date(u.user.id, date).unwrap()
        };
        let mut hours: f64 = 0.0;

        for e in d.day_entries {
            hours += e.hours;
        }

        let key = (format!("{}", d.for_day), name);
        summary.insert(key, hours);
    }

    for (k,v) in summary {
        println!("{},{},{}", k.0, k.1, v);
    }
}

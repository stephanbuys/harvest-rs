extern crate harvest;

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

#[test]
fn who_am_i() {

    let c = get_api_client();
    let who = c.who_am_i().unwrap();
    println!("{:#?}", who);

}

#[test]
fn people() {
    let c = get_api_client();
    let r = c.people().unwrap();
    println!("{:#?}", r);

}

#[test]
fn user() {
    let c = get_api_client();
    let r = c.user(649670).unwrap();
    println!("{:#?}", r);

}

#[test]
fn entries_2() {
    let c = get_api_client();
    let r = c.project_entries(1185442, "20161213", "20161213").unwrap();
    println!("{:#?}", r);

}
#[test]
fn entries() {
    let c = get_api_client();
    let r = c.project_entries(649670, "20170109", "20170109").unwrap();
    println!("{:#?}", r);

}

#[test]
fn entry() {
    let c = get_api_client();
    let r = c.entry(649670, 558482078).unwrap();
    println!("{:#?}", r);

}

#[test]
fn day() {
    let c = get_api_client();
    let r = c.day(649670).unwrap();
    println!("{:#?}", r);

}

#[test]
fn day_for_date() {
    let c = get_api_client();
    let r = c.day_for_date(649670, "20170109").unwrap();
    println!("{:#?}", r);

}


#[test]
fn clients() {
    let c = get_api_client();
    let r = c.clients().unwrap();
    println!("{:#?}", r);

}

#[test]
fn client() {
    let c = get_api_client();
    let r = c.client(2573214).unwrap();
    println!("{:#?}", r);

}

#[test]
fn tasks() {
    let c = get_api_client();
    let r = c.tasks().unwrap();
    println!("{:#?}", r);

}

#[test]
fn task() {
    let c = get_api_client();
    let r = c.task(2673117).unwrap();
    println!("{:#?}", r);

}

#[test]
fn projects() {
    let c = get_api_client();
    let r = c.projects().unwrap();
    println!("{:#?}", r);

}
#[test]
fn project() {
    let c = get_api_client();
    let r = c.project(12080300).unwrap();
    println!("{:#?}", r);

}

#[test]
fn projects_by_client() {
    let c = get_api_client();
    let r = c.projects_by_client(2573214).unwrap();
    println!("{:#?}", r);

}


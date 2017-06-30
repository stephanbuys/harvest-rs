extern crate harvest;
extern crate chrono;

use std::env;
use chrono::*;

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
    // Get the 1st user
    let mut users = c.people().unwrap();
    let u = users.0.pop().unwrap();
    let r = c.user(u.user.id).unwrap();
    println!("{:#?}", r);

}

#[test]
fn entries() {
    let c = get_api_client();
    // Get the 1st user
    let mut users = c.people().unwrap();
    let u = users.0.pop().unwrap();
    // Today's date string
    let dt = Local::now();
    let d = dt.format("%Y%m%d").to_string();
    let r = c.project_entries(u.user.id, &d, &d).unwrap();
    println!("{:#?}", r);
}

#[test]
fn entry() {
    let c = get_api_client();
    // Get the 1st user
    let mut users = c.people().unwrap();
    let u = users.0.pop().unwrap();
    // Today's date string
    let dt = Local::now();
    let d = dt.format("%Y%m%d").to_string();
    let mut entries = c.project_entries(u.user.id, &d, &d).unwrap();
    // Get the 1st entry
    let e = entries.0.pop();
    match e {
        None => println!("No entries"),
        Some(e) => {
            let e = c.entry(u.user.id, e.day_entry.id).unwrap();
            println!("{:#?}", e);
        }
    }
}

#[test]
fn day() {
    let c = get_api_client();
    // Get the 1st user
    let mut users = c.people().unwrap();
    let u = users.0.pop().unwrap();
    let r = c.day(u.user.id).unwrap();
    println!("{:#?}", r);

}


#[test]
fn day_for_date() {
    let c = get_api_client();
    // Get the 1st user
    let mut users = c.people().unwrap();
    let u = users.0.pop().unwrap();
    // Today    's date string
    let dt = Local::now();
    let d = dt.format("%Y%m%d").to_string();
    let r = c.day_for_date(u.user.id, &d).unwrap();
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
    // Get the 1st client
    let mut clients = c.clients().unwrap();
    let x = clients.0.pop().unwrap();
    let r = c.client(x.client.id).unwrap();
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
    // Get the 1st client
    let mut tasks = c.tasks().unwrap();
    let t = tasks.0.pop().unwrap();
    let r = c.task(t.task.id).unwrap();
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
    // Get the 1st project
    let mut projects = c.projects().unwrap();
    let p = projects.0.pop().unwrap();
    let r = c.project(p.project.id).unwrap();
    println!("{:#?}", r);

}

#[test]
fn projects_by_client() {
    let c = get_api_client();
    // Get the 1st client
    let mut clients = c.clients().unwrap();
    let x = clients.0.pop().unwrap();
    let r = c.projects_by_client(x.client.id).unwrap();
    println!("{:#?}", r);

}

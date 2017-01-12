#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate chrono;

extern crate hyper;
extern crate reqwest;

use hyper::header::{Authorization, Basic};
use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel};
use reqwest::Client as reqwest_Client;
use reqwest::header::{Headers, Accept, qitem};

pub mod whoami;
pub mod people;
pub mod report;
pub mod clients;
pub mod tasks;
pub mod projects;
pub mod timesheet;

pub struct Client {
    subdomain: String,
    username: String,
    password: String,
}

impl Client {
    pub fn new_with(subdomain: String, username: String, password: String) -> Client {

        Client {
            subdomain: subdomain,
            username: username,
            password: password
        }
    }

    pub fn request(&self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        let client = reqwest_Client::new().expect("client failed to construct");
        let mut headers = Headers::new();

        headers.set(Accept(vec![
        qitem(Mime(TopLevel::Application, SubLevel::Json, vec![]))
        ]));

        headers.set(
            ContentType(Mime(TopLevel::Application, SubLevel::Json,
                             vec![]))
        );

        headers.set(
            Authorization(
                Basic {
                    username: self.username.to_owned(),
                    password: Some(self.password.to_owned())
                }
            )
        );

        let res = client.get(url)
            .headers(headers.clone())
            .send();

        res
    }

    pub fn who_am_i(&self) -> Result<whoami::WhoAmI, reqwest::Error> {
        let url = self::whoami::WhoAmI::url(&self.subdomain);
        let mut res = self.request(&url)?;
        let body_as_json: whoami::WhoAmI = res.json()?;
        Ok(body_as_json)
    }

    pub fn people(&self) -> Result<people::People, reqwest::Error> {
        let url = self::people::People::base_url(&self.subdomain);
        let mut res = self.request(&url)?;
        let body_as_json: people::People = res.json()?;
        Ok(body_as_json)
    }

    pub fn user(&self, id: u64) -> Result<people::User, reqwest::Error> {
        let url = self::people::People::base_url_user(&self.subdomain, id);
        let mut res = self.request(&url)?;
        let body_as_json: people::User = res.json()?;
        Ok(body_as_json)
    }

    pub fn day(&self, uid: u64) -> Result<timesheet::TimesheetEntries, reqwest::Error> {
        let url = self::timesheet::TimesheetEntries::base_url_daily(&self.subdomain, uid);
        let mut res = self.request(&url)?;
        let body_as_json: timesheet::TimesheetEntries = res.json()?;
        Ok(body_as_json)
    }

    pub fn day_for_date(&self, uid: u64, date: &str) -> Result<timesheet::TimesheetEntries, reqwest::Error> {
        let url = self::timesheet::TimesheetEntries::base_url_for_day(&self.subdomain, uid, date).expect("Could not parse date string");
        let mut res = self.request(&url)?;
        let body_as_json: timesheet::TimesheetEntries = res.json()?;
        Ok(body_as_json)
    }

    pub fn entry(&self, uid: u64, id: u64) -> Result<timesheet::TimesheetEntryFields, reqwest::Error> {
        let url = self::timesheet::TimesheetEntries::base_url_entry(&self.subdomain, uid, id);
        let mut res = self.request(&url)?;
        let body_as_json: timesheet::TimesheetEntryFields = res.json()?;
        Ok(body_as_json)
    }

    pub fn project_entries(&self, id: u64, from: &str, to: &str) -> Result<report::Entries, reqwest::Error> {
        let url = self::report::Entries::base_url(&self.subdomain, id, from, to);
        let mut res = self.request(&url)?;
        let body_as_json: report::Entries = res.json()?;
        Ok(body_as_json)
    }

    pub fn client(&self, id: u64) -> Result<clients::Client, reqwest::Error> {
        let url = self::clients::Clients::base_url_client(&self.subdomain,id);
        let mut res = self.request(&url)?;
        let body_as_json: clients::Client = res.json()?;
        Ok(body_as_json)
    }
    pub fn clients(&self) -> Result<clients::Clients, reqwest::Error> {
        let url = self::clients::Clients::base_url(&self.subdomain);
        let mut res = self.request(&url)?;
        let body_as_json: clients::Clients = res.json()?;
        Ok(body_as_json)
    }

    pub fn task(&self, id: u64) -> Result<tasks::Task, reqwest::Error> {
        let url = self::tasks::Tasks::base_url_task(&self.subdomain,id);
        let mut res = self.request(&url)?;
        let body_as_json: tasks::Task = res.json()?;
        Ok(body_as_json)
    }
    pub fn tasks(&self) -> Result<tasks::Tasks, reqwest::Error> {
        let url = self::tasks::Tasks::base_url(&self.subdomain);
        let mut res = self.request(&url)?;
        let body_as_json: tasks::Tasks = res.json()?;
        Ok(body_as_json)
    }

    pub fn projects(&self) -> Result<projects::Projects, reqwest::Error> {
        let url = self::projects::Projects::base_url(&self.subdomain);
        let mut res = self.request(&url)?;
        let body_as_json: projects::Projects = res.json()?;
        Ok(body_as_json)
    }

    pub fn project(&self, id: u64) -> Result<projects::Project, reqwest::Error> {
        let url = self::projects::Projects::base_url_project(&self.subdomain, id);
        let mut res = self.request(&url)?;
        let body_as_json: projects::Project = res.json()?;
        Ok(body_as_json)
    }

    pub fn projects_by_client(&self, cid: u64) -> Result<projects::Projects, reqwest::Error> {
        let url = self::projects::Projects::base_url_by_client(&self.subdomain, cid);
        let mut res = self.request(&url)?;
        let body_as_json: projects::Projects = res.json()?;
        Ok(body_as_json)
    }
}
use reqwest::{Client, IntoUrl};
use std::collections::HashMap;

pub struct Session {
    session: Client,
    headers: HashMap<String, String>,
}

impl Session {
    pub fn get<U: IntoUrl>(&self, url: U) {}

    pub fn post<U: IntoUrl>(&self, url: U) {}

    pub fn put<U: IntoUrl>(&self, url: U) {}

    pub fn patch<U: IntoUrl>(&self, url: U) {}

    pub fn delete<U: IntoUrl>(&self, url: U) {}

    pub fn head<U: IntoUrl>(&self, url: U) {}
}

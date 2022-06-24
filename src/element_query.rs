use std::fmt::Display;

use crate::depend::*;
use crate::prelude::*;

impl Ele {
    pub fn query(&self, selector: &str) -> Option<Ele> {
        if let Ok(query) = self.ele.query_selector(selector) {
            match query {
                Some(ele) => return Some(from_element(ele)),
                _ => return None,
            }
        }

        None
    }
    pub fn query_all(&self, selector: &str) -> Vec<Ele> {
        if let Ok(nodelist) = self.ele.query_selector_all(selector) {
            return nodelist_to_eles(nodelist);
        }

        vec![]
    }
}

pub fn document_query(selector: &str) -> Option<Ele> {
    if let Ok(query) = doc().query_selector(selector) {
        match query {
            Some(ele) => return Some(from_element(ele)),
            _ => return None,
        }
    }

    None
}
pub fn document_query_all(selector: &str) -> Vec<Ele> {
    if let Ok(nodelist) = doc().query_selector_all(selector) {
        return nodelist_to_eles(nodelist);
    }

    vec![]
}
pub fn body_query(selector: &str) -> Option<Ele> {
    if let Ok(query) = doc().body().unwrap_throw().query_selector(selector) {
        match query {
            Some(ele) => return Some(from_element(ele)),
            _ => return None,
        }
    }

    None
}
pub fn body_query_all(selector: &str) -> Vec<Ele> {
    if let Ok(nodelist) = doc().body().unwrap_throw().query_selector_all(selector) {
        return nodelist_to_eles(nodelist);
    }

    vec![]
}

pub fn get_element_by_id<T: Display>(id: T) -> Option<Ele> {
    if let Some(ele) = doc().get_element_by_id(&id.to_string()) {
        return Some(from_element(ele));
    }
    None
}

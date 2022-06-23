use crate::depend::*;
use crate::prelude::*;

impl Ele {
    pub fn query(&self, selector: &str) -> Option<Ele> {
        if let Some(ele) = self.ele.query_selector(selector).unwrap_throw() {
            return Some(Ele::new(ele));
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

impl Ele {
    pub fn document_query(selector: &str) -> Option<Ele> {
        if let Some(ele) = doc().query_selector(selector).unwrap_throw() {
            return Some(Ele::new(ele));
        }

        None
    }
    pub fn document_query_all(&self, selector: &str) -> Vec<Ele> {
        if let Ok(nodelist) = doc().query_selector_all(selector) {
            return nodelist_to_eles(nodelist);
        }

        vec![]
    }
    pub fn body_query(selector: &str) -> Option<Ele> {
        if let Some(ele) = doc()
            .body()
            .unwrap_throw()
            .query_selector(selector)
            .unwrap_throw()
        {
            return Some(Ele::new(ele));
        }

        None
    }
    pub fn body_query_all(&self, selector: &str) -> Vec<Ele> {
        if let Ok(nodelist) = doc().body().unwrap_throw().query_selector_all(selector) {
            return nodelist_to_eles(nodelist);
        }

        vec![]
    }
    pub fn get_element_by_id(id: &str) -> Option<Ele> {
        if let Some(ele) = doc().get_element_by_id(id) {
            return Some(Ele::new(ele));
        }

        None
    }
}

use crate::depend::*;
use crate::prelude::*;

impl Ele {
    pub fn remove(&self) {
        self.ele.remove()
    }
    pub fn append(&self, list: Vec<&Ele>) -> &Self {
        for v in list.iter() {
            self.ele.append_child(v.ele.as_ref()).unwrap_throw();
        }
        self
    }
    pub fn append_child(&self, target: &Ele) -> &Self {
        self.ele.append_child(target.ele.as_ref()).unwrap_throw();
        self
    }
    pub fn children(&self, list: Vec<&Ele>) -> &Self {
        self.ele.set_inner_html("");
        for v in list.iter() {
            self.ele.append_child(v.ele.as_ref()).unwrap_throw();
        }
        self
    }
}

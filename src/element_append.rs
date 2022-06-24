use crate::depend::*;
use crate::prelude::*;

impl Ele {
    pub fn remove(self) -> Self {
        self.ele.remove();
        self
    }
    pub fn get_parent(self) -> Option<Self> {
        self.ele.parent_element().map(from_element)
    }
    pub fn append(self, list: Vec<&Ele>) -> Self {
        for v in list.iter() {
            self.ele.append_child(v.ele.as_ref()).unwrap_throw();
        }
        self
    }
    pub fn append_child(self, target: &Ele) -> Self {
        self.ele.append_child(target.ele.as_ref()).unwrap_throw();
        self
    }
    pub fn children(self, list: Vec<Ele>) -> Self {
        self.ele.set_inner_html("");

        for v in list.iter() {
            self.ele.append_child(v.ele.as_ref()).unwrap_throw();
        }
        self
    }
    pub fn render(self, render: Box<dyn Fn() -> Vec<Ele>>) -> Self {
        self.modify(
            "render",
            Box::new(move |target| {
                target.ele.set_inner_html("");
                for v in render().iter() {
                    target.ele.append_child(v.ele.as_ref()).unwrap_throw();
                }
            }),
        )
    }
}

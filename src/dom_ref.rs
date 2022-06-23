use crate::depend::*;
use crate::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

// #[allow(dead_code)]
// #[derive(Debug, Clone)]
// pub struct Dom {
//     #[allow(dead_code)]
//     ele: Rc<RefCell<HtmlElement>>,
// }

// impl Dom {
//     pub fn new(target: Element) -> Self {
//         let ele: HtmlElement = target.dyn_into().unwrap_throw();

//         Self {
//             ele: Rc::new(RefCell::new(ele)),
//         }
//     }
//     pub fn new_tag(tag: &str) -> Self {
//         let ele = doc().create_element(intern(tag)).unwrap_throw();
//         Dom::new(ele)
//     }
//     pub fn set_style_text(&self, style: &str) -> &Self {
//         self.ele
//             .borrow()
//             .set_attribute(intern("style"), style)
//             .unwrap_throw();
//         self
//     }
//     pub fn set_text(&self, text: &str) -> &Self {
//         self.ele.borrow().set_text_content(Some(text));
//         self
//     }
//     pub fn onclick(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
//         let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
//         self.ele
//             .borrow()
//             .set_onclick(Some(a.as_ref().unchecked_ref()));
//         a.forget();
//         self
//     }
//     pub fn oninput(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
//         let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
//         self.ele
//             .borrow()
//             .set_oninput(Some(a.as_ref().unchecked_ref()));
//         a.forget();
//         self
//     }
//     pub fn value(&self) -> String {
//         self.get_string("value")
//     }
//     pub fn get_string(&self, name: &str) -> String {
//         #[allow(unused_unsafe)]
//         unsafe {
//             match js_sys::Reflect::get(&self.ele.borrow(), &name.into())
//                 .unwrap_throw()
//                 .as_string()
//             {
//                 Some(v) => v,
//                 None => "".into(),
//             }
//         }
//     }
//     pub fn get_element_by_id(id: &str) -> Option<Dom> {
//         if let Some(ele) = doc().get_element_by_id(id) {
//             return Some(Dom::new(ele));
//         }

//         None
//     }
//     pub fn get_current_target(e: Event) -> Self {
//         let ele: Element = e.current_target().unwrap_throw().dyn_into().unwrap_throw();

//         Dom::new(ele)
//     }
//     pub fn append_child(&self, target: &Dom) -> &Self {
//         self.ele
//             .borrow()
//             .append_child(target.ele.borrow().as_ref())
//             .unwrap_throw();
//         self
//     }
// }

#[derive(Debug, Clone)]
pub struct Dom {
    ele: HtmlElement,
}

impl Dom {
    pub fn new(target: Element) -> Rc<RefCell<Self>> {
        let ele: HtmlElement = target.dyn_into().unwrap_throw();

        Rc::new(RefCell::new(Self { ele }))
    }
    pub fn new_tag(tag: &str) -> Rc<RefCell<Self>> {
        let ele = doc().create_element(intern(tag)).unwrap_throw();
        Dom::new(ele)
    }
    pub fn set_style_text(&self, style: &str) -> &Self {
        self.ele
            .set_attribute(intern("style"), style)
            .unwrap_throw();
        self
    }
    pub fn set_text(&self, text: &str) -> &Self {
        self.ele.set_text_content(Some(text));
        self
    }
    pub fn onclick(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_onclick(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn oninput(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_oninput(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn value(&self) -> String {
        self.get_string("value")
    }
    pub fn get_string(&self, name: &str) -> String {
        #[allow(unused_unsafe)]
        unsafe {
            match js_sys::Reflect::get(&self.ele, &name.into())
                .unwrap_throw()
                .as_string()
            {
                Some(v) => v,
                None => "".into(),
            }
        }
    }
    pub fn get_element_by_id(id: &str) -> Option<Rc<RefCell<Self>>> {
        if let Some(ele) = doc().get_element_by_id(id) {
            return Some(Dom::new(ele));
        }

        None
    }
    pub fn get_current_target(e: Event) -> Rc<RefCell<Self>> {
        let ele: Element = e.current_target().unwrap_throw().dyn_into().unwrap_throw();

        Dom::new(ele)
    }
    pub fn append_child(&self, target: &Dom) -> &Self {
        self.ele.append_child(target.ele.as_ref()).unwrap_throw();
        self
    }
}

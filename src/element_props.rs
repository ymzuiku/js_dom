use std::fmt::Display;

use crate::depend::*;
use crate::prelude::*;

impl Ele {
    pub fn rc_refcell(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }
    pub fn rc(self) -> Rc<Self> {
        Rc::new(self)
    }
    pub fn inner_html(self, text: &str) -> Self {
        self.ele.set_inner_html(intern(text));
        self
    }
    pub fn get_inner_html(self) -> String {
        self.ele.inner_html()
    }
    pub fn id<T: Display>(self, id: T) -> Self {
        self.ele.set_id(&id.to_string());
        self
    }
    pub fn get_id(&self) -> String {
        self.ele.id()
    }
    pub fn get_id_f64(&self) -> f64 {
        self.ele.id().parse().unwrap_throw()
    }
    pub fn text(self, text: &str) -> Self {
        self.ele.set_text_content(Some(text));
        self
    }
    pub fn text_i32(self, text: i32) -> Self {
        self.text(&text.to_string())
    }
    pub fn text_f64(self, text: f64) -> Self {
        self.text(&text.to_string())
    }
    pub fn get_text(&self) -> String {
        match self.ele.text_content() {
            Some(v) => v,
            None => "".to_string(),
        }
    }

    pub fn style_text(self, style: &str) -> Self {
        self.ele
            .set_attribute(intern("style"), style)
            .unwrap_throw();
        self
    }
    pub fn get_style_text(&self) -> String {
        self.get_attr("style")
    }
    pub fn set_class(self, style: &str) -> Self {
        self.ele
            .set_attribute(intern("class"), style)
            .unwrap_throw();
        self
    }
    pub fn get_class(&self) -> String {
        self.ele.class_name()
    }
    pub fn add_class(self, class: &str) -> Self {
        self.ele.class_list().add_1(class).unwrap_throw();
        self
    }
    pub fn remove_class(self, class: &str) -> Self {
        self.ele.class_list().remove_1(class).unwrap_throw();
        self
    }
    pub fn contains_class(&self, class: &str) -> bool {
        self.ele.class_list().contains(class)
    }
    pub fn set_attr(self, name: &str, value: &str) -> Self {
        self.ele
            .set_attribute(intern(name), intern(value))
            .unwrap_throw();
        self
    }
    pub fn get_attr(&self, name: &str) -> String {
        match self.ele.get_attribute(intern(name)) {
            Some(v) => v,
            None => "".to_string(),
        }
    }
    pub fn has_attr(&self, name: &str) -> bool {
        self.ele.has_attribute(name)
    }
    pub fn remove_attr(self, name: &str) -> Self {
        let _ = self.ele.remove_attribute(name);
        self
    }
    pub fn clone_node(&self) -> Self {
        let node = self.ele.clone_node_with_deep(true).unwrap_throw();
        from_node(node)
    }
    pub fn value(self, value: &str) -> Self {
        self.ele.set_node_value(Some(intern(value)));
        self
    }
    pub fn get_value(&self) -> String {
        self.get("value")
    }
    pub fn set_js_value(self, name: &str, value: &JsValue) -> Self {
        #[allow(unused_unsafe)]
        unsafe {
            assert!(js_sys::Reflect::set(&self.ele, &name.into(), value).unwrap_throw());
        }
        self
    }
    pub fn set(self, name: &str, value: &str) -> Self {
        #[allow(unused_unsafe)]
        unsafe {
            assert!(
                js_sys::Reflect::set(&self.ele, &name.into(), &str_to_js(value)).unwrap_throw()
            );
        }
        self
    }
    pub fn get_js_value(&self, name: &str) -> JsValue {
        #[allow(unused_unsafe)]
        unsafe {
            js_sys::Reflect::get(&self.ele, &name.into()).unwrap_throw()
        }
    }
    pub fn get(&self, name: &str) -> String {
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
    pub fn delete(self, name: &str) -> Self {
        #[allow(unused_unsafe)]
        unsafe {
            assert!(js_sys::Reflect::delete_property(&self.ele, &name.into()).unwrap_throw());
        }
        self
    }
    pub fn placeholder(self, value: &str) -> Self {
        self.set_attr("placeholder", value)
    }
    pub fn get_placeholder(&self) -> String {
        self.get_attr("placeholder")
    }
    pub fn typed(self, value: &str) -> Self {
        self.set_attr("type", value)
    }
    pub fn get_typed(&self) -> String {
        self.get_attr("type")
    }
    pub fn name(self, value: &str) -> Self {
        self.set_attr("name", value)
    }
    pub fn get_name(&self) -> String {
        self.get_attr("name")
    }
    pub fn disabled(self, value: bool) -> Self {
        if value {
            self.set_attr("disabled", "true")
        } else {
            self.set_attr("disabled", "false")
        }
    }
    pub fn get_disable(&self) -> bool {
        self.get_attr("name") == "true"
    }
    pub fn selected(self, value: bool) -> Self {
        if value {
            self.set_attr("selected", "true")
        } else {
            self.set_attr("selected", "false")
        }
    }
    pub fn get_selectd(&self) -> bool {
        self.get_attr("name") == "true"
    }
}

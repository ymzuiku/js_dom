use crate::depend::*;
use crate::prelude::*;

impl Ele {
    pub fn set_inner_html(&self, text: &str) -> &Self {
        self.ele.set_inner_html(intern(text));
        self
    }
    pub fn inner_html(&self) -> String {
        self.ele.inner_html()
    }
    pub fn set_id(&self, id: &str) -> &Self {
        self.ele.set_id(id);
        self
    }
    pub fn id(&self) -> String {
        self.ele.id()
    }
    pub fn set_text(&self, text: &str) -> &Self {
        self.ele.set_text_content(Some(text));
        self
    }
    pub fn set_text_i32(&self, text: i32) -> &Self {
        self.set_text(&text.to_string())
    }
    pub fn set_text_f64(&self, text: f64) -> &Self {
        self.set_text(&text.to_string())
    }
    pub fn text(&self) -> String {
        match self.ele.text_content() {
            Some(v) => v,
            None => "".to_string(),
        }
    }

    pub fn set_style_text(&self, style: &str) -> &Self {
        self.ele
            .set_attribute(intern("style"), style)
            .unwrap_throw();
        self
    }
    pub fn style_text(&self) -> String {
        self.attr("style")
    }
    pub fn set_class(&self, style: &str) -> &Self {
        self.ele
            .set_attribute(intern("class"), style)
            .unwrap_throw();
        self
    }
    pub fn class(&self) -> String {
        self.ele.class_name()
    }
    pub fn add_class(&self, class: &str) -> &Self {
        self.ele.class_list().add_1(class).unwrap_throw();
        self
    }

    pub fn remove_class(&self, class: &str) -> &Self {
        self.ele.class_list().remove_1(class).unwrap_throw();
        self
    }
    pub fn contains_class(&self, class: &str) -> bool {
        self.ele.class_list().contains(class)
    }
    pub fn set_attr(&self, name: &str, value: &str) -> &Self {
        self.ele
            .set_attribute(intern(name), intern(value))
            .unwrap_throw();
        self
    }
    pub fn attr(&self, name: &str) -> String {
        match self.ele.get_attribute(intern(name)) {
            Some(v) => v,
            None => "".to_string(),
        }
    }
    pub fn has_attr(&self, name: &str) -> bool {
        self.ele.has_attribute(name)
    }
    pub fn clone_node(&self) -> Self {
        let node = self.ele.clone_node_with_deep(true).unwrap_throw();
        Self::new_node(node)
    }
    pub fn set_value(&self, value: &str) -> &Self {
        self.ele.set_node_value(Some(intern(value)));
        self
    }
    pub fn value(&self) -> String {
        self.get_string("value")
    }
    pub fn set(&self, name: &str, value: &JsValue) {
        #[allow(unused_unsafe)]
        unsafe {
            assert!(js_sys::Reflect::set(&self.ele, &name.into(), value).unwrap_throw());
        }
    }
    pub fn set_str(&self, name: &str, value: &str) {
        #[allow(unused_unsafe)]
        unsafe {
            assert!(
                js_sys::Reflect::set(&self.ele, &name.into(), &str_to_js(value)).unwrap_throw()
            );
        }
    }
    pub fn get(&self, name: &str) -> JsValue {
        #[allow(unused_unsafe)]
        unsafe {
            js_sys::Reflect::get(&self.ele, &name.into()).unwrap_throw()
        }
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
    pub fn delete(&self, name: &str) {
        #[allow(unused_unsafe)]
        unsafe {
            assert!(js_sys::Reflect::delete_property(&self.ele, &name.into()).unwrap_throw());
        }
    }
    pub fn placeholder(&self, value: &str) -> &Self {
        self.set_attr("placeholder", value);
        self
    }
}

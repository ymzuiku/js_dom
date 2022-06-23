// pub fn ontouchstart(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
//   let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
//   self.ele.set_ontouchstart(Some(a.as_ref().unchecked_ref()));
//   a.forget();
//   self
// }

pub fn set_timeout() {}

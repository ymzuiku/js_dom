use crate::depend::*;
use crate::prelude::*;

impl Ele {
    #[inline]
    pub fn onevent(&self, name: &str, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele
            .add_event_listener_with_callback(intern(name), a.as_ref().unchecked_ref())
            .unwrap_throw();
        a.forget();
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
    pub fn onchange(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_onchange(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn onscroll(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_onscroll(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn onwheel(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_onwheel(Some(a.as_ref().unchecked_ref()));
        self
    }
    pub fn onblur(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_onblur(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn onfocus(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_onfocus(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn onsubmit(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_onsubmit(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn onselect(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_onselect(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn onresize(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_onresize(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn onerror(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_onerror(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn ondrag(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_ondrag(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn ondragenter(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_ondragenter(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn ondragexit(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_ondragexit(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn ondragleave(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_ondragleave(Some(a.as_ref().unchecked_ref()));
        self
    }
    pub fn ondragstart(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_ondragstart(Some(a.as_ref().unchecked_ref()));
        self
    }
    pub fn onkeydown(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_onkeydown(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn onkeypress(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_onkeypress(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn onkeyup(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_onkeyup(Some(a.as_ref().unchecked_ref()));
        self
    }
    pub fn onmousedown(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_onmousedown(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn onmouseenter(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_onmouseenter(Some(a.as_ref().unchecked_ref()));
        self
    }
    pub fn onmouseleave(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_onmouseleave(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn onmousemove(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_onmousemove(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn onmouseout(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_onmouseout(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn onmouseup(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_onmouseup(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn onmouseover(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_onmouseover(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn ontouchstart(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_ontouchstart(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn ontouchcancel(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_ontouchcancel(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn ontouchmove(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_ontouchmove(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
    pub fn ontouchend(&self, handle: Box<dyn FnMut(web_sys::Event)>) -> &Self {
        let a = Closure::wrap(handle as Box<dyn FnMut(web_sys::Event)>);
        self.ele.set_ontouchend(Some(a.as_ref().unchecked_ref()));
        a.forget();
        self
    }
}

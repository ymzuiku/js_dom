use web_sys::EventInit;

use crate::depend::*;
use crate::prelude::*;

// fn event<'a>(&self, ctx: ScopeRef<'a>, name: &str, handler: Box<dyn Fn(Self::EventType) + 'a>) {
//     // SAFETY: extend lifetime because the closure is dropped when the ctx is disposed,
//     // preventing the handler from ever being accessed after its lifetime.
//     let handler: Box<dyn Fn(Self::EventType) + 'static> =
//         unsafe { std::mem::transmute(handler) };
//     let closure = Closure::wrap(handler);
//     self.node
//         .add_event_listener_with_callback(intern(name), closure.as_ref().unchecked_ref())
//         .unwrap_throw();

//     ctx.on_cleanup(move || {
//         drop(closure);
//     });
// }

impl Ele {
    pub fn modify<T: Display>(self, id: T, handle: Box<dyn FnMut(Ele)>) -> Self {
        let id = &id.to_string();
        let mut handle = handle as Box<dyn FnMut(Ele)>;
        let next = self.set_attr(&format!("data-{}", id), "true");
        let a = Closure::wrap(Box::new(move |e| {
            let target = from_event_current(e);
            handle(target);
        }) as Box<dyn FnMut(Event)>);
        next.ele
            .add_event_listener_with_callback(intern(id), a.as_ref().unchecked_ref())
            .unwrap_throw();
        a.forget();
        ele_modify(&next, id);
        next
    }
}

pub fn dispatch_win<T: Display>(name: T) {
    let init = EventInit::default();
    let event = Event::new_with_event_init_dict(&name.to_string(), &init).unwrap_throw();
    win().dispatch_event(&event).unwrap_throw();
}

pub fn dispatch_modify<T: Display>(id: T) {
    let id = &id.to_string();
    let list = body_query_all(&format!("[data-{}]", id));
    let init = EventInit::default();
    let event = Event::new_with_event_init_dict(id, &init).unwrap_throw();
    for e in list {
        e.ele.dispatch_event(&event).unwrap_throw();
    }
}

pub fn ele_modify<T: Display>(target: &Ele, id: T) {
    let id = &id.to_string();
    // let list = body_query_all(&format!("[data-{}]", id));
    let init = EventInit::default();
    let event = Event::new_with_event_init_dict(id, &init).unwrap_throw();
    target.ele.dispatch_event(&event).unwrap_throw();
}

#![recursion_limit = "1024"]

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc & ::WeeAlloc::INIT;

pub use js_dom::depend::*;
pub use js_dom::prelude::*;
// pub use js_dom::prelude::*;

#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    app();
}

#[derive(Debug, Clone, Default)]
struct Dog {
    pub name: String,
    pub age: i32,
    pub event_id: String,
    pub label_id: String,
}

/// A read-only [`Signal`].
pub struct State<T>
where
    T: Clone,
{
    pub value: RefCell<Rc<T>>,
}

impl<T> State<T>
where
    T: Clone,
{
    pub fn get(&self) -> Rc<T> {
        self.value.borrow().clone()
    }
    pub fn get_mut(&self) -> Rc<T> {
        self.value.borrow_mut().clone()
    }
}

fn use_state<T>(t: T) -> State<T>
where
    T: Clone,
{
    State {
        value: RefCell::new(Rc::new(t)),
    }
}

fn app() {
    from_id("root").render(Box::new(move || {
        let state = use_state(Dog {
            name: "".into(),
            age: 0,
            event_id: nanoid(),
            label_id: nanoid(),
        });

        let event_id = &state.get().event_id;
        let label_id = &state.get().label_id;
        let state1 = state.get();
        let state2 = state.get();
        let state3 = state.get();
        let state4 = state.get_mut();
        let state5 = state.get_mut();
        let state6 = state.get();
        let state7 = state.get_mut();

        return vec![
            div().modify(
                event_id,
                Box::new(move |t| {
                    t.text(&state1.name.len().to_string())
                        .style_text("font-size:30px; color:#f00");
                }),
            ),
            div().modify(
                event_id,
                Box::new(move |t| {
                    let l = state2.name.len();
                    t.text(&l.to_string())
                        .style_text("font-size:30px; color:#f00");
                    println!("__debug__ {:?} {:?}", l > 15, l < 15);
                    if l > 15 {
                        println!("__debug__ {:?}", "111111");
                        ele_modify(&from_id("root"), "render");
                    }
                }),
            ),
            div()
                .text("label")
                .style_text("font-size:100px")
                .id(label_id),
            input()
                .style_text("background:#f00; font-size:24px;")
                .oninput(Box::new(move |e| {
                    let v = get_value(e);
                    let label = from_id(&state3.label_id);
                    label.text(&v);
                    // state4.name = v;
                    dispatch_modify(&state3.event_id);
                })),
            input()
                .style_text("background:#f00; font-size:24px;")
                .oninput(Box::new(move |e| {
                    let v = get_value(e);
                    let label = from_id(&state4.label_id);
                    label.text(&v);
                    // state5.name = v;
                    dispatch_modify(&state4.event_id);
                })),
            input()
                .style_text("background:#f00; font-size:24px;")
                .oninput(Box::new(move |e| {
                    let v = get_value(e);
                    let label = from_id(&state5.label_id);
                    label.text(&v);
                    // state7.name = v;
                    dispatch_modify(&state5.event_id);
                })),
        ];
    }));
}

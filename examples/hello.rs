#![recursion_limit = "1024"]

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub use js_dom::depend::*;
// pub use js_dom::dom_move::Dom;
pub use js_dom::dom::Dom;
// pub use js_dom::prelude::*;

#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    app();
}

fn app() {
    let rc_label = Dom::new_tag("div").rc();
    let label = rc_label.clone();
    label.set_text("222").set_style_text("font-size:100px");

    let label = rc_label.clone();
    let button = Dom::new_tag("button");
    button
        .set_text("hello")
        .set_style_text("background:#f00; font-size:24px;")
        .onclick(Box::new(move |_e| {
            label.set_text("123");
        }));

    let label = rc_label.clone();
    let input = Dom::new_tag("input");
    input
        .set_style_text("background:#f00; font-size:24px;")
        .oninput(Box::new(move |e| {
            let v = Dom::get_current_target(e).value();
            label.set_text(&v);
        }));

    let label = rc_label.clone();
    let input2 = Dom::new_tag("input");
    input2
        .set_style_text("background:#f00; font-size:24px;")
        .oninput(Box::new(move |e| {
            let v = Dom::get_current_target(e).value();
            label.set_text(&v);
        }));

    let root = Dom::get_element_by_id("root").unwrap();

    {
        // let _label = label.clone();
        // let _label2 = _label.borrow_mut();
        root.append_child(&rc_label)
            .append_child(&button)
            .append_child(&input);
    }
}

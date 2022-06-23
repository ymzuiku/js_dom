use crate::depend::*;

pub fn doc() -> Document {
    window().and_then(|win| win.document()).unwrap_throw()
}

// pub fn create_element(tag: &str) -> HtmlElement {
//     let ele: HtmlElement = window()
//         .and_then(|win| win.document())
//         .unwrap_throw()
//         .create_element(tag)
//         .unwrap_throw()
//         .dyn_into()
//         .unwrap_throw();
//     ele
// }

pub fn win() -> Window {
    window().unwrap_throw()
}

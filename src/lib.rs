#![recursion_limit = "1024"]

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// pub use stdweb::js;

// pub use wasm_bindgen::*;
// pub use wasm_bindgen_futures::{future_to_promise, spawn_local};
// pub use web_sys::*;

pub mod depend;
pub mod document;
pub mod element_append;
pub mod element_create;
pub mod element_dyn;
pub mod element_events;
pub mod element_ob;
pub mod element_props;
pub mod element_query;
pub mod error;
pub mod fetch;
pub mod from;
pub mod nanoid;
pub mod object;
pub mod pointer;
pub mod prelude;
pub mod timer;
pub mod win;

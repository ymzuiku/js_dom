// impl<E> From<E> for js_sys::Error
// where
//     E: std::error::Error,
// {
//     #[inline]
//     fn from(value: E) -> js_sys::Error {
//         js_sys::Error::new(&value.to_string())
//     }
// }

// impl<E> From<E> for wasm_bindgen::JsValue
// where
//     E: std::error::Error,
// {
//     #[inline]
//     fn from(value: E) -> Self {
//         wasm_bindgen::JsValue::from_str(&value.to_string())
//     }
// }

// impl<E> From<E> for wasm_bindgen::JsError
// where
//     E: std::error::Error,
// {
//     #[inline]
//     fn from(value: E) -> Self {
//         wasm_bindgen::JsError::from_str(&value.to_string())
//     }
// }

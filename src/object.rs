use crate::depend::*;

#[inline]
pub fn url_search_params(input: &Value) -> UrlSearchParams {
    UrlSearchParams::new_with_str_sequence_sequence(&value_to_js(input)).unwrap_throw()
}

#[inline]
pub fn value_to_js(input: &Value) -> JsValue {
    JsValue::from_serde(input).unwrap_throw()
}

pub fn str_to_js(input: &str) -> JsValue {
    JsValue::from_serde(input).unwrap_throw()
}

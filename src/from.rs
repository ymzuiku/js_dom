use crate::depend::*;
use crate::prelude::*;

impl From<Ele> for JsValue {
    fn from(s: Ele) -> Self {
        JsValue::try_from(&s.ele).unwrap()
    }
}

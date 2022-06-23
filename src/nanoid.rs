pub fn nanoid() -> String {
    format!("id_{}", js_sys::Math::random())
}

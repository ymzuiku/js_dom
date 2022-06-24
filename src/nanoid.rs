pub fn nanoid() -> String {
    format!("n_{}", js_sys::Math::random().to_string().replace('.', "_"))
}
// pub fn nanoid() -> String {
//     format!("i_{}", js_sys::Math::random().to_string().replace('.', "_"))
// }

// pub fn nanoid_to_string(id: f64) -> String {
//     id.to_string().replace('.', "_")
// }

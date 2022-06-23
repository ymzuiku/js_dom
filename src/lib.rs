/// # js_dom
/// js_dom is easy use js and dom library
/// ## Example
/// ```rust
/// let arg = 5;
/// let answer = js_dom::add_one(arg);
/// assert_eq!(6, answer);
///
/// let arg2 = 6;
/// let dog2 = js_dom::add_one(arg2);
/// assert_eq!(8, dog2);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

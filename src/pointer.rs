use std::{cell::RefCell, rc::Rc};

pub fn rc_refcell<T>(t: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(t))
}

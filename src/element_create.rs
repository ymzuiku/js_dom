use std::fmt::Display;

use crate::depend::*;
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Ele {
    pub ele: HtmlElement,
}

pub fn document_query_all(selector: &str) -> Vec<Ele> {
    let document = doc();
    if let Ok(nodelist) = document.query_selector_all(selector) {
        return nodelist_to_eles(nodelist);
    }

    vec![]
}

pub fn document_query(selector: &str) -> Option<Ele> {
    let document = doc();
    if let Some(ele) = document.query_selector(selector).unwrap_throw() {
        return Some(from_element(ele));
    }

    None
}

pub fn nodelist_to_eles(nodelist: NodeList) -> Vec<Ele> {
    let l = nodelist.length();
    let mut list = vec![];

    for i in 0..l {
        if let Some(node) = nodelist.get(i) {
            list.push(from_node(node));
        }
    }

    list
}

// impl Ele {
//     #[inline]

// }

pub fn from_id<T: Display>(id: T) -> Ele {
    get_element_by_id(id).unwrap_throw()
}

pub fn from_element(target: Element) -> Ele {
    let ele: HtmlElement = target.dyn_into().unwrap_throw();

    Ele { ele }
}

pub fn create_element(tag: &str) -> Ele {
    let ele = doc().create_element(intern(tag)).unwrap_throw();
    from_element(ele)
}
pub fn from_node(node: Node) -> Ele {
    let v = JsValue::from(node);
    let ele: Element = v.dyn_into().unwrap_throw();

    from_element(ele)
}
pub fn from_event(e: Event) -> Ele {
    let ele: Element = e.target().unwrap_throw().dyn_into().unwrap_throw();

    from_element(ele)
}
pub fn get_value(e: Event) -> String {
    from_event(e).get_value()
}
pub fn from_event_current(e: Event) -> Ele {
    let ele: Element = e.current_target().unwrap_throw().dyn_into().unwrap_throw();

    from_element(ele)
}

pub fn body() -> Ele {
    let body = doc().body().unwrap_throw();
    let ele: Element = body.dyn_into().unwrap_throw();

    from_element(ele)
}

pub fn div() -> Ele {
    create_element("div")
}

pub fn svg() -> Ele {
    create_element("svg")
}

pub fn input() -> Ele {
    create_element("input")
}

pub fn textarea() -> Ele {
    create_element("textarea")
}

pub fn header() -> Ele {
    create_element("header")
}

pub fn strong() -> Ele {
    create_element("strong")
}
pub fn footer() -> Ele {
    create_element("footer")
}

pub fn meta() -> Ele {
    create_element("meta")
}

pub fn style_element() -> Ele {
    create_element("style")
}

pub fn script_element() -> Ele {
    create_element("script")
}

pub fn template_element() -> Ele {
    create_element("template")
}

pub fn link() -> Ele {
    create_element("link")
}

pub fn iframe() -> Ele {
    create_element("iframe")
}

pub fn p() -> Ele {
    create_element("p")
}

pub fn span() -> Ele {
    create_element("span")
}
pub fn i() -> Ele {
    create_element("i")
}

pub fn kbd() -> Ele {
    create_element("kbd")
}

pub fn button() -> Ele {
    create_element("button")
}

pub fn nav() -> Ele {
    create_element("nav")
}

pub fn section() -> Ele {
    create_element("section")
}

pub fn label() -> Ele {
    create_element("label")
}

pub fn form() -> Ele {
    create_element("form")
}

pub fn aside() -> Ele {
    create_element("aside")
}

pub fn article() -> Ele {
    create_element("article")
}

pub fn menu() -> Ele {
    create_element("menu")
}

pub fn mark() -> Ele {
    create_element("mark")
}

pub fn dd() -> Ele {
    create_element("dd")
}

pub fn dt() -> Ele {
    create_element("dt")
}

pub fn li() -> Ele {
    create_element("li")
}

pub fn ul() -> Ele {
    create_element("ul")
}

pub fn ol() -> Ele {
    create_element("ol")
}

pub fn code() -> Ele {
    create_element("code")
}

pub fn h1() -> Ele {
    create_element("h1")
}

pub fn h2() -> Ele {
    create_element("h2")
}

pub fn h3() -> Ele {
    create_element("h3")
}
pub fn h4() -> Ele {
    create_element("h4")
}
pub fn h5() -> Ele {
    create_element("h5")
}
pub fn h6() -> Ele {
    create_element("h6")
}
pub fn table() -> Ele {
    create_element("table")
}
pub fn tr() -> Ele {
    create_element("tr")
}
pub fn th() -> Ele {
    create_element("th")
}
pub fn td() -> Ele {
    create_element("td")
}

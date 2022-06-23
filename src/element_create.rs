use crate::depend::*;
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Ele {
    pub ele: HtmlElement,
}

// impl Default for Ele{
//     fn default() -> Self {
//         id: nanoid!(),
//         Ele::div()
//     }
// }

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
        return Some(Ele::new(ele));
    }

    None
}

pub fn nodelist_to_eles(nodelist: NodeList) -> Vec<Ele> {
    let l = nodelist.length();
    let mut list = vec![];

    for i in 0..l {
        if let Some(node) = nodelist.get(i) {
            list.push(Ele::new_node(node));
        }
    }

    list
}

impl Ele {
    #[inline]
    pub fn new(target: Element) -> Self {
        let ele: HtmlElement = target.dyn_into().unwrap_throw();
        // match ele.get_attribute("id") {
        //     Some(_) => {}
        //     None => {
        //         ele.set_id(&nanoid());
        //     }
        // };

        Self { ele }
    }
    pub fn new_tag(tag: &str) -> Self {
        let ele = doc().create_element(intern(tag)).unwrap_throw();
        Ele::new(ele)
    }
    pub fn new_node(node: Node) -> Self {
        let v = JsValue::from(node);
        let ele: Element = v.dyn_into().unwrap_throw();

        Ele::new(ele)
    }
    pub fn get_target(e: Event) -> Self {
        let ele: Element = e.target().unwrap_throw().dyn_into().unwrap_throw();

        Ele::new(ele)
    }
    pub fn get_current_target(e: Event) -> Self {
        let ele: Element = e.current_target().unwrap_throw().dyn_into().unwrap_throw();

        Ele::new(ele)
    }

    pub fn body() -> Self {
        let body = doc().body().unwrap_throw();
        let ele: Element = body.dyn_into().unwrap_throw();

        Ele::new(ele)
    }

    pub fn div() -> Self {
        Ele::new_tag("div")
    }

    pub fn svg() -> Self {
        Ele::new_tag("svg")
    }

    pub fn input() -> Self {
        Ele::new_tag("input")
    }

    pub fn textarea() -> Self {
        Ele::new_tag("textarea")
    }

    pub fn header() -> Self {
        Ele::new_tag("header")
    }

    pub fn strong() -> Self {
        Ele::new_tag("strong")
    }
    pub fn footer() -> Self {
        Ele::new_tag("footer")
    }

    pub fn meta() -> Self {
        Ele::new_tag("meta")
    }

    pub fn style_element() -> Self {
        Ele::new_tag("style")
    }

    pub fn script_element() -> Self {
        Ele::new_tag("script")
    }

    pub fn template_element() -> Self {
        Ele::new_tag("template")
    }

    pub fn link() -> Self {
        Ele::new_tag("link")
    }

    pub fn iframe() -> Self {
        Ele::new_tag("iframe")
    }

    pub fn p() -> Self {
        Ele::new_tag("p")
    }

    pub fn span() -> Self {
        Ele::new_tag("span")
    }
    pub fn i() -> Self {
        Ele::new_tag("i")
    }

    pub fn kbd() -> Self {
        Ele::new_tag("kbd")
    }

    pub fn button() -> Self {
        Ele::new_tag("button")
    }

    pub fn nav() -> Self {
        Ele::new_tag("nav")
    }

    pub fn section() -> Self {
        Ele::new_tag("section")
    }

    pub fn label() -> Self {
        Ele::new_tag("label")
    }

    pub fn form() -> Self {
        Ele::new_tag("form")
    }

    pub fn aside() -> Self {
        Ele::new_tag("aside")
    }

    pub fn article() -> Self {
        Ele::new_tag("article")
    }

    pub fn menu() -> Self {
        Ele::new_tag("menu")
    }

    pub fn mark() -> Self {
        Ele::new_tag("mark")
    }

    pub fn dd() -> Self {
        Ele::new_tag("dd")
    }

    pub fn dt() -> Self {
        Ele::new_tag("dt")
    }

    pub fn li() -> Self {
        Ele::new_tag("li")
    }

    pub fn ul() -> Self {
        Ele::new_tag("ul")
    }

    pub fn ol() -> Self {
        Ele::new_tag("ol")
    }

    pub fn code() -> Self {
        Ele::new_tag("code")
    }

    pub fn h1() -> Self {
        Ele::new_tag("h1")
    }

    pub fn h2() -> Self {
        Ele::new_tag("h2")
    }

    pub fn h3() -> Self {
        Ele::new_tag("h3")
    }
    pub fn h4() -> Self {
        Ele::new_tag("h4")
    }
    pub fn h5() -> Self {
        Ele::new_tag("h5")
    }
    pub fn h6() -> Self {
        Ele::new_tag("h6")
    }
    pub fn table() -> Self {
        Ele::new_tag("table")
    }
    pub fn tr() -> Self {
        Ele::new_tag("tr")
    }
    pub fn th() -> Self {
        Ele::new_tag("th")
    }
    pub fn td() -> Self {
        Ele::new_tag("td")
    }
}

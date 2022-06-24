use crate::depend::*;
use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Fetch {
    pub url: String,
    pub method: String,
    pub body: Value,
    pub headers: Option<Value>,
}

pub async fn fetch<'a>(input: &Fetch) -> Result<Value, JsValue> {
    let mut opts = RequestInit::new();
    opts.method(&input.method);

    let body_text = input.body.to_string();
    let body = JsValue::from_str(&body_text);
    let mut url = input.url.clone();

    if input.method != "GET" {
        opts.body(Some(&body));
    } else {
        let search = url_search_params(&input.body)
            .to_string()
            .as_string()
            .unwrap_throw();
        url = url.add("?").add(&search);
    }

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request.headers().set("Content-Type", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap_throw();

    let json = JsFuture::from(resp.json()?).await?;

    let out: Value = JsValue::into_serde(&json).unwrap_throw();

    Ok(out)
}

#[allow(dead_code)]
async fn fetch_example() {
    match fetch(&Fetch {
        url: "/v1/example".into(),
        method: "POST".into(),
        body: json!({"name":"aaaa", "age":10_i32}),
        headers: None,
    })
    .await
    {
        Ok(res) => {
            gloo_log!("__debug__", &format!("{:?}", res));
        }
        Err(err) => {
            gloo_log!("__debug__err", &format!("{:?}", err));
        }
    }
}

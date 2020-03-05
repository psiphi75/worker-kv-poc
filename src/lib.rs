extern crate cfg_if;
extern crate wasm_bindgen;
#[macro_use]
extern crate serde_derive;

mod kv_error;
#[macro_use]
mod utils;
mod workers_kv;

use cfg_if::cfg_if;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys;

use kv_error::KvError;
use workers_kv::{KvResponse, WorkersKv};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[derive(Serialize)]
pub struct Response {
    status: u16,
    headers: HashMap<String, String>,
    body: String,
}

#[derive(Serialize)]
pub struct JsonBody {
    method: String,
    message: String,
}

#[derive(Deserialize, Debug)]
pub struct Request {
    method: String,
    headers: HashMap<String, String>,
    url: String,
    body: Option<String>, // FIXME work with binary data
    kv_account_id: String,
    kv_namespace_id: String,
    kv_auth_email: String,
    kv_auth_key: String,
}

#[wasm_bindgen]
pub async fn respond_wrapper(req: JsValue) -> Result<JsValue, JsValue> {
    let req = req.into_serde().map_err(|e| e.to_string())?;
    let res = respond(req).await.map_err(|e| e.to_string())?;
    let res = JsValue::from_serde(&res).map_err(|e| e.to_string())?;
    Ok(res)
}

fn html_response(status: u16, body: String) -> Response {
    let mut headers = HashMap::new();
    headers.insert(
        "Content-Type".to_string(),
        "text/json; charset=utf-8".to_string(),
    );

    Response {
        status,
        headers,
        body: format!("{}", body),
    }
}

fn handle_response(res: Result<KvResponse, KvError>) -> Response {
    if res.is_err() {
        return html_response(500, format!("Unhandled error"));
    }

    let res_json = res.unwrap();

    if !res_json.success {
        html_response(404, format!("There was an error: {:?}", res_json.errors[0]))
    } else {
        html_response(200, format!("{:?}", res_json))
    }
}

fn get_method_key(req: Request) -> (String, String, Option<String>) {
    let method = req.method;
    let url: url::Url = req.url.parse().unwrap();
    let split_url = url.path().split("/");
    let v_url: Vec<&str> = split_url.collect();
    let key = v_url[1].to_string();

    log!("req.body: {:?}", req.body);
    let value = req.body;

    (method, key, value)
}

async fn respond(req: Request) -> Result<Response, Box<dyn std::error::Error>> {
    let worker = WorkersKv::new(
        req.kv_account_id.clone(),
        req.kv_namespace_id.clone(),
        req.kv_auth_email.clone(),
        req.kv_auth_key.clone(),
    );
    let (method, key, value) = get_method_key(req);

    log!("key: '{}', method: '{}', value: '{:?}'", key, method, value);

    if key.is_empty() {
        return Ok(html_response(404, format!("Invalid request")));
    }

    Ok(match &method as &str {
        "GET" => html_response(200, worker.get(key).await.unwrap()),
        "PUT" => handle_response(worker.add(key, value.unwrap()).await),
        method => html_response(404, format!("Invalid method: {}", method)),
    })
}

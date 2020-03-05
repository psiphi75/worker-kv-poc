use crate::kv_error::KvError;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{JsFuture};
use js_sys::{ Promise};

#[wasm_bindgen]
extern "C" {
    type WORKER_KV;

    #[wasm_bindgen(static_method_of = WORKER_KV)]
    fn get(key: &str) -> Promise;

    #[wasm_bindgen(static_method_of = WORKER_KV)]
    fn put(key: &str, val: &str) -> Promise;

    #[wasm_bindgen(static_method_of = WORKER_KV)]
    fn delete(key: &str) -> Promise;
}


#[derive(Debug, Serialize, Deserialize)]
pub struct KvResponse {
    result: Option<String>,
    pub success: bool,
    pub errors: Vec<KvResponseErrorMsg>,
    messages: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KvResponseErrorMsg {
    code: i32,
    message: String,
}

pub struct WorkersKv {
    base_url: String,
    auth_email: String,
    auth_key: String,
}

impl WorkersKv {
    pub fn new(
        account_id: String,
        namespace_id: String,
        auth_email: String,
        auth_key: String,
    ) -> Self {
        Self {
            base_url: format!(
                "https://api.cloudflare.com/client/v4/accounts/{}/storage/kv/namespaces/{}/values",
                account_id, namespace_id
            ),
            auth_email,
            auth_key,
        }
    }

    fn set_headers(&self, request: &web_sys::Request) -> Result<(), KvError> {
        request.headers().set("User-Agent", "surf")?;
        request.headers().set("X-Auth-Email", &self.auth_email)?;
        request.headers().set("X-Auth-Key", &self.auth_key)?;
        Ok(())
    }

    pub async fn add(&self, key: String, value: String) -> Result<KvResponse, KvError> {
        let mut opts = web_sys::RequestInit::new();
        opts.method("PUT");
        let body = JsValue::from_str(&value); // web-sys should really require mut here...
        opts.body(Some(&body));

        let kv_url = format!("{}/{}", self.base_url, key);
        let request = web_sys::Request::new_with_str_and_init(&kv_url, &opts)?;
        self.set_headers(&request)?;

        let window = worker_global_scope().ok_or(KvError::NoWindow)?;

        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

        let resp: web_sys::Response = resp_value.dyn_into()?;

        let json = JsFuture::from(resp.json()?).await?;

        let response: KvResponse = json.into_serde()?;

        Ok(response)
    }

    pub async fn get(&self, key: String) -> Result<String, KvError> {
        let mut opts = web_sys::RequestInit::new();
        opts.method("GET");
        let kv_url = format!("{}/{}", self.base_url, key);
        let request = web_sys::Request::new_with_str_and_init(&kv_url, &opts)?;
        self.set_headers(&request)?;

        let window = worker_global_scope().ok_or(KvError::NoWindow)?;

        let resp_value: JsValue = JsFuture::from(window.fetch_with_request(&request)).await?;

        let resp: web_sys::Response = resp_value.dyn_into()?;

        let s = JsFuture::from(resp.text()?).await?;

        Ok(s.as_string().unwrap())
    }
}

fn worker_global_scope() -> Option<web_sys::ServiceWorkerGlobalScope> {
    js_sys::global()
        .dyn_into::<web_sys::ServiceWorkerGlobalScope>()
        .ok()
}

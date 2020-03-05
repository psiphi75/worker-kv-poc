use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum KvError {
    IO(std::io::Error),
    SerdeUrl(serde_urlencoded::ser::Error),
    Js(JsValue),
    SerdeJson(serde_json::error::Error),
    NoWindow,
}

impl From<std::io::Error> for KvError {
    fn from(e: std::io::Error) -> Self {
        KvError::IO(e)
    }
}

impl From<serde_urlencoded::ser::Error> for KvError {
    fn from(e: serde_urlencoded::ser::Error) -> Self {
        KvError::SerdeUrl(e)
    }
}

impl From<serde_json::error::Error> for KvError {
    fn from(e: serde_json::error::Error) -> Self {
        KvError::SerdeJson(e)
    }
}

impl From<JsValue> for KvError {
    fn from(e: JsValue) -> Self {
        KvError::Js(e)
    }
}

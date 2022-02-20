use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

pub const COOKIE_USER_NAME: &'static str = "name";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateUserIdRequest {
    pub name: String,
}

impl From<&CreateUserIdRequest> for JsValue {
    fn from(req: &CreateUserIdRequest) -> Self {
        JsValue::from_serde(req).unwrap()
    }
}

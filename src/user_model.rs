use serde::{Serialize, Deserialize};
use wasm_bindgen::JsValue;

use crate::game_model::CreateGameRequest;

pub const COOKIE_USER_ID: &'static str = "user_id";
pub const COOKIE_USER_NAME: &'static str = "name";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateUserIdRequest {
    pub name: String
}

impl From<&CreateUserIdRequest> for JsValue {
    fn from(req: &CreateUserIdRequest) -> Self {
        JsValue::from_serde(req).unwrap()
    }
}
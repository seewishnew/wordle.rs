use wasm_bindgen::JsCast;
#[allow(unused, dead_code)]
use yew::{classes, html, Component, Context, Html, Properties};

use crate::user_model::COOKIE_USER_NAME;
mod answer_input;
mod charcell;
mod game_model;
mod keyboard;
mod leaderboard;
mod menu;
mod register;
mod routes;
mod user_model;
mod wordle;
mod snackbar;

use routes::*;

fn check_user_set() -> Option<Vec<char>> {
    let name_cookie = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .dyn_into::<web_sys::HtmlDocument>()
        .unwrap()
        .cookie()
        .unwrap();
    let user_name = name_cookie.split(";").find_map(|s| {
        if s.starts_with(COOKIE_USER_NAME) {
            s.split("=").nth(1)
        } else {
            None
        }
    });
    if user_name.is_none() || user_name.as_ref().unwrap().is_empty() {
        None
    } else {
        log::info!("Found user_name cookie with name: {user_name:?}");
        log::info!("Not re-registering user!");
        Some(user_name.unwrap().chars().collect())
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

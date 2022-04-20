use gloo::console::error;
use reqwasm::http::Request;
use web_sys::RequestCredentials;
#[allow(unused, dead_code)]
use yew::{classes, html, Component, Context, Html, Properties};

mod answer_input;
mod charcell;
mod game_model;
mod keyboard;
mod leaderboard;
mod menu;
mod register;
mod routes;
mod snackbar;
mod user_model;
mod wordle;

use routes::*;

async fn check_user_set() -> bool {
    Request::get("/api/v1/user_id/verify")
        .credentials(RequestCredentials::Include)
        .send()
        .await
        .map_or_else(
            |error| {
                error!(format!("An error occurred verifying user id: {error:?}"));
                false
            },
            |resp| {
                if resp.ok() {
                    true
                } else if resp.status() == 401 {
                    false
                } else {
                    error!("An error occurred on the server end: {error:?}");
                    false
                }
            },
        )
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

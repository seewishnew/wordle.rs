use crate::{
    answer_input::AnswerInput, leaderboard::Leaderboard, menu::Menu, register::Register,
    wordle::Wordle,
};
use yew::{function_component, html, Html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/register")]
    Register,
    #[at("/")]
    Menu,
    #[at("/create")]
    Create,
    #[at("/manage/:game_id")]
    Manage { game_id: String },
    #[at("/play/:game_id")]
    Play { game_id: String },
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Register => html! {<Register></Register>},
        Route::Menu => html! {<Menu></Menu>},
        Route::Create => html! {<AnswerInput></AnswerInput>},
        Route::Manage { game_id } => {
            html! {<Leaderboard game_id={ game_id.clone() }></Leaderboard>}
        }
        Route::Play { game_id } => html! {<Wordle game_id={ game_id.clone() }></Wordle>},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

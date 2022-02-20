use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, RequestCredentials};
#[allow(unused, dead_code)]
use yew::{classes, html, Component, Context, Html, Properties};
use yew::{events::Event, Callback};
use yew_router::{history::History, prelude::RouterScopeExt};

use crate::{check_user_set, Route};
pub enum MenuMsg {
    Input(String),
    Submit,
    SubmitResponse(Result<(), reqwasm::Error>),
}

pub struct Menu {
    game_id: String,
}

impl Component for Menu {
    type Message = MenuMsg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        if check_user_set().is_none() {
            ctx.link().history().unwrap().push(Route::Register);
        }

        Self {
            game_id: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Input(s) => {
                self.game_id = s;
                false
            }
            Self::Message::Submit => {
                if self.game_id.len() == 24 {
                    let url = format!("/api/v1/game/{}/register", self.game_id);
                    ctx.link().send_future(async move {
                        Self::Message::SubmitResponse(
                            match Request::post(&url)
                                .header("Content-Type", "application/json")
                                .credentials(RequestCredentials::Include)
                                .send()
                                .await
                            {
                                Ok(_) => Ok(()),
                                Err(error) => Err(error),
                            },
                        )
                    })
                }
                true
            }
            Self::Message::SubmitResponse(Ok(_)) => {
                ctx.link().history().unwrap().push(Route::Play {
                    game_id: self.game_id.clone(),
                });
                false
            }
            Self::Message::SubmitResponse(Err(error)) => {
                log::error!("Received error: {error:?}");
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let history = ctx.link().history().clone().unwrap();
        let on_create_click = Callback::once(move |_| history.push(Route::Create));
        let on_cautious_change = ctx.link().batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| Self::Message::Input(input.value()))
        });
        let on_play_click = ctx.link().callback(|_| Self::Message::Submit);
        html! {
            <div class={classes!("w-full", "h-full", "grid", "place-content-center")}>
                <div class={classes!("grid", "w-80", "md:w-100", "lg:w-150",  "h-full", "gap-y-5", "text-white", "justify-items-center", "content-center")}>
                    <button onclick={on_create_click} class={classes!("border", "border-solid", "border-white")}>{"Create New Game"}</button>
                    <div class={classes!("grid", "gap-y-3")}>
                        <input onchange={on_cautious_change} class={classes!("text-black")} type="text" placeholder="Game ID"/>
                        <button onclick={on_play_click} class={classes!("border", "border-solid", "border-white")}>{"Play"}</button>
                    </div>
                </div>
            </div>
        }
    }
}

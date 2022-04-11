use reqwasm::http::Request;
use web_sys::RequestCredentials;
#[allow(unused, dead_code)]
use yew::{classes, html, Component, Context, Html, Properties};
use yew_router::{history::History, prelude::RouterScopeExt};

use crate::{
    check_user_set,
    keyboard::{Keyboard, KeyboardMsg},
    snackbar::Snackbar,
    user_model::CreateUserIdRequest,
    Route,
};

const PROMPT: &'static str = "Enter name";
pub enum RegisterMsg {
    KeyboardInput(KeyboardMsg),
    RegisterUserResponse(Result<(), reqwasm::Error>),
}

#[derive(PartialEq, Properties)]
pub struct RegisterProps;

pub struct Register {
    user_name: Vec<char>,
    toast_msg: Option<String>,
}

impl Component for Register {
    type Message = RegisterMsg;

    type Properties = RegisterProps;

    fn create(ctx: &Context<Self>) -> Self {
        if check_user_set().is_some() {
            ctx.link().history().unwrap().push(Route::Menu);
        }
        log::info!("Setting toast msg");
        Self {
            user_name: PROMPT.chars().collect(),
            toast_msg: Some("Please register to begin".to_owned()),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RegisterMsg::KeyboardInput(msg) => {
                self.toast_msg = None;
                self.keydown_handler(ctx, msg)
            }
            RegisterMsg::RegisterUserResponse(Ok(_)) => {
                ctx.link().history().unwrap().push(Route::Menu);
                false
            }
            RegisterMsg::RegisterUserResponse(Err(error)) => {
                log::error!("Error registering user: {error:?}");
                self.toast_msg = Some("Error registering user".to_owned());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let register_classes = vec![
            "h-80",
            "w-full",
            "grid",
            "grid-rows-2",
            "gap-y-1",
            "text-white",
        ];

        let onkeyclick = ctx.link().callback(|e: KeyboardMsg| {
            log::info!("Received KeyboardMsg: {e}");
            RegisterMsg::KeyboardInput(e)
        });

        html! {
            <div class={classes!("w-full", "h-full", "grid", "place-content-center")}>
                <div class={classes!("grid", "w-80", "md:w-100", "lg:w-150",  "h-full", "gap-y-5", "justify-items-center", "content-center")}>
                    <div class={classes!(register_classes)}>
                        <div class={classes!("w-full", "flex", "justify-items-center")}>{"Welcome!"}</div>
                        <div class={classes!("w-full", "flex", "justify-items-center", "overflow-x-auto")}>{self.user_name.iter().collect::<String>()}</div>
                    </div>
                    <Keyboard callback={onkeyclick}></Keyboard>
                    <Snackbar message={self.toast_msg.as_ref().cloned().unwrap_or(String::new())} display={self.toast_msg.is_some()}></Snackbar>
                </div>
            </div>
        }
    }
}

impl Register {
    fn keydown_handler(&mut self, ctx: &Context<Self>, msg: KeyboardMsg) -> bool {
        let user_name_str = self.user_name.iter().collect::<String>();
        if user_name_str == PROMPT {
            self.user_name.clear();
        }

        match msg {
            KeyboardMsg::Backspace => {
                let user_name_len = self.user_name.len();
                if user_name_len <= 1 {
                    self.user_name = PROMPT.chars().collect();
                } else {
                    self.user_name.pop();
                }
            }
            KeyboardMsg::Enter => {
                if !self.user_name.is_empty() {
                    let origin = web_sys::window().unwrap().location().origin().unwrap();
                    log::info!("origin: {origin}");
                    let mut api_url = url::Url::parse(&origin).unwrap();
                    api_url.set_path("/api/v1");
                    log::info!("api_url: {api_url:?}");
                    log::info!("Registering user name: {user_name_str}");
                    ctx.link().send_future(async {
                        let user_name = CreateUserIdRequest {
                            name: user_name_str,
                        };
                        log::info!("user_name: {user_name:?}");
                        RegisterMsg::RegisterUserResponse(
                            match Request::post("/api/v1/user_id")
                                .header("Content-Type", "application/json")
                                .body(serde_json::to_string(&user_name).unwrap())
                                .credentials(RequestCredentials::Include)
                                .send()
                                .await
                            {
                                Ok(_resp) => Ok(()),
                                Err(error) => Err(error),
                            },
                        )
                    });
                } else {
                    self.user_name = PROMPT.chars().collect();
                    return false;
                }
            }
            ch => {
                if self.user_name.len() <= 20 {
                    let ch: &str = ch.into();
                    let ch = ch.chars().next().unwrap();
                    self.user_name.push(ch);
                }
            }
        }

        true
    }
}

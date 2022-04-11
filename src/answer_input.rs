use reqwasm::http::Request;
use web_sys::RequestCredentials;
#[allow(unused, dead_code)]
use yew::{classes, html, Component, Context, Html, Properties};
use yew_router::{history::History, prelude::RouterScopeExt};

use crate::{
    charcell::{CharCellState, Correctness, FilledState},
    check_user_set,
    game_model::{CreateGameRequest, CreateGameResponse},
    keyboard::{Keyboard, KeyboardMsg},
    snackbar::Snackbar,
    wordle::Word,
    Route,
};

pub struct AnswerInput {
    answer: Vec<CharCellState>,
    cell_i: usize,
    submitted: bool,
    animate: bool,
    toast_msg: Option<String>,
}

pub enum AnswerInputResponse {
    CreateGame(Result<CreateGameResponse, reqwasm::Error>),
}
pub enum AnswerInputMsg {
    KeyboardInput(KeyboardMsg),
    ApiResponse(AnswerInputResponse),
}

impl Component for AnswerInput {
    type Message = AnswerInputMsg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        if check_user_set().is_none() {
            ctx.link().history().unwrap().push(Route::Register);
        }

        Self {
            answer: vec![CharCellState::Empty; 5],
            cell_i: 0,
            submitted: false,
            animate: false,
            toast_msg: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AnswerInputMsg::KeyboardInput(e) => self.keydown_handler(ctx, e),
            AnswerInputMsg::ApiResponse(AnswerInputResponse::CreateGame(Ok(resp))) => {
                log::info!("Created game with ID: {:?}", resp);
                ctx.link().history().unwrap().push(Route::Manage {
                    game_id: resp.game_id,
                });
                true
            }
            AnswerInputMsg::ApiResponse(AnswerInputResponse::CreateGame(Err(error))) => {
                log::error!("Could not create a new game: {error:?}");
                self.toast_msg = Some("An error occurred".to_string());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeyclick = ctx.link().callback(|e: KeyboardMsg| {
            log::info!("Received KeyboardMsg: {e}");
            AnswerInputMsg::KeyboardInput(e)
        });
        let answer_classes = vec![
            "h-80",
            "w-full",
            "grid",
            "grid-rows-1",
            "gap-y-1",
            "text-white",
        ];

        html! {
            <div class={classes!("w-full", "h-full", "grid", "place-content-center")}>
                <div class={classes!("grid", "w-80", "md:w-100", "lg:w-150",  "h-full", "gap-y-5", "justify-items-center", "content-center")}>
                    <div class={classes!(answer_classes)}>
                        <Word text={self.answer.clone()} animate={self.animate}></Word>
                    </div>
                    <Keyboard callback={onkeyclick}></Keyboard>
                    <Snackbar message={self.toast_msg.as_ref().cloned().unwrap_or(String::new())} display={self.toast_msg.is_some()}></Snackbar>
                </div>
            </div>
        }
    }
}

impl AnswerInput {
    fn keydown_handler(&mut self, ctx: &Context<Self>, e: KeyboardMsg) -> bool {
        if self.submitted {
            return false;
        }
        self.animate = false;
        match e {
            KeyboardMsg::Backspace => {
                if self.cell_i > 0 {
                    self.cell_i -= 1;
                    self.answer[self.cell_i] = CharCellState::Empty;
                } else {
                    return false;
                }
            }
            KeyboardMsg::Enter => {
                if self.cell_i == 5 {
                    self.submitted = true;
                    let answer: String = self
                        .answer
                        .iter_mut()
                        .map(|css| {
                            if let CharCellState::Filled(FilledState { correctness, ch }) = css {
                                *correctness = Correctness::Correct;
                                *ch
                            } else {
                                unreachable!()
                            }
                        })
                        .collect();
                    log::info!("Set answer to {:?}!", answer);
                    ctx.link().send_future(async {
                        let game_req = CreateGameRequest { answer: answer };
                        AnswerInputMsg::ApiResponse(AnswerInputResponse::CreateGame(
                            match Request::post("/api/v1/create")
                                .header("Content-Type", "application/json")
                                .body(serde_json::to_string(&game_req).unwrap())
                                .credentials(RequestCredentials::Include)
                                .send()
                                .await
                            {
                                Ok(resp) => resp.json::<CreateGameResponse>().await,
                                Err(error) => Err(error),
                            },
                        ))
                    });
                }
                self.animate = true;
            }

            ch => {
                let ch: &str = ch.into();
                if ch.len() == 1 && self.cell_i <= 4 {
                    self.answer[self.cell_i] = CharCellState::Filled(FilledState {
                        ch: ch.chars().next().unwrap().to_ascii_uppercase(),
                        correctness: Correctness::Guess,
                    });
                    if self.cell_i <= 4 {
                        self.cell_i += 1;
                    }
                } else {
                    return false;
                }
            }
        }

        return true;
    }
}

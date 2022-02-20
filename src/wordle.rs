use crate::game_model::PlayRequest;
use crate::{
    charcell::*,
    check_user_set,
    game_model::PlayResponse,
    keyboard::{Keyboard, KeyboardMsg},
    Route,
};
use reqwasm::http::Request;
use web_sys::RequestCredentials;
#[allow(unused, dead_code)]
use yew::{classes, html, Component, Context, Html, Properties};
use yew_router::prelude::*;
#[derive(Clone, PartialEq, Properties)]
pub struct WordProps {
    pub text: Vec<CharCellState>,
    #[prop_or(false)]
    pub animate: bool,
}

pub struct Word;

impl Component for Word {
    type Message = ();

    type Properties = WordProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties { text, animate } = ctx.props();
        html! {
            <div class={classes!("grid", "grid-cols-5", "gap-x-1", "justify-items-center", "content-evenly")}>
            {
                text.iter().enumerate().map(|(i, ccs)| {
                    html!{
                        <CharCell state={ccs.clone()} animate={(*animate, i as u16*500)}></CharCell>
                    }
                }).collect::<Html>()
            }
            </div>
        }
    }
}

pub enum WordleResponse {
    PlayGame(Result<PlayResponse, reqwasm::Error>),
}
pub enum WordleMsg {
    KeyboardInput(KeyboardMsg),
    ApiResponse(WordleResponse),
}

#[derive(PartialEq, Properties)]
pub struct WordleProps {
    pub game_id: String,
}

pub struct Wordle {
    animate: bool,
    game_over: bool,
    game_id: String,
    cell_i: usize,
    word_i: usize,
    state: Vec<Vec<CharCellState>>,
    correctness_map: [Correctness; 28],
}

impl Component for Wordle {
    type Message = WordleMsg;

    type Properties = WordleProps;

    fn create(ctx: &Context<Self>) -> Self {
        if check_user_set().is_none() {
            ctx.link().history().unwrap().push(Route::Register);
        }

        let Self::Properties { game_id } = ctx.props();

        Self {
            animate: false,
            game_over: false,
            game_id: game_id.into(),
            cell_i: 0,
            word_i: 0,
            state: vec![vec![CharCellState::Empty; 5]; 6],
            correctness_map: [Correctness::Guess; 28],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::KeyboardInput(msg) => self.keydown_handler(ctx, msg),
            Self::Message::ApiResponse(WordleResponse::PlayGame(Ok(resp))) => {
                log::info!("Play submitted to leaderboard");
                log::info!("Received response: {resp:?}");
                self.game_over = self.game_over || resp.game_over;
                resp.guess
                    .into_iter()
                    .enumerate()
                    .for_each(|(i, (ch, correctness))| {
                        self.state[self.word_i][i] = CharCellState::Filled(FilledState {
                            ch,
                            correctness: Correctness::from(correctness),
                        });
                    });
                self.cell_i = 0;
                self.word_i += 1;
                if self.word_i == 6 {
                    self.game_over = true;
                }
                self.animate = true;
                true
            }
            _ => {
                log::error!("Something went wrong!");
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeyclick = ctx.link().callback(|e: KeyboardMsg| {
            log::info!("Received KeyboardMsg: {e}");
            Self::Message::KeyboardInput(e)
        });

        html! {
            <div class={classes!("w-full", "h-full", "grid", "place-content-center")}>
                <div class={classes!("grid", "w-80", "md:w-100", "lg:w-150",  "h-full", "gap-y-5", "justify-items-center", "content-center")}>
                    <div class={classes!("h-80", "w-full", "grid", "grid-rows-6", "gap-y-1", "text-white")}>
                        {
                            self.state.iter().enumerate().map(|(i, text)| {
                                // log::info!("Rendering word #{i}, word_i: {}, animation status: {}", self.word_i, i+1 == self.word_i && self.animate);
                                html!{
                                    <Word text={text.clone()} animate={i+1 == self.word_i && self.animate}></Word>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    <Keyboard callback={onkeyclick} correctness_map={self.correctness_map}></Keyboard>
                </div>
            </div>

        }
    }
}

impl Wordle {
    fn keydown_handler(&mut self, ctx: &Context<Self>, e: KeyboardMsg) -> bool {
        if self.game_over {
            return false;
        }
        self.animate = false;
        match e {
            KeyboardMsg::Backspace => {
                if self.cell_i > 0 {
                    self.cell_i -= 1;
                    self.state[self.word_i][self.cell_i] = CharCellState::Empty;
                } else {
                    return false;
                }
            }
            KeyboardMsg::Enter => {
                if self.cell_i == 5 {
                    let guess: Vec<char> = self.state[self.word_i]
                        .iter()
                        .map(|css| match css {
                            &CharCellState::Filled(FilledState { ch, .. }) => ch,
                            _ => unreachable!(),
                        })
                        .collect();
                    let url = format!("/api/v1/game/{}/play", self.game_id);
                    ctx.link().send_future(async move {
                        WordleMsg::ApiResponse(WordleResponse::PlayGame(
                            match Request::post(&url)
                                .header("Content-Type", "application/json")
                                .credentials(RequestCredentials::Include)
                                .body(serde_json::to_string(&PlayRequest { guess }).unwrap())
                                .send()
                                .await
                            {
                                Ok(resp) => resp.json::<PlayResponse>().await,
                                Err(error) => Err(error),
                            },
                        ))
                    });
                    return false;
                } else {
                    return false;
                }
            }
            ch => {
                let ch: &str = ch.into();
                if ch.len() == 1 && self.cell_i <= 4 {
                    self.state[self.word_i][self.cell_i] = CharCellState::Filled(FilledState {
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

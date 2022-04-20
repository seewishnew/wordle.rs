use crate::game_model::{self, GetStateResponse, PlayRequest};
use crate::{
    charcell::*,
    check_user_set,
    game_model::PlayResponse,
    keyboard::{Keyboard, KeyboardMsg},
    snackbar::Snackbar,
    Route,
};
use gloo::timers::callback::Timeout;
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
    GetState(Result<GetStateResponse, reqwasm::Error>),
    PlayGame(Result<PlayResponse, reqwasm::Error>),
}
pub enum WordleMsg {
    KeyboardInput(KeyboardMsg),
    VerifyUserResponse(bool),
    ApiResponse(WordleResponse),
}

#[derive(PartialEq, Properties)]
pub struct WordleProps {
    pub game_id: String,
}

pub struct Wordle {
    animate: bool,
    loading: bool,
    game_over: bool,
    game_id: String,
    cell_i: usize,
    word_i: usize,
    verification_pending: bool,
    state: Vec<Vec<CharCellState>>,
    correctness_map: [Correctness; 28],
    toast_msg: Option<String>,
}

impl Component for Wordle {
    type Message = WordleMsg;

    type Properties = WordleProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link()
            .send_future(async { WordleMsg::VerifyUserResponse(check_user_set().await) });

        let Self::Properties { game_id } = ctx.props();

        Self {
            animate: false,
            game_over: false,
            loading: true,
            game_id: game_id.into(),
            cell_i: 0,
            word_i: 0,
            verification_pending: true,
            state: vec![vec![CharCellState::Empty; 5]; 6],
            correctness_map: [Correctness::Guess; 28],
            toast_msg: Some("Loading".to_owned()),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::VerifyUserResponse(false) => {
                ctx.link().history().unwrap().push(Route::Register);
                false
            }
            Self::Message::VerifyUserResponse(true) => {
                self.verification_pending = false;
                let Self::Properties { game_id } = ctx.props();
                let url = format!("/api/v1/game/{game_id}/state");
                ctx.link().send_future(async move {
                    Self::Message::ApiResponse(WordleResponse::GetState(
                        match Request::get(&url)
                            .credentials(RequestCredentials::Include)
                            .send()
                            .await
                        {
                            Ok(resp) => resp.json::<GetStateResponse>().await,
                            Err(error) => {
                                log::error!(
                            "Something went wrong while trying to load game state! {error:?}"
                        );
                                Err(error)
                            }
                        },
                    ))
                });

                self.toast_msg = Some("Loading game state".to_owned());
                true
            }

            Self::Message::KeyboardInput(msg) => self.keydown_handler(ctx, msg),
            Self::Message::ApiResponse(WordleResponse::PlayGame(Ok(resp))) => {
                log::info!("Play submitted to leaderboard");
                log::info!("Received response: {resp:?}");
                self.game_over = self.game_over || resp.game_over;
                let has_won = resp.guess.into_iter().enumerate().fold(
                    true,
                    |all_correct, (i, (ch, correctness))| {
                        self.state[self.word_i][i] = CharCellState::Filled(FilledState {
                            ch,
                            correctness: Correctness::from(correctness),
                        });
                        self.update_correctness_map(ch, correctness);
                        all_correct && (correctness == game_model::Correctness::Correct)
                    },
                );
                self.cell_i = 0;
                self.word_i += 1;
                if self.word_i == 6 {
                    self.game_over = true;
                }
                self.animate = true;
                self.loading = false;
                if has_won {
                    self.toast_msg = Some("You won!".to_owned());
                } else if self.game_over {
                    self.toast_msg = Some("Game over :(".to_owned());
                }
                true
            }
            Self::Message::ApiResponse(WordleResponse::GetState(Ok(resp))) => {
                log::info!("Received game state response: {resp:?}");
                self.game_over = resp.game_over;
                self.word_i = resp.guesses.len();
                let has_won = resp
                    .guesses
                    .last()
                    .map(|guess| {
                        guess.iter().all(|&(_, correctness)| {
                            correctness == game_model::Correctness::Correct
                        })
                    })
                    .unwrap_or(false);
                resp.guesses
                    .into_iter()
                    .enumerate()
                    .for_each(|(word_i, guess)| {
                        guess
                            .into_iter()
                            .enumerate()
                            .for_each(|(char_i, (ch, correctness))| {
                                self.state[word_i][char_i] = CharCellState::Filled(FilledState {
                                    ch,
                                    correctness: Correctness::from(correctness),
                                });
                                self.update_correctness_map(ch, correctness);
                            });
                    });
                self.animate = true;
                self.loading = false;
                if has_won {
                    self.toast_msg = Some("You won!".to_owned());
                    main_menu_timer(ctx, 10_000);
                } else if self.game_over {
                    self.toast_msg = Some("Game over!".to_owned());
                    main_menu_timer(ctx, 7_000);
                }
                true
            }
            _ => {
                log::error!("Something went wrong!");
                self.toast_msg =
                    Some("An error occurred; please try refreshing this page".to_owned());
                true
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
                                html!{
                                    <Word text={text.clone()} animate={i+1 == self.word_i && self.animate}></Word>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    <Keyboard callback={onkeyclick} correctness_map={self.correctness_map}></Keyboard>
                    <Snackbar message={self.toast_msg.as_ref().cloned().unwrap_or(String::new())} display={self.toast_msg.is_some()}></Snackbar>
                </div>
            </div>

        }
    }
}

fn main_menu_timer(ctx: &Context<Wordle>, delay: u32) {
    let link = ctx.link().clone();
    Timeout::new(delay, move || link.history().unwrap().push(Route::Menu)).forget();
}

impl Wordle {
    fn update_correctness_map(&mut self, ch: char, correctness: game_model::Correctness) {
        let ord = ch as usize - 'A' as usize;
        // We do not demote correctness map for a character if it has already been set to correct anywhere
        if self.correctness_map[ord] != Correctness::Correct {
            if self.correctness_map[ord] == Correctness::Incorrect
                || self.correctness_map[ord] == Correctness::Guess
            {
                self.correctness_map[ord] = Correctness::from(correctness);
            } else {
                // The character is already incorrect position; we should not demote it to incorrect
                // Demotion can happen when the answer does not have repeating characters but guess
                // has repetition
                if correctness != game_model::Correctness::Incorrect {
                    self.correctness_map[ord] = Correctness::from(correctness);
                }
            }
        }
    }
    fn keydown_handler(&mut self, ctx: &Context<Self>, e: KeyboardMsg) -> bool {
        if self.game_over || self.loading {
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
                    self.loading = true;
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

use std::fmt::Display;

use game_model::{ManageGameResponse, PlayRequest, PlayResponse, PlayerResponse};
use gloo::timers::callback::Interval;
use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, RequestCredentials};
use yew::prelude::*;
use yew::Callback;
#[allow(unused, dead_code)]
use yew::{classes, html, Component, Context, Html, Properties};
use yew_router::prelude::*;
use yew_router::Routable;

use crate::{
    game_model::{CreateGameRequest, CreateGameResponse},
    user_model::{CreateUserIdRequest, COOKIE_USER_NAME},
};
mod game_model;
mod user_model;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Correctness {
    Incorrect,
    IncorrectPosition,
    Guess,
    Correct,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct FilledState {
    ch: char,
    correctness: Correctness,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CharCellState {
    Empty,
    Filled(FilledState),
}

impl Default for CharCellState {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Clone, Copy, Properties, PartialEq)]
struct CharCellProps {
    #[prop_or_default]
    state: CharCellState,
    #[prop_or((false, 0))]
    animate: (bool, u16),
}
struct CharCell;

impl Component for CharCell {
    type Message = ();

    type Properties = CharCellProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties { state, animate } = ctx.props();

        let mut classes = vec![
            "h-12",
            "w-12",
            "border",
            "border-solid",
            "grid",
            "place-content-center",
        ];
        // let delay = format!("delay-{}", animate.1);
        // classes.push(&delay);

        match state {
            CharCellState::Empty => html! {
                <div class={classes!("h-12", "w-12", "border", "border-white", "border-solid")}></div>
            },
            CharCellState::Filled(FilledState { ch, correctness }) => match correctness {
                Correctness::Guess => {
                    classes.push("border-white");
                    html! {
                        <div class={classes}>{ch.to_ascii_uppercase()}</div>
                    }
                }
                Correctness::Incorrect => {
                    if animate.0 {
                        classes.push("animate-card-flip-incorrect");
                    } else {
                        classes.push("border-gray-400");
                    }
                    html! {
                        <div class={classes!(classes)}>{ch.to_ascii_uppercase()}</div>
                    }
                }
                Correctness::IncorrectPosition => {
                    if animate.0 {
                        classes.push("animate-card-flip-position");
                    } else {
                        classes.push("border-orange-400");
                    }
                    html! {
                        <div class={classes!(classes)}>{ch.to_ascii_uppercase()}</div>
                    }
                }
                Correctness::Correct => {
                    if animate.0 {
                        classes.push("animate-card-flip-correct");
                    } else {
                        classes.push("border-green-400");
                    }
                    html! {
                        <div class={classes!(classes)}>{ch.to_ascii_uppercase()}</div>
                    }
                }
            },
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
struct WordProps {
    text: Vec<CharCellState>,
    #[prop_or(false)]
    animate: bool,
}

struct Word;

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

struct Keyboard;

#[derive(Clone, Copy)]
enum KeyboardMsg {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Backspace = 26,
    Enter = 27,
}

impl From<KeyboardMsg> for &'static str {
    fn from(km: KeyboardMsg) -> &'static str {
        match km {
            KeyboardMsg::A => "A",
            KeyboardMsg::B => "B",
            KeyboardMsg::C => "C",
            KeyboardMsg::D => "D",
            KeyboardMsg::E => "E",
            KeyboardMsg::F => "F",
            KeyboardMsg::G => "G",
            KeyboardMsg::H => "H",
            KeyboardMsg::I => "I",
            KeyboardMsg::J => "J",
            KeyboardMsg::K => "K",
            KeyboardMsg::L => "L",
            KeyboardMsg::M => "M",
            KeyboardMsg::N => "N",
            KeyboardMsg::O => "O",
            KeyboardMsg::P => "P",
            KeyboardMsg::Q => "Q",
            KeyboardMsg::R => "R",
            KeyboardMsg::S => "S",
            KeyboardMsg::T => "T",
            KeyboardMsg::U => "U",
            KeyboardMsg::V => "V",
            KeyboardMsg::W => "W",
            KeyboardMsg::X => "X",
            KeyboardMsg::Y => "Y",
            KeyboardMsg::Z => "Z",
            KeyboardMsg::Backspace => "⌫",
            KeyboardMsg::Enter => "Enter",
        }
    }
}

impl Display for KeyboardMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &'static str = (*self).into();
        write!(f, "{}", s)
    }
}

#[derive(PartialEq, Properties)]
struct KeyboardProps {
    #[prop_or(true)]
    display: bool,
    callback: Callback<KeyboardMsg>,
    #[prop_or([Correctness::Guess; 28])]
    correctness_map: [Correctness; 28],
}

impl Component for Keyboard {
    type Message = KeyboardMsg;

    type Properties = KeyboardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let Self::Properties { callback, .. } = ctx.props();
        callback.emit(msg);
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties {
            correctness_map,
            display,
            ..
        } = ctx.props();
        let mut wrapper_classes = vec![
            "w-full",
            "grid",
            "grid-rows-3",
            "gap-y-1",
            "place-content-center",
        ];
        if !display {
            wrapper_classes.push("hidden");
        }
        html! {
            <div class={classes!(wrapper_classes)}>
                <div class={classes!("w-full", "grid", "grid-cols-10", "gap-x-1", "justify-items-center", "content-center")}>
                    {
                        [KeyboardMsg::Q, KeyboardMsg::W, KeyboardMsg::E, KeyboardMsg::R, KeyboardMsg::T, KeyboardMsg::Y, KeyboardMsg::U, KeyboardMsg::I, KeyboardMsg::O, KeyboardMsg::P]
                        .into_iter().map(|k| {
                            render_key(ctx, k, correctness_map[k as usize])
                        }).collect::<Html>()
                    }
                </div>
                <div class={classes!("w-full", "grid", "grid-cols-11", "gap-x-1", "justify-items-center", "content-center")}>
                    <div class={classes!("h-10", "w-4", "text-white", "grid", "place-content-center")}></div>
                    {
                         [KeyboardMsg::A, KeyboardMsg::S, KeyboardMsg::D, KeyboardMsg::F, KeyboardMsg::G, KeyboardMsg::H, KeyboardMsg::J, KeyboardMsg::K, KeyboardMsg::L]
                         .into_iter().map(|k| {
                             render_key(ctx, k, correctness_map[k as usize])
                         }).collect::<Html>()
                    }
                    <div class={classes!("h-10", "w-4", "text-white", "grid", "place-content-center")}></div>
                </div>
                <div class={classes!("w-full", "grid", "grid-cols-11", "gap-x-1", "justify-items-center", "content-center")}>
                    <div class={classes!("h-10", "w-4", "text-white", "grid", "place-content-center")}></div>
                {
                    [KeyboardMsg::Enter, KeyboardMsg::Z, KeyboardMsg::X, KeyboardMsg::C, KeyboardMsg::V, KeyboardMsg::B, KeyboardMsg::N, KeyboardMsg::M, KeyboardMsg::Backspace]
                    .into_iter().map(|k| {
                        render_key(ctx, k, correctness_map[k as usize])
                    }).collect::<Html>()
                }
                    <div class={classes!("h-10", "w-4", "text-white", "grid", "place-content-center")}></div>
                </div>
            </div>
        }
    }
}

fn render_key(ctx: &Context<Keyboard>, key: KeyboardMsg, state: Correctness) -> Html {
    let mut classes = vec![
        "h-10",
        "w-8",
        "text-white",
        "grid",
        "place-content-center",
        "border",
        "border-solid",
    ];
    match key {
        KeyboardMsg::Backspace => html! {
            <div onclick={ctx.link().callback(|_| KeyboardMsg::Backspace)} class={classes!(classes)}>{KeyboardMsg::Backspace}</div>
        },
        KeyboardMsg::Enter => {
            classes.push("text-xs");
            html! {
                <div onclick={ctx.link().callback(|_| KeyboardMsg::Enter)} class={classes!(classes)}>{KeyboardMsg::Enter}</div>
            }
        }
        k => {
            match state {
                Correctness::Guess => classes.push("border-white"),
                Correctness::Correct => classes.push("border-green-500"),
                Correctness::IncorrectPosition => classes.push("border-orange-400"),
                Correctness::Incorrect => classes.push("border-gray-500"),
            }

            html! {
                <div onclick={ctx.link().callback(move |_| k)} class={classes!(classes)}>{k}</div>
            }
        }
    }
}

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
    let mut registered = true;
    if user_name.is_none() || user_name.as_ref().unwrap().is_empty() {
        None
    } else {
        log::info!("Found user_name cookie with name: {user_name:?}");
        log::info!("Not re-registering user!");
        Some(user_name.unwrap().chars().collect())
    }
}
enum WordleResponse {
    PlayGame(Result<PlayResponse, reqwasm::Error>),
}
enum WordleMsg {
    KeyboardInput(KeyboardMsg),
    ApiResponse(WordleResponse),
}

#[derive(PartialEq, Properties)]
struct WordleProps {
    game_id: String,
}

struct Wordle {
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

#[derive(Clone, PartialEq, Properties)]
struct LeaderboardProps {
    game_id: String,
}

enum LeaderboardMsg {
    Api(Result<ManageGameResponse, reqwasm::Error>),
}

struct Leaderboard {
    answer: Option<String>,
    interval: Interval,
    players: Option<Vec<PlayerResponse>>,
}

impl Component for Leaderboard {
    type Message = LeaderboardMsg;

    type Properties = LeaderboardProps;

    fn create(ctx: &Context<Self>) -> Self {
        let Self::Properties { game_id } = ctx.props().clone();
        let handle = {
            let link = ctx.link().clone();
            Interval::new(5_000, move || {
                let path = format!("/api/v1/manage/{}", game_id);
                link.send_future(async move {
                    log::info!("Querying api server");
                    Self::Message::Api(
                        match Request::get(&path)
                            .credentials(RequestCredentials::Include)
                            .send()
                            .await
                        {
                            Ok(resp) => {
                                log::info!("Received response: {resp:?}");
                                resp.json::<ManageGameResponse>().await
                            }
                            Err(error) => Err(error),
                        },
                    )
                })
            })
        };

        Self {
            answer: None,
            interval: handle,
            players: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Api(Ok(resp)) => {
                log::info!("Decoded response: {resp:?}");
                self.answer = Some(resp.answer);
                self.players = Some(resp.players);
                true
            }
            Self::Message::Api(Err(error)) => {
                log::info!("Error: {error:?}");
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties { game_id } = ctx.props();
        let mut classes = vec![
            "w-full",
            "flex",
            "flex-col",
            "flex-nowrap",
            "text-white",
            "items-center",
            "justify-center",
            "border",
            "border-red-400",
            "border-solid",
        ];

        html! {
            <div class={classes!("w-full", "h-full", "grid", "place-content-center")}>
                <div class={classes!("grid", "w-80", "md:w-100", "lg:w-150",  "h-full", "gap-y-5", "justify-items-center", "content-center")}>
                    <div class={classes!(classes)}>
                        <h1>{"Leaderboard"}</h1>
                        <h2>{format!("Game id: {}", game_id)}</h2>
                        <h3>{if let Some(answer) = &self.answer { answer } else { "Loading..." }}</h3>
                        {
                            if let Some(players) = &self.players {
                                if players.len() == 0 {
                                    html!{"No players yet"}
                                } else {

                                players.iter().map(|player| {
                                    let (ncorr, nincorr_pos, nincorr) = player.guesses.iter().map(|guess| guess.guess.iter()).flatten().fold((0,0,0), |(st_correct, st_incorrect_pos, st_incorrect), (_, correctness)|
                                        match correctness {
                                            game_model::Correctness::Correct => (st_correct+1, st_incorrect_pos, st_incorrect),
                                            game_model::Correctness::IncorrectPosition => (st_correct, st_incorrect_pos+1, st_incorrect),
                                            game_model::Correctness::Incorrect=> (st_correct, st_incorrect_pos, st_incorrect+1),
                                        });
                                    html!{
                                        <div class={classes!("flex", "w-full", "text-white", "justify-between")}>
                                            <div>{player.name.clone()}</div>
                                            <div>{format!("{}/6", player.guesses.len())}</div>
                                            <div class={classes!("flex", "text-white")}>
                                                <div class={classes!("bg-green-400")}>{ncorr}</div>
                                                <div class={classes!("bg-orange-400")}>{nincorr_pos}</div>
                                                <div class={classes!("bg-black-400")}>{nincorr}</div>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                            } else {
                                html!{"Loading..."}
                            }
                        }
                    </div>
                </div>
            </div>
        }
    }
}

const PROMPT: &'static str = "Enter name";
enum RegisterMsg {
    KeyboardInput(KeyboardMsg),
    RegisterUserResponse(Result<(), reqwasm::Error>),
}

#[derive(PartialEq, Properties)]
struct RegisterProps {
    #[prop_or(true)]
    display: bool,
}
struct Register {
    user_name: Vec<char>,
}

impl Component for Register {
    type Message = RegisterMsg;

    type Properties = RegisterProps;

    fn create(ctx: &Context<Self>) -> Self {
        if check_user_set().is_some() {
            ctx.link().history().unwrap().push(Route::Menu);
        }
        Self {
            user_name: PROMPT.chars().collect(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RegisterMsg::KeyboardInput(msg) => self.keydown_handler(ctx, msg),
            RegisterMsg::RegisterUserResponse(Ok(_)) => {
                ctx.link().history().unwrap().push(Route::Menu);
                false
            }
            RegisterMsg::RegisterUserResponse(Err(error)) => {
                log::error!("Error registering user: {error:?}");
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties { display } = ctx.props();
        let mut register_classes = vec![
            "h-80",
            "w-full",
            "grid",
            "grid-rows-2",
            "gap-y-1",
            "text-white",
        ];

        if !display {
            register_classes.push("hidden");
        }

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

struct AnswerInput {
    answer: Vec<CharCellState>,
    cell_i: usize,
    submitted: bool,
    animate: bool,
}

enum AnswerInputResponse {
    CreateGame(Result<CreateGameResponse, reqwasm::Error>),
}
enum AnswerInputMsg {
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
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeyclick = ctx.link().callback(|e: KeyboardMsg| {
            log::info!("Received KeyboardMsg: {e}");
            AnswerInputMsg::KeyboardInput(e)
        });
        let mut answer_classes = vec![
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

enum MenuMsg {
    Input(String),
    Submit,
    SubmitResponse(Result<(), reqwasm::Error>),
}
struct Menu {
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
                                Ok(resp) => Ok(()),
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

#[derive(Clone, Routable, PartialEq)]
enum Route {
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

fn switch(route: &Route) -> Html {
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
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

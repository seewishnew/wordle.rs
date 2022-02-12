use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::Sub,
};

#[allow(unused, dead_code)]
use yew::{classes, html, Component, Context, Html, Properties};
use yew::{virtual_dom::Key, Callback, KeyboardEvent};

#[derive(Clone, Copy, PartialEq)]
enum CorrectnessLevel {
    Incorrect,
    IncorrectPosition,
    Guess,
    Correct,
}

#[derive(Clone, Copy, PartialEq)]
struct FilledState {
    ch: char,
    correctness: CorrectnessLevel,
}

#[derive(Clone, Copy, PartialEq)]
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
                CorrectnessLevel::Guess => {
                    classes.push("border-white");
                    html! {
                        <div class={classes}>{ch.to_ascii_uppercase()}</div>
                    }
                }
                CorrectnessLevel::Incorrect => {
                    if animate.0 {
                        classes.push("animate-card-flip-incorrect");
                    } else {
                        classes.push("border-gray-400");
                    }
                    html! {
                        <div class={classes!(classes)}>{ch.to_ascii_uppercase()}</div>
                    }
                }
                CorrectnessLevel::IncorrectPosition => {
                    if animate.0 {
                        classes.push("animate-card-flip-position");
                    } else {
                        classes.push("border-orange-400");
                    }
                    html! {
                        <div class={classes!(classes)}>{ch.to_ascii_uppercase()}</div>
                    }
                }
                CorrectnessLevel::Correct => {
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
    callback: Callback<KeyboardMsg>,
    correctness_map: [CorrectnessLevel; 28],
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
            correctness_map, ..
        } = ctx.props();
        html! {
            <div class={classes!("w-full", "grid", "grid-rows-3", "gap-y-1", "place-content-center")}>
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

fn render_key(ctx: &Context<Keyboard>, key: KeyboardMsg, state: CorrectnessLevel) -> Html {
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
                CorrectnessLevel::Guess => classes.push("border-white"),
                CorrectnessLevel::Correct => classes.push("border-green-500"),
                CorrectnessLevel::IncorrectPosition => classes.push("border-orange-400"),
                CorrectnessLevel::Incorrect => classes.push("border-gray-500"),
            }

            html! {
                <div onclick={ctx.link().callback(move |_| k)} class={classes!(classes)}>{k}</div>
            }
        }
    }
}

struct Wordle {
    animate: bool,
    game_over: bool,
    answer: Vec<char>,
    ans_imap: HashMap<char, HashSet<usize>>,
    cell_i: usize,
    word_i: usize,
    state: Vec<Vec<CharCellState>>,
    correctness_map: [CorrectnessLevel; 28],
}

impl Component for Wordle {
    type Message = KeyboardMsg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let answer = vec!['H', 'E', 'L', 'L', 'O'];
        let mut ans_imap = HashMap::new();
        answer.iter().enumerate().for_each(|(i, &ch)| {
            ans_imap.entry(ch).or_insert(HashSet::new()).insert(i);
        });

        Self {
            animate: false,
            game_over: false,
            answer: answer,
            ans_imap: ans_imap,
            cell_i: 0,
            word_i: 0,
            state: vec![vec![CharCellState::Empty; 5]; 6],
            correctness_map: [CorrectnessLevel::Guess; 28],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.keydown_handler(msg)
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeyclick = ctx.link().callback(|e: KeyboardMsg| {
            log::info!("Received KeyboardMsg: {e}");
            e
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
    fn keydown_handler(&mut self, e: KeyboardMsg) -> bool {
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
                    let mut guess_imap = HashMap::new();
                    self.state[self.word_i]
                        .iter()
                        .enumerate()
                        .for_each(|(i, ccs)| match ccs {
                            CharCellState::Filled(FilledState { ch, correctness }) => {
                                match correctness {
                                    CorrectnessLevel::Guess => {
                                        guess_imap.entry(*ch).or_insert(HashSet::new()).insert(i);
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            _ => unreachable!(),
                        });
                    log::info!("guess_imap: {:?}", guess_imap);
                    log::info!("ans_imap: {:?}", self.ans_imap);
                    if self.ans_imap == guess_imap {
                        self.game_over = true;
                        self.state[self.word_i]
                            .iter_mut()
                            .zip(self.answer.iter())
                            .for_each(|(css, &ch)| {
                                *css = CharCellState::Filled(FilledState {
                                    ch: ch,
                                    correctness: CorrectnessLevel::Correct,
                                });
                                self.correctness_map[ch as usize - 'A' as usize] =
                                    CorrectnessLevel::Correct;
                            });
                        log::info!("Congratulations!");
                    } else {
                        guess_imap.iter().for_each(|(guess_ch, guess_pos)| {
                            if let Some(ans_pos) = self.ans_imap.get(guess_ch) {
                                guess_pos.difference(ans_pos).for_each(|&i| {
                                    self.state[self.word_i][i] =
                                        CharCellState::Filled(FilledState {
                                            ch: *guess_ch,
                                            correctness: CorrectnessLevel::IncorrectPosition,
                                        });
                                    self.correctness_map[*guess_ch as usize - 'A' as usize] =
                                        CorrectnessLevel::IncorrectPosition;
                                });
                                guess_pos.intersection(ans_pos).for_each(|&i| {
                                    self.state[self.word_i][i] =
                                        CharCellState::Filled(FilledState {
                                            ch: *guess_ch,
                                            correctness: CorrectnessLevel::Correct,
                                        });
                                    self.correctness_map[*guess_ch as usize - 'A' as usize] =
                                        CorrectnessLevel::Correct;
                                });
                            } else {
                                guess_pos.iter().for_each(|&i| {
                                    self.state[self.word_i][i] =
                                        CharCellState::Filled(FilledState {
                                            ch: *guess_ch,
                                            correctness: CorrectnessLevel::Incorrect,
                                        });
                                    self.correctness_map[*guess_ch as usize - 'A' as usize] =
                                        CorrectnessLevel::Incorrect;
                                })
                            }
                        });
                    }
                    self.cell_i = 0;
                    self.word_i += 1;
                    if self.word_i == 6 {
                        self.game_over = true;
                    }
                    self.animate = true;
                } else {
                    return false;
                }
            }
            ch => {
                let ch: &str = ch.into();
                if ch.len() == 1 && self.cell_i <= 4 {
                    self.state[self.word_i][self.cell_i] = CharCellState::Filled(FilledState {
                        ch: ch.chars().next().unwrap().to_ascii_uppercase(),
                        correctness: CorrectnessLevel::Guess,
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

struct Game;

impl Component for Game {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // log::info!("Rendering Game");
        html! {
            <Wordle></Wordle>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Game>();
}

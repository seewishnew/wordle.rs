use std::{collections::{HashMap, HashSet}, ops::Sub};

use yew::{KeyboardEvent, virtual_dom::Key};
#[allow(unused, dead_code)]
use yew::{classes, html, Component, Context, Html, Properties};

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
}
struct CharCell;

impl Component for CharCell {
    type Message = ();

    type Properties = CharCellProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties { state } = ctx.props();

        match state {
            CharCellState::Empty => html! {
                <div class={classes!("h-12", "w-12", "border", "border-white", "border-solid")}></div>
            },
            CharCellState::Filled(FilledState { ch, correctness }) => match correctness {
                CorrectnessLevel::Guess => html! {
                    <div class={classes!("h-12", "w-12", "border", "border-white", "border-solid", "grid", "place-content-center")}>{ch.to_ascii_uppercase()}</div>
                },
                CorrectnessLevel::Incorrect => html! {
                    <div class={classes!("h-12", "w-12", "border", "border-gray-400", "border-solid", "grid", "place-content-center")}>{ch.to_ascii_uppercase()}</div>
                },
                CorrectnessLevel::IncorrectPosition => html! {
                    <div class={classes!("h-12", "w-12", "border", "border-orange-400", "border-solid", "grid", "place-content-center")}>{ch.to_ascii_uppercase()}</div>
                },
                CorrectnessLevel::Correct => html! {
                    <div class={classes!("h-12", "w-12", "border", "border-green-400", "border-solid", "grid", "place-content-center")}>{ch.to_ascii_uppercase()}</div>
                },
            },
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
struct WordProps {
    text: Vec<CharCellState>,
}

struct Word;

impl Component for Word {
    type Message = ();

    type Properties = WordProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties { text} = ctx.props();
            html! {
                <div class={classes!("grid", "grid-cols-5", "gap-x-1", "justify-items-center", "content-evenly")}>
                {
                    text.iter().map(|ccs| {
                        html!{
                            <CharCell state={ccs.clone()}></CharCell>
                        }
                    }).collect::<Html>()
                }
                </div>
            }
    }
}

struct Wordle{
    game_over: bool,
    answer: Vec<char>,
    ans_imap: HashMap<char, HashSet<usize>>,
    cell_i: usize,
    word_i: usize,
    state: Vec<Vec<CharCellState>>,
}

impl Component for Wordle {
    type Message = KeyboardEvent;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let answer = vec!['H', 'E', 'L', 'L', 'O'];
        let mut ans_imap = HashMap::new();
        answer.iter().enumerate().for_each(|(i, &ch)| {
            ans_imap.entry(ch).or_insert(HashSet::new()).insert(i);
        });

        Self {
            game_over: false,
            answer: answer,
            ans_imap: ans_imap,
            cell_i: 0,
            word_i: 0,
            state: vec![vec![CharCellState::Empty; 5]; 6],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.keydown_handler(msg)
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeypress = ctx.link().callback(|e: KeyboardEvent| {
            log::info!("Received key: {}", e.key());
            e
        });
        html! {
            <div tabIndex={"0"} {onkeypress} class={classes!("h-80", "w-80", "md:w-100", "lg:w-150", "grid", "grid-rows-6", "gap-y-1", "text-white")}>
                {
                    self.state.iter().enumerate().map(|(i, text)| {
                        log::info!("Rendering word #{i}, word_i: {}", self.word_i);
                        html!{
                            <Word text={text.clone()}></Word>
                        }
                    }).collect::<Html>()
                }
            </div>
        }
    }
}

impl Wordle {
    fn keydown_handler(&mut self, e: KeyboardEvent) -> bool {
        if self.game_over {
            return false;
        }

        match e.key().as_str() {
            "Backspace" => if self.cell_i > 0 {
                self.cell_i -= 1;
                self.state[self.word_i][self.cell_i] = CharCellState::Empty;
            } else {
                return false;
            }
            "Enter" => if self.cell_i == 5 {
                let mut guess_imap = HashMap::new();
                self.state[self.word_i].iter().enumerate().for_each(|(i, ccs)| {
                    match ccs {
                        CharCellState::Filled(FilledState { ch, correctness}) => match correctness {
                            CorrectnessLevel::Guess => {
                                guess_imap.entry(*ch).or_insert(HashSet::new()).insert(i);
                            }, 
                            _ => unreachable!(),
                        }
                        _ => unreachable!(),
                    }
                });
                log::info!("guess_imap: {:?}", guess_imap);
                log::info!("ans_imap: {:?}", self.ans_imap);
                if self.ans_imap == guess_imap {
                    self.game_over = true;
                    self.state[self.word_i].iter_mut().zip(self.answer.clone().into_iter()).for_each(|(css, ch)| {
                        *css = CharCellState::Filled(FilledState{
                            ch: ch,
                            correctness: CorrectnessLevel::Correct,
                        })
                    });
                    log::info!("Congratulations!");
                } else {
                    guess_imap.iter().for_each(|(guess_ch, guess_pos)| {
                        if let Some(ans_pos) = self.ans_imap.get(guess_ch) {
                            guess_pos.difference(ans_pos).for_each(|&i| {
                                self.state[self.word_i][i] = CharCellState::Filled(FilledState{
                                    ch: *guess_ch,
                                    correctness: CorrectnessLevel::IncorrectPosition,
                                });
                            });
                            guess_pos.intersection(ans_pos).for_each(|&i| {
                                self.state[self.word_i][i] = CharCellState::Filled(FilledState{
                                    ch: *guess_ch,
                                    correctness: CorrectnessLevel::Correct,
                                })
                            });
                        } else {
                            guess_pos.iter().for_each(|&i| {
                                self.state[self.word_i][i] = CharCellState::Filled(FilledState{
                                    ch: *guess_ch,
                                    correctness: CorrectnessLevel::Incorrect,
                                });
                            })
                        }
                    });
                }
                self.cell_i = 0;
                self.word_i += 1;
                if self.word_i == 6 {
                    self.game_over = true;
                }
            } else {
                return false;
            }
            ch => if ch.len() == 1 && self.cell_i <= 4 {
                    self.state[self.word_i][self.cell_i] = CharCellState::Filled(FilledState{
                        ch: ch.chars().next().unwrap().to_ascii_uppercase(), 
                        correctness: CorrectnessLevel::Guess
                    });
                    if self.cell_i <= 4 {
                        self.cell_i += 1;
                    }
                } else {
                    return false;
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
        log::info!("Rendering Game");
        html!{
            <div class={classes!("flex", "justify-center", "items-center", "h-100")}>
                <Wordle></Wordle>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Hello, Yew!");
    yew::start_app::<Game>();
}

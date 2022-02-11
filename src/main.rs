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
                    <div class={classes!("h-12", "w-12", "border", "border-white", "border-solid")}>{ch.to_ascii_uppercase()}</div>
                },
                CorrectnessLevel::Incorrect => html! {
                    <div class={classes!("h-12", "w-12", "border", "border-white", "border-solid")}>{ch.to_ascii_uppercase()}</div>
                },
                CorrectnessLevel::IncorrectPosition => html! {
                    <div class={classes!("h-12", "w-12", "border", "border-orange-400", "border-solid")}>{ch.to_ascii_uppercase()}</div>
                },
                CorrectnessLevel::Correct => html! {
                    <div class={classes!("h-12", "w-12", "border", "border-green", "border-solid")}>{ch.to_ascii_uppercase()}</div>
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
        let Self::Properties { text } = ctx.props();
        // let nchars = text.len();
        // let grid_cols = format!("grid-cols-[{nchars}]");

        html! {
            <div class={classes!("grid", "grid-cols-5", "gap-x-1", "justify-items-center", "content-evenly", "border", "border-blue-300", "border-solid")}>
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
    cell_i: usize,
    word_i: usize,
    state: Vec<Vec<CharCellState>>,
}

impl Component for Wordle {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            cell_i: 0,
            word_i: 0,
            state: vec![vec![CharCellState::Empty; 5]; 6],
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("h-80", "w-80", "md:w-100", "lg:w-150", "grid", "grid-rows-6", "gap-y-1", "border-gray-50", "border-2", "border-solid")}>
                {
                    self.state.iter().enumerate().map(|(i, text)| {
                        html!{
                            <Word text={text.clone()}></Word>
                        }
                    }).collect::<Html>()
                }
            </div>
        }
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
        html!{
            <div class={classes!("flex", "justify-center", "items-center", "h-100")}>
                <Wordle></Wordle>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Game>();
}

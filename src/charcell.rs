#[allow(unused, dead_code)]
use yew::{classes, html, Component, Context, Html, Properties};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Correctness {
    Incorrect,
    IncorrectPosition,
    Guess,
    Correct,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FilledState {
    pub ch: char,
    pub correctness: Correctness,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CharCellState {
    Empty,
    Filled(FilledState),
}

impl Default for CharCellState {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Clone, Copy, Properties, PartialEq)]
pub struct CharCellProps {
    #[prop_or_default]
    pub state: CharCellState,
    #[prop_or((false, 0))]
    pub animate: (bool, u16),
}
pub struct CharCell;

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

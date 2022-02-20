use std::fmt::Display;

use crate::charcell::Correctness;
#[allow(unused, dead_code)]
use yew::{classes, html, Callback, Component, Context, Html, Properties};

pub struct Keyboard;

#[derive(Clone, Copy)]
pub enum KeyboardMsg {
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
pub struct KeyboardProps {
    #[prop_or(true)]
    pub display: bool,
    pub callback: Callback<KeyboardMsg>,
    #[prop_or([Correctness::Guess; 28])]
    pub correctness_map: [Correctness; 28],
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

use gloo::timers::callback::Interval;
use reqwasm::http::Request;
use web_sys::RequestCredentials;
#[allow(unused, dead_code)]
use yew::{classes, html, Component, Context, Html, Properties};

use crate::game_model::{self, ManageGameResponse, PlayerResponse};

#[derive(Clone, PartialEq, Properties)]
pub struct LeaderboardProps {
    pub game_id: String,
}

pub enum LeaderboardMsg {
    Api(Result<ManageGameResponse, reqwasm::Error>),
}

pub struct Leaderboard {
    answer: Option<String>,
    players: Option<Vec<PlayerResponse>>,
}

impl Component for Leaderboard {
    type Message = LeaderboardMsg;

    type Properties = LeaderboardProps;

    fn create(ctx: &Context<Self>) -> Self {
        let Self::Properties { game_id } = ctx.props().clone();
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
        .forget();

        Self {
            answer: None,
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
        let classes = vec![
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

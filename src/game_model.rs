use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CreateGameRequest {
    pub answer: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Guess {
    pub guess: Vec<(char, Correctness)>,
    pub submit_time: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Correctness {
    Correct,
    IncorrectPosition,
    Incorrect,
}

impl From<Correctness> for crate::Correctness {
    fn from(correctness: Correctness) -> Self {
        match correctness {
            Correctness::Correct => crate::Correctness::Correct,
            Correctness::IncorrectPosition => crate::Correctness::IncorrectPosition,
            Correctness::Incorrect=> crate::Correctness::Incorrect,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateGameResponse {
    pub game_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerResponse {
    pub name: String,
    pub start_time: u64,
    pub guesses: Vec<Guess>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ManageGameResponse {
    pub start_time: u64,
    pub players: Vec<PlayerResponse>,
    pub answer: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayRequest {
    pub guess: Vec<char>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayResponse {
    pub game_over: bool,
    pub guess: Vec<(char, Correctness)>
}
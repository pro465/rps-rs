pub mod game;
pub mod object;

mod ai;
pub use crate::ai::AI;

pub mod game_res {
    pub enum GameRes {
        AiWon,
        PlayerWon,
        Draw,
    }
}

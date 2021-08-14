pub mod game;
pub mod object;
//pub mod rand;
//pub mod pool;

mod ai;
pub use ai::AI;

pub mod game_res {
    pub enum GameRes {
        AiWon,
        PlayerWon,
        Draw,
    }
}

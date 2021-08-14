use ai::object::Object;
use ai::AI;

use ai::game;
use ai::game_res::GameRes;

use std::convert::TryInto;
use std::io::{self, Write};

fn main() {
    let mut ai = AI::<5>::new();

    let (mut player_score, mut ai_score) = (0, 0);

    let mut i = 1;

    loop {
        let ai_choice = ai.choose();

        let player_choice: Object = loop {
            let mut s = String::new();

            print!("Your choice: ");

            io::stdout().flush().unwrap();

            io::stdin().read_line(&mut s).unwrap();

            match s.trim().try_into() {
                Ok(x) => break x,
                Err(x) => println!("{}", x),
            }
        };

        println!(
            "AI chose: {}",
            ["rock", "paper", "scissor"][ai_choice as usize]
        );

        let result = game::winner(ai_choice, player_choice);

        match result {
            GameRes::AiWon => ai_score += 1,
            GameRes::PlayerWon => player_score += 1,
            _ => (),
        }

        println!();

        game::show_score(ai_score, player_score);

        ai.update(player_choice);

        if i == 6 {
            i = 0;
            ai.train();
        }

        i += 1;
    }
}

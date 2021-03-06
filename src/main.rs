use ai::object::Object;
use ai::AI;

use ai::game;
use ai::game_res::GameRes;

use std::convert::TryInto;
use std::io::{self, BufRead, Write};

fn main() {
    let mut ai = AI::<5>::new();

    let (mut player_score, mut ai_score) = (0, 0);

    let mut i = 1;

    print!("Your choice: ");
    io::stdout().flush().unwrap();

    for inp in io::stdin().lock().lines() {
        let player_choice: Object = match inp.unwrap().as_str().try_into() {
            Ok(x) => x,
            Err(x) => {
                println!("{}", x);
                print!("Your choice: ");
                io::stdout().flush().unwrap();
                continue;
            }
        };

        let ai_choice = ai.choose();

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

        print!("Your choice: ");
        io::stdout().flush().unwrap();

        ai.update(player_choice);

        if i == 6 {
            i = 0;
            ai.train();
        }

        i += 1;
    }
}

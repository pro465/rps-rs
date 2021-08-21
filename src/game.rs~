use crate::game_res::GameRes;
use crate::object::Object;

use std::convert::TryInto;

pub fn winner(ai: Object, player: Object) -> GameRes {
    match (player, ai) {
        (Object::Paper, Object::Rock) => {
            println!("ai got covered by you");
            GameRes::PlayerWon
        }
        (Object::Rock, Object::Paper) => {
            println!("you got covered by ai");
            GameRes::AiWon
        }

        (Object::Scissor, Object::Paper) => {
            println!("ai got cut by you");
            GameRes::PlayerWon
        }
        (Object::Paper, Object::Scissor) => {
            println!("you got cut by ai");
            GameRes::AiWon
        }

        (Object::Rock, Object::Scissor) => {
            println!("ai got blunted by you");
            GameRes::PlayerWon
        }
        (Object::Scissor, Object::Rock) => {
            println!("you got blunted by ai");
            GameRes::AiWon
        }

        _ => {
            println!("it is a draw");
            GameRes::Draw
        }
    }
}

pub fn show_score(ai: u32, person: u32) {
    let person = person.to_string();
    let ai = ai.to_string();
    println!(
        "{}+{}",
        "=".repeat(person.len() + 3),
        "=".repeat(ai.len() + 2),
    );
    println!("You{}|{}AI", " ".repeat(person.len()), " ".repeat(ai.len()));
    println!(
        "{}+{}",
        "=".repeat(person.len() + 3),
        "=".repeat(ai.len() + 2),
    );
    println!("{}   |  {}\n\n", person, ai);
}

pub fn winning_move(curr: Object) -> Object {
    ((curr as usize + 1) % 3).try_into().unwrap()
}

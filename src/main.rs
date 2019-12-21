
#[macro_use]
extern crate enum_display_derive;
extern crate termion;

use termion::{color, style};
use std::io::{self, Write};

mod game;
mod minimax;

const PLAYER_USER: game::Player = game::Player::X;
//const PLAYER_AI: game::Player = game::Player::O;
const START_PLAYER: game::Player = PLAYER_USER;

fn main() {
    let mut board = game::Board::new(START_PLAYER);

    while !board.is_ended() {
        board.print(true);      // true = clearing the screen
        let next_player = board.next_player();

        //println!("Turn: {}", next_player);
        println!("");
        let action;

        if next_player == PLAYER_USER {
            print!("Action [e.g. 1a]: ");
            io::stdout().flush().unwrap();

            // Player's turn
            action = get_user_input(board);
            
        } else {

        	// AI's turn
        	action = minimax::find_best_move(board, board.next_player);
        	
        }
        board.perform_action(action); // perform human or AI's turn
		println!();
    }

    println!("{}{}Game Ended{}\n", color::Fg(color::Yellow), style::Bold, style::Reset);

    if let Some(value) = board.get_winner() {
        println!("Winner is Player {}", value);
    } else {
        println!("Game ended with a draw");
    }

    println!("\nFinal board:\n");
    board.print(false);         // false = not clearing the screen
    println!("\nExiting...\n");

}

fn get_user_input(board: game::Board) -> (i32, i32) {

    let mut user_action;

    loop {
        // Read player's action
        let mut player_action = String::new();
        io::stdin().read_line(&mut player_action).unwrap();

        let player_action = player_action.trim();

        if player_action.is_empty() {
            print!("> ");
            io::stdout().flush().unwrap();
            continue;
        }

        // Validate input
        if player_action.len() != 2 {
            println!("Invalid action");
            print!("> ");
            io::stdout().flush().unwrap();
            continue;
        };

        let mut chars = player_action.chars();

        let row = chars.next().unwrap() as i32 - '1' as i32;
        let col = chars.next().unwrap() as i32 - 'a' as i32;

        user_action = (row, col);

        if !board.is_legal_action(user_action) {
            println!("Illegal action");
            print!("> ");
            io::stdout().flush().unwrap();
            continue;
        };

        break;
    }

    user_action
}


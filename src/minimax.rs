
use crate::game::Board;
use crate::game::Player;

use std::cmp;

pub fn minimax(board: Board, player: Player, depth: i32) -> i32 { 

	if board.is_ended() {
		match board.get_winner() {
			Some(Player::O) 	=> return 10,	// AI won
			Some(Player::X)   	=> return -10,	// PLAYER won
			None				=> return 0,	// draw
		}
	}

	if player == Player::O { 					// Simulate AI (Max of Minimax)
		let mut best_move = -1000;
		let possible_moves = board.get_actions();
		for amove in possible_moves {
			let mut board_copy = board.clone(); // copy board
			board_copy.perform_action(amove);	// perform action in board copy
			let result = minimax(board_copy, player.opponent(), depth + 1);
			best_move = cmp::max(best_move, result);
		}
		return best_move;
	} else {									// Simulate Human (Mini of Minimax)
		let mut best_move = 1000;
		let possible_moves = board.get_actions();
		for amove in possible_moves {
			let mut board_copy = board.clone(); // copy board
			board_copy.perform_action(amove);	// perform action in board copy
			let result = minimax(board_copy, player.opponent(), depth + 1);
			best_move = cmp::min(best_move, result);
		}
		return best_move;
	}

}

pub fn find_best_move(board: Board, player: Player) -> (i32, i32) {

	let mut best_score = -1000;
	let mut best_move = (-1, -1);

	let possible_moves = board.get_actions();
	for amove in possible_moves {
		let mut board_copy = board.clone(); // copy board
		board_copy.perform_action(amove);	// perform action in board copy
		//let score = minimax(board_copy, Player::X, 0);
		let score = minimax(board_copy, player.opponent(), 0);
		if score > best_score {
			best_score = score;
			best_move = amove;
		}
	}
	return best_move;
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_minimax() {
        let mut board = Board::new(Player::O);
        board.fields = [
            [Some(Player::X),    None,     			Some(Player::O)],
            [Some(Player::X),    None,              None],
            [Some(Player::O),    Some(Player::X),   None]
        ];

        assert_eq!(find_best_move(board, board.next_player), (1,1));
    }
}



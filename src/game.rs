
use std::fmt::{self, Debug, Display};
use termion::color;

#[derive(Display, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn opponent(&self) -> Player {
        match *self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Board {
    pub fields: [[Option<Player>; 3]; 3],
    pub next_player: Player,
}

impl Board {
    pub fn new(first_player: Player) -> Board {
        Board {
            fields: [
                [None, None, None],
                [None, None, None],
                [None, None, None]
            ],
            next_player: first_player,
        }
    }

    pub fn next_player(&self) -> Player {
        self.next_player
    }

    pub fn is_ended(&self) -> bool {
        if let Some(_) = self.get_winner() {
            true
        } else {
            // All cells used: a draw (ended)
            self.fields.iter().all(|row| {
                row.iter().all(|cell| cell.is_some())
            })
        }
    }

    pub fn get_winner(&self) -> Option<Player> {
        macro_rules! has {
            ($player:expr, $x:expr, $y:expr) => {
                self.fields[$x][$y] == Some(*$player)
            };
        }

        for player in &[Player::X, Player::O] {
            // Three in a row: horizontally
            for row in 0..=2 {
                if has!(player, row, 0) && has!(player, row, 1) && has!(player, row, 2) {
                    return Some(*player);
                }
            }

            // Three in a row: vertically
            for col in 0..=2 {
                if has!(player, 0, col) && has!(player, 1, col) && has!(player, 2, col) {
                    return Some(*player);
                }
            }

            // Three in a row: diagonally (top-left to bottom-right)
            if has!(player, 0, 0) && has!(player, 1, 1) && has!(player, 2, 2) {
                return Some(*player);
            }

            // Three in a row: diagonally (top-right to bottom-left)
            if has!(player, 0, 2) && has!(player, 1, 1) && has!(player, 2, 0) {
                return Some(*player);
            }
        }

        None
    }

    pub fn is_legal_action(&self, action: (i32, i32)) -> bool {
        if action.0 < 0 || action.0 > 2 || action.1 < 0 || action.1 > 2 {
            return false;
        }

        self.fields[action.0 as usize][action.1 as usize].is_none()
    }

    pub fn perform_action(&mut self, action: (i32, i32)) {
        debug_assert!(self.is_legal_action(action));

        // Perform...
        self.fields[action.0 as usize][action.1 as usize] = Some(self.next_player);

        // Next player's turn
        self.next_player = self.next_player.opponent();
    }

    pub fn get_actions(&self) -> Vec<(i32, i32)> {
        if self.is_ended() {
            return Vec::new();
        }

        let mut actions = Vec::with_capacity(9);

        // Calculate possible moves
        for row in 0..3 {
            for col in 0..3 {
                if self.is_legal_action((row, col)) {
                    actions.push((row, col));
                }
            }
        }

        actions
    }

    pub fn print(&self, clear_screen: bool) {
        if clear_screen {
            // clear screen and reset cursor to 1-1
            print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        }
        
        println!("  a b c");

        for (i, row) in self.fields.iter().enumerate() {
            print!("{} ", i + 1);

            for cell in row {
                match *cell {
                    Some(Player::X) => print!("{}x{} ", color::Fg(color::Green), color::Fg(color::Reset)),
                    Some(Player::O) => print!("{}o{} ", color::Fg(color::Yellow), color::Fg(color::Reset)),
                    None => print!(". "),
                };
            }

            println!();
        }
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "Board {{")?;
        writeln!(f, "    Next player: {}", self.next_player)?;
        writeln!(f, "    Board:")?;
        writeln!(f, "      a b c")?;

        for (i, row) in self.fields.iter().enumerate() {
            write!(f, "    {} ", i + 1)?;

            for cell in row {
                match *cell {
                    Some(Player::X) => write!(f, "x ")?,
                    Some(Player::O) => write!(f, "o ")?,
                    None => write!(f, ". ")?,
                };
            }

            writeln!(f)?;
        }

        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_opponent() {
        let player_x = Player::X;
        let player_o = Player::O;
        assert_eq!(player_x.opponent(), player_o);
        assert_eq!(player_o.opponent(), player_x);
    }

    #[test]
    fn check_new_empty_board() {
        let board = Board::new(Player::X);
        assert_eq!(board.fields, [
            [None, None, None],
            [None, None, None],
            [None, None, None]
        ]);
    }

    #[test]
    fn check_get_actions() {
        let mut board = Board::new(Player::X);
        board.fields = [
            [None,              Some(Player::X),     Some(Player::O)],
            [Some(Player::X),   None,                Some(Player::X)],
            [Some(Player::O),   Some(Player::X),     None]
        ];
        let manual = vec![(0,0), (1,1), (2,2)]; // row and column
        assert_eq!(board.get_actions(), manual);
    }


    #[test]
    fn check_is_legal_action() {
        let mut board = Board::new(Player::X);
        board.fields = [
            [None,               Some(Player::X),     Some(Player::O)],
            [Some(Player::X),    None,                Some(Player::X)],
            [Some(Player::O),    Some(Player::X),     None]
        ];
        let mut action = (4, 3);
        assert!(!board.is_legal_action(action));
        action = (1, 1);
        assert!(board.is_legal_action(action));
    }

    #[test]
    fn check_perform_action() {
        let mut board = Board::new(Player::X);
        board.fields = [
            [None,              Some(Player::X),     Some(Player::O)],
            [Some(Player::X),   None,                Some(Player::X)],
            [Some(Player::O),   Some(Player::X),     None]
        ];
        let action = (2, 2);
        board.perform_action(action);

        let mut board2 = Board::new(Player::X);
        board2.fields = [
            [None,              Some(Player::X),     Some(Player::O)],
            [Some(Player::X),   None,                Some(Player::X)],
            [Some(Player::O),   Some(Player::X),     Some(Player::X)]
        ];

        assert_eq!(board.fields, board2.fields);
    }

    #[test]
    fn check_board_cloning() {
        let mut board = Board::new(Player::X);
        board.fields = [
            [None,              Some(Player::X),     Some(Player::O)],
            [Some(Player::X),   None,                Some(Player::X)],
            [Some(Player::O),   Some(Player::X),     None]
        ];

        let board2 = board.clone();
        assert_eq!(board.fields, board2.fields);
    }

    #[test]
    fn check_is_ended_winner() {
        let mut board = Board::new(Player::X);
        board.fields = [
            [None,              Some(Player::X),     Some(Player::O)],
            [Some(Player::X),   Some(Player::X),     Some(Player::X)],
            [Some(Player::O),   Some(Player::O),     None]
        ];
        assert!(board.is_ended()); // X Won

    }

    #[test]
    fn check_is_ended_no_moves() {
        let mut board = Board::new(Player::X);

        board.fields = [
            [Some(Player::O),   Some(Player::X),     Some(Player::O)],
            [Some(Player::X),   Some(Player::O),     Some(Player::X)],
            [Some(Player::X),   Some(Player::O),     Some(Player::X)]
        ];
        assert!(board.is_ended()); // No more fields available with None

    }

    #[test]
    fn check_get_winner() {
        let mut board = Board::new(Player::O);
        board.fields = [
            [None,              Some(Player::X),     Some(Player::O)],
            [Some(Player::X),   Some(Player::X),     Some(Player::X)],
            [Some(Player::O),   Some(Player::O),     None]
        ];

        let mut manual_winner = Player::X;
        assert_eq!(board.get_winner().unwrap(), manual_winner); 

        board.fields = [
            [None,              None,                None],
            [None,              None,                None],
            [Some(Player::O),   Some(Player::O),     Some(Player::O)]
        ];

        manual_winner = Player::O;
        assert_eq!(board.get_winner().unwrap(), manual_winner); 

    }
    
    
}
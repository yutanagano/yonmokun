use std::cmp::Ordering;
use std::fmt;
use std::ops::{AddAssign, Neg};


pub struct Position {
    pub active_player: Player,
    board: [[[Slot; 4]; 4]; 4],
    num_moves_played: u8
}

impl Position {
    pub fn new() -> Self {
        let board = [[[Slot::Empty; 4]; 4]; 4];
        let active_player = Player::White;
        let num_moves_played = 0;
        Position{board, active_player, num_moves_played}
    }

    pub fn print(&self) {
        print!("\n");
        for floor in (0..4).rev() {
            for row in (0..4).rev() {
                println!("{} {}  {}  {}  {}", row+1, self.board[floor][0][row], self.board[floor][1][row], self.board[floor][2][row], self.board[floor][3][row]);
            }
            println!("  1  2  3  4\n");
        }
    }

    pub fn get_static_evaluation(&self) -> Evaluation {
        let mut heuristic_score = 0;

        match self.evaluate_straight_lines() {
            Evaluation::HeuristicScore(s) => heuristic_score += s,
            terminal_eval => return terminal_eval
        }

        match self.evaluate_first_degree_diagonals() {
            Evaluation::HeuristicScore(s) => heuristic_score += s,
            terminal_eval => return terminal_eval
        }

        match self.evaluate_second_degree_diagonals() {
            Evaluation::HeuristicScore(s) => heuristic_score += s,
            terminal_eval => return terminal_eval
        }

        if self.num_moves_played == 64 {
            return Evaluation::Draw
        };

        Evaluation::HeuristicScore(heuristic_score)
    }

    fn evaluate_straight_lines(&self) -> Evaluation {
        let mut heuristic_score = 0;

        for fixed_index_1 in 0..4 {
            for fixed_index_2 in 0..4 {
                for axis in 0..3 {
                    let mut linestate = LineState::Uncontrolled;

                    for moving_index in 0..4 {
                        linestate += match axis {
                            0 => self.board[fixed_index_1][fixed_index_2][moving_index], // along columns
                            1 => self.board[fixed_index_1][moving_index][fixed_index_2], // along rows
                            2 => self.board[moving_index][fixed_index_1][fixed_index_2], // along floors
                            _ => panic!("Something went wrong.")
                        }
                    }

                    match linestate {
                        LineState::Completed => return Evaluation::Loss,
                        LineState::Controlled { controlling_player, degree } => match controlling_player == self.active_player {
                            true => heuristic_score += degree,
                            false => heuristic_score -= degree
                        }
                        _ => ()
                    }
                }
            }
        }

        Evaluation::HeuristicScore(heuristic_score)
    }

    fn evaluate_first_degree_diagonals(&self) -> Evaluation {
        let mut heuristic_score = 0;

        for fixed_index in 0..4 {
            for fixed_axis in 0..3 {
                let mut linestate_1 = LineState::Uncontrolled;
                let mut linestate_2 = LineState::Uncontrolled;

                for moving_index in 0..4 {
                    linestate_1 += match fixed_axis {
                        0 => self.board[moving_index][moving_index][fixed_index],
                        1 => self.board[moving_index][fixed_index][moving_index],
                        2 => self.board[fixed_index][moving_index][moving_index],
                        _ => panic!("Something went wrong.")
                    };
                    linestate_2 += match fixed_axis {
                        0 => self.board[moving_index][3-moving_index][fixed_index],
                        1 => self.board[moving_index][fixed_index][3-moving_index],
                        2 => self.board[fixed_index][moving_index][3-moving_index],
                        _ => panic!("Something went wrong.")
                    }
                }

                match linestate_1 {
                    LineState::Completed => return Evaluation::Loss,
                    LineState::Controlled { controlling_player, degree } => match controlling_player == self.active_player {
                        true => heuristic_score += degree,
                        false => heuristic_score -= degree
                    },
                    _ => ()
                }

                match linestate_2 {
                    LineState::Completed => return Evaluation::Loss,
                    LineState::Controlled { controlling_player, degree } => match controlling_player == self.active_player {
                        true => heuristic_score += degree,
                        false => heuristic_score -= degree
                    },
                    _ => ()
                }
            }
        }

        Evaluation::HeuristicScore(heuristic_score)
    }

    fn evaluate_second_degree_diagonals(&self) -> Evaluation {
        let mut heuristic_score = 0;

        let mut linestate_1 = LineState::Uncontrolled;
        let mut linestate_2 = LineState::Uncontrolled;
        let mut linestate_3 = LineState::Uncontrolled;
        let mut linestate_4 = LineState::Uncontrolled;

        for moving_index in 0..4 {
            linestate_1 += self.board[moving_index][moving_index][moving_index];
            linestate_2 += self.board[moving_index][moving_index][3-moving_index];
            linestate_3 += self.board[moving_index][3-moving_index][moving_index];
            linestate_4 += self.board[moving_index][3-moving_index][3-moving_index];
        }

        match linestate_1 {
            LineState::Completed => return Evaluation::Loss,
            LineState::Controlled { controlling_player, degree } => match controlling_player == self.active_player {
                true => heuristic_score += degree,
                false => heuristic_score -= degree
            },
            _ => ()
        }

        match linestate_2 {
            LineState::Completed => return Evaluation::Loss,
            LineState::Controlled { controlling_player, degree } => match controlling_player == self.active_player {
                true => heuristic_score += degree,
                false => heuristic_score -= degree
            },
            _ => ()
        }

        match linestate_3 {
            LineState::Completed => return Evaluation::Loss,
            LineState::Controlled { controlling_player, degree } => match controlling_player == self.active_player {
                true => heuristic_score += degree,
                false => heuristic_score -= degree
            },
            _ => ()
        }

        match linestate_4 {
            LineState::Completed => return Evaluation::Loss,
            LineState::Controlled { controlling_player, degree } => match controlling_player == self.active_player {
                true => heuristic_score += degree,
                false => heuristic_score -= degree
            },
            _ => ()
        }

        Evaluation::HeuristicScore(heuristic_score)
    }

    pub fn play(&self, coordinates: Coordinates) -> Position {
        if !self.can_play(coordinates) {
            panic!("Cannot play {}, {}", coordinates.file, coordinates.rank)
        };

        let mut new_board = self.board.clone();
        for floor in 0..4 {
            if new_board[floor][coordinates.file][coordinates.rank] == Slot::Empty {
                new_board[floor][coordinates.file][coordinates.rank] = Slot::Occupied(self.active_player);
                break;
            }
        }

        let new_active_player = match self.active_player {
            Player::White => Player::Black,
            Player::Black => Player::White
        };

        Position { board: new_board, active_player: new_active_player, num_moves_played: self.num_moves_played+1 }
    }

    pub fn can_play(&self, coordinates: Coordinates) -> bool {
        self.board[3][coordinates.file][coordinates.rank] == Slot::Empty
    }

    pub fn is_terminal(&self) -> bool {
        match self.get_static_evaluation() {
            Evaluation::HeuristicScore(_) => false,
            _ => true
        }
    }

    pub fn generate_moves(&self) -> Vec<Coordinates> {
        let mut playable_coords = Vec::new();

        for file in 0..4 {
            for rank in 0..4 {
                let coords = Coordinates::new(file, rank);
                if self.can_play(coords) {
                    playable_coords.push(coords);
                };
            }
        }

        playable_coords.sort_by(
            |a,b| -> Ordering {
                let position_a = self.play(*a);
                let position_b = self.play(*b);
                position_a.get_static_evaluation().cmp(&position_b.get_static_evaluation())
            }
        );

        playable_coords
    }
}


#[derive(Clone, Copy, PartialEq)]
enum Slot {
    Occupied(Player),
    Empty
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Slot::Empty => return write!(f, " "),
            Slot::Occupied(p) => match p {
                Player::White => return write!(f, "○"),
                Player::Black => return write!(f, "●")
            }
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    White,
    Black
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Evaluation {
    Win,
    Loss,
    Draw,
    HeuristicScore(i8)
}

impl Evaluation {
    pub fn to_confidence(&self) -> f32 {
        match self {
            Evaluation::Win => 1.0,
            Evaluation::Loss => 0.0,
            Evaluation::Draw => 0.5,
            Evaluation::HeuristicScore(s) => {
                let exponent = *s as f32 / 5.0;
                let base: f32 = 2.0;
                let denominator = 1.0 + base.powf(-exponent);
                1.0 / denominator
            }
        }
    }
}

impl Neg for Evaluation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Evaluation::Win => Evaluation::Loss,
            Evaluation::Loss => Evaluation::Win,
            Evaluation::Draw => Evaluation::Draw,
            Evaluation::HeuristicScore(s) => Evaluation::HeuristicScore(-s)
        }
    }
}

impl PartialOrd for Evaluation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Evaluation {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal
        };

        match self {
            Evaluation::Win => Ordering::Greater,
            Evaluation::Loss => Ordering::Less,
            Evaluation::Draw => match other {
                Evaluation::Win => Ordering::Less,
                Evaluation::Loss => Ordering::Greater,
                Evaluation::HeuristicScore(s) => {
                    if *s > 0 {
                        Ordering::Less
                    } else if *s < 0 {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                },
                _ => panic!("Something went wrong.")
            },
            Evaluation::HeuristicScore(s) => match other {
                Evaluation::Win => Ordering::Less,
                Evaluation::Loss => Ordering::Greater,
                Evaluation::Draw => {
                    if *s > 0 {
                        Ordering::Greater
                    } else if *s < 0 {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                },
                Evaluation::HeuristicScore(os) => s.cmp(os)
            }
        }
    }
}


enum LineState {
    Uncontrolled,
    Controlled { controlling_player: Player, degree: i8 },
    Completed,
    Plugged
}

impl AddAssign<Slot> for LineState {
    fn add_assign(&mut self, rhs: Slot) {
        let player = match rhs {
            Slot::Empty => return,
            Slot::Occupied(p) => p
        };

        match self {
            LineState::Uncontrolled => *self = LineState::Controlled { controlling_player: player, degree: 1 },
            LineState::Controlled { controlling_player, degree } => match *controlling_player == player {
                true => {
                    if *degree == 3 {
                        *self = LineState::Completed
                    } else {
                        *self = LineState::Controlled { controlling_player: *controlling_player, degree: *degree+1 }
                    }
                },
                false => *self = LineState::Plugged
            },
            LineState::Plugged => return,
            LineState::Completed => panic!("You can't add slots to a completed line.")
        }
    }
}


#[derive(Clone, Copy)]
pub struct Coordinates {
    pub file: usize,
    pub rank: usize
}

impl Coordinates {
    pub fn new(file: usize, rank: usize) -> Self {
        Coordinates { file, rank }
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.file+1, self.rank+1)
    }
}


#[cfg(test)]
mod tests {
    use super::{Position, Coordinates, Evaluation, Player};

    #[test]
    fn test_position_evolution() {
        let mut position = Position::new();

        position = position.play(Coordinates::new(0, 0));

        assert_eq!(position.get_static_evaluation(), Evaluation::HeuristicScore(-7));
        assert_eq!(position.active_player, Player::Black);

        position = position.play(Coordinates::new(0, 3));
        position = position.play(Coordinates::new(3, 0));
        position = position.play(Coordinates::new(3, 3));
        position = position.play(Coordinates::new(1, 0));
        position = position.play(Coordinates::new(2, 0));

        assert_eq!(position.get_static_evaluation(), Evaluation::HeuristicScore(-2));
        assert_eq!(position.active_player, Player::White);
    }
}

use crate::position::{Position, Evaluation, Coordinates};
use std::time::{Duration, Instant};


pub fn analyse(position: &Position, depth: u8) -> AnalysisReport {
    if position.is_terminal() {
        panic!("Cannot play on a terminal state.")
    };

    let start_time = Instant::now();

    let mut num_positions_traversed_including_root = 1;
    let mut best_eval_so_far = Evaluation::Loss;
    let mut best_move_so_far = None;

    for coordinates in position.generate_moves() {
        let new_position = position.play(coordinates);
        let evaluation = -get_negamax_evaluation(&new_position, depth, &mut num_positions_traversed_including_root, Evaluation::Loss, -best_eval_so_far);

        if best_move_so_far.is_none() {
            best_move_so_far = Some(coordinates);
        };

        if evaluation > best_eval_so_far {
            best_eval_so_far = evaluation;
            best_move_so_far = Some(coordinates);
        }
    }

    AnalysisReport{
        evaluation: best_eval_so_far,
        best_move: best_move_so_far.unwrap(),
        search_time: start_time.elapsed(),
        num_positions_traversed: num_positions_traversed_including_root
    }
}


fn get_negamax_evaluation(position: &Position, depth: u8, num_positions_traversed_so_far: &mut u32, mut alpha: Evaluation, beta: Evaluation) -> Evaluation {
    *num_positions_traversed_so_far += 1;

    let static_evaluation = position.get_static_evaluation();

    match static_evaluation {
        Evaluation::Win => Evaluation::Win,
        Evaluation::Loss => Evaluation::Loss,
        Evaluation::Draw => Evaluation::Draw,
        Evaluation::HeuristicScore(_) => {
            if depth == 0 {
                return static_evaluation
            };

            for coordinates in position.generate_moves() {
                let new_position = position.play(coordinates);
                let evaluation = -get_negamax_evaluation(&new_position, depth-1, num_positions_traversed_so_far, -beta, -alpha);

                if evaluation >= beta {
                    return evaluation
                };

                if evaluation >= alpha {
                    alpha = evaluation
                };
            }

            alpha
        }
    }
}


pub struct AnalysisReport {
    pub evaluation: Evaluation,
    pub best_move: Coordinates,
    pub search_time: Duration,
    pub num_positions_traversed: u32
}


struct _TranspositionTableEntry {
    pub position: u128,
    pub evaluation_upper_bound: Evaluation,
    pub evaluation_lower_bound: Evaluation,
    pub search_depth: u8,
}

use crate::position::{Position, Evaluation};


pub fn get_best_move(position: &Position, depth: u8) -> (usize, usize) {
    if position.is_terminal() {
        panic!("Cannot play on a terminal state.")
    };

    let mut best_eval_so_far = Evaluation::Loss;
    let mut best_move_so_far = (0,0);

    for file in 0..4 {
        for rank in 0..4 {
            if !position.can_play(file, rank) {
                continue
            };

            let new_position = position.play(file, rank);
            let evaluation = -get_negamax_evaluation(&new_position, depth, Evaluation::Loss, -best_eval_so_far);

            if evaluation > best_eval_so_far {
                best_eval_so_far = evaluation;
                best_move_so_far = (file, rank);
            }
        }
    }

    best_move_so_far
}


pub fn get_negamax_evaluation(position: &Position, depth: u8, mut alpha: Evaluation, beta: Evaluation) -> Evaluation {
    let static_evaluation = position.get_static_evaluation();

    match static_evaluation {
        Evaluation::Win => Evaluation::Win,
        Evaluation::Loss => Evaluation::Loss,
        Evaluation::Draw => Evaluation::Draw,
        Evaluation::HeuristicScore(_) => {
            if depth == 0 {
                return static_evaluation
            };

            for file in 0..4 {
                for rank in 0..4 {
                    if !position.can_play(file, rank) {continue};

                    let new_position = position.play(file, rank);
                    let evaluation = -get_negamax_evaluation(&new_position, depth-1, -beta, -alpha);

                    if evaluation >= beta {
                        return evaluation
                    };

                    if evaluation >= alpha {
                        alpha = evaluation
                    };
                }
            }

            alpha
        }
    }
}

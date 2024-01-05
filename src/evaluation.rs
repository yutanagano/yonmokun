use crate::position::{Position, Evaluation, Coordinates};


pub fn get_best_move(position: &Position, depth: u8) -> (Coordinates, Evaluation) {
    if position.is_terminal() {
        panic!("Cannot play on a terminal state.")
    };

    let mut best_eval_so_far = Evaluation::Loss;
    let mut best_move_so_far = None;

    for file in 0..4 {
        for rank in 0..4 {
            let coordinates = Coordinates::new(file, rank);

            if !position.can_play(coordinates) {
                continue
            };

            let new_position = position.play(coordinates);
            let evaluation = -get_negamax_evaluation(&new_position, depth, Evaluation::Loss, -best_eval_so_far);

            if evaluation > best_eval_so_far {
                best_eval_so_far = evaluation;
                best_move_so_far = Some(coordinates);
            }
        }
    }

    (best_move_so_far.unwrap(), best_eval_so_far)
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
                    let coordinates = Coordinates::new(file, rank);

                    if !position.can_play(coordinates) {continue};

                    let new_position = position.play(coordinates);
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

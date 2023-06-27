use crate::game::KalahaState;
pub fn negamax(kalaha_state: &KalahaState, alpha: i32, beta: i32, depth: u8) -> i32 {
    if depth == 0 || kalaha_state.game_is_over() {
        return kalaha_state.evaluate();
    }
    let mut best_eval = -i32::MAX;
    let mut generated_sequences = kalaha_state.generate_move_sequence_results();
    generated_sequences.sort_by(|a, b| b.last_moves.len().cmp(&a.last_moves.len()));
    for game in generated_sequences {
        let eval = -negamax(&game, beta, alpha, depth - 1);
        let alpha = alpha.max(eval);

        if eval >= best_eval {
            best_eval = eval;
        }
        if alpha >= beta {
            break;
        }
    }
    best_eval
}
pub fn negamax_search(kalaha_state: &KalahaState, depth: u8) -> (Option<Vec<usize>>, i32) {
    let mut best_move = None;
    let mut best_eval = -i32::MAX;
    let mut alpha = -i32::MAX;
    let beta = i32::MAX;
    for game in &kalaha_state.generate_move_sequence_results() {
        let eval = -negamax(game, -beta, -alpha, depth - 1);
        if eval >= best_eval {
            best_eval = eval;
            best_move = Some(game.last_moves.clone());
        }
        alpha = alpha.max(eval);
    }
    (best_move, best_eval)
}

use crate::game::KalahaState;
pub fn negamax(kalaha_state: &KalahaState, depth: i32) -> (i32, Option<Vec<usize>>) {
    if depth == 0 || kalaha_state.game_is_over() {
        return (kalaha_state.evaluate(), None);
    }
    let mut best_eval = i32::MIN;
    let mut best_moves = None;
    for game in &kalaha_state.generate_move_sequence_results() {
        let (eval, _) = negamax(game, depth - 1);
        let eval = -eval;
        if eval > best_eval {
            best_eval = eval;
            best_moves = Some(game.last_moves.clone());
        }
    }
    (best_eval, best_moves)
}

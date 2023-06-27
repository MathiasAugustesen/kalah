use crate::game::KalahaState;
pub fn negamax(
    kalaha_state: &KalahaState,
    depth: u8,
    alpha: i32,
    beta: i32,
) -> (i32, Option<Vec<usize>>) {
    if depth == 0 || kalaha_state.game_is_over() {
        return (kalaha_state.evaluate(), None);
    }
    let mut best_eval = -i32::MAX;
    let mut best_moves = None;
    let mut generated_sequences = kalaha_state.generate_move_sequence_results();
    generated_sequences.sort_by(|a, b| a.last_moves.len().cmp(&b.last_moves.len()));
    for game in generated_sequences {
        let (eval, _) = negamax(&game, depth - 1, -beta, -alpha);
        let eval = -eval;
        let alpha = alpha.max(eval);
        if alpha >= beta {
            break;
        }
        if eval >= best_eval {
            best_eval = eval;
            best_moves = Some(game.last_moves);
        }
    }
    (best_eval, best_moves)
}

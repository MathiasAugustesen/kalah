use crate::game::KalahaState;
pub fn negamax(
    kalaha_state: &KalahaState,
    alpha: &mut i32,
    beta: &mut i32,
    depth: u8,
) -> (i32, Option<Vec<usize>>) {
    if depth == 0 || kalaha_state.game_is_over() {
        return (kalaha_state.evaluate(), None);
    }
    let mut best_eval = -i32::MAX;
    let mut best_moves = None;
    let mut generated_sequences = kalaha_state.generate_move_sequence_results();
    generated_sequences.sort_by(|a, b| b.last_moves.len().cmp(&a.last_moves.len()));
    for game in generated_sequences {
        let (eval, _) = negamax(&game, &mut -*beta, &mut -*alpha, depth - 1);
        let mut eval = -eval;
        *alpha = *alpha.max(&mut eval);

        if eval >= best_eval {
            best_eval = eval;
            best_moves = Some(game.last_moves.clone());
        }
        if alpha >= beta {
            break;
        }
    }
    (best_eval, best_moves)
}

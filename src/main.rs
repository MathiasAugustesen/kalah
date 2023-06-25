use engine::negamax;
use game::KalahaState;
mod engine;
mod game;
fn main() {
    let mut game = KalahaState::new_game();
    let (eval, moves) = negamax(&game, 2);
    println!("{} {:?}", eval, moves);
    let mut count = 0;
    let continuations = game.generate_move_sequence_results();
    for continuation in continuations {
        count += 1;
        println!("{}", continuation);
    }
    println!("count: {}", count);
}

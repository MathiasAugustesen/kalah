use engine::negamax;
use game::KalahaState;
mod engine;
mod game;
fn main() {
    KalahaState::ai_vs_ai(9);
}

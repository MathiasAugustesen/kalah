use engine::negamax;
use game::KalahaState;
mod engine;
mod game;
fn main() {
    KalahaState::player_vs_ai(10, game::Player::Almuta);
}

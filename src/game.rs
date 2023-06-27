use PitKind::*;
use Player::*;

use crate::engine::negamax;
#[derive(Debug, Clone)]
pub struct KalahaState {
    pub to_play: Player,
    pub board: [Pit; 14],
    pub game_state: GameState,
    pub last_moves: Vec<usize>,
    pub switched_turn: bool,
    pub turns_played: u8,
}
#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Playing,
    GameOver(Option<Player>),
}
impl KalahaState {
    pub fn new_game() -> KalahaState {
        let board = new_board();
        KalahaState {
            to_play: Almuta,
            board,
            game_state: GameState::Playing,
            last_moves: Vec::new(),
            switched_turn: true,
            turns_played: 0,
        }
    }
    pub fn play_moves(&mut self, moves: Vec<usize>) {
        if moves.is_empty() {
            self.snatch_seeds();
        }
        for mov in moves {
            self.play_move(mov);
        }
    }
    pub fn play_move(&mut self, pit_index: usize) {
        if self.switched_turn {
            self.last_moves = Vec::new();
            self.switched_turn = false
        }
        self.last_moves.push(pit_index);
        let same_player_to_play = self.distribute_seeds(pit_index);
        if !same_player_to_play {
            self.to_play = self.to_play.opposite();
            self.switched_turn = true;
            self.turns_played += 1;
            if self.end_by_majority_seeds() {
                self.resolve_game()
            }
        }
    }
    fn end_by_majority_seeds(&self) -> bool {
        self.stash_seeds(Almuta) > 36 || self.stash_seeds(Batal) > 36
    }
    pub fn generate_move_sequence_results(&self) -> Vec<KalahaState> {
        let valid_moves = self.valid_moves();
        if valid_moves.is_empty() {
            let mut game_end = self.clone();
            game_end.snatch_seeds();
            return vec![game_end];
        }
        let mut sequence_results = Vec::new();
        for mov in valid_moves {
            let mut new_kalaha_state = self.clone();
            new_kalaha_state.play_move(mov);
            if !new_kalaha_state.switched_turn {
                sequence_results.append(&mut new_kalaha_state.generate_move_sequence_results());
            } else {
                sequence_results.push(new_kalaha_state);
            }
        }
        sequence_results
    }
    // Distributes the seeds and returns whether it is the same player's turn
    fn distribute_seeds(&mut self, pit_index: usize) -> bool {
        let selected_pit = &mut self.board[pit_index];
        let mut seeds_to_distribute = std::mem::take(&mut selected_pit.value);
        let mut deposit_pit_index = pit_index;

        while seeds_to_distribute > 0 {
            deposit_pit_index = (deposit_pit_index + 1) % 14;
            let deposit_pit = &mut self.board[deposit_pit_index];
            let seed_can_be_deposited_legally = match self.to_play {
                Almuta => deposit_pit.kind != BatalStash,
                Batal => deposit_pit.kind != AlmutaStash,
            };

            if seed_can_be_deposited_legally {
                deposit_pit.add_seed(&mut seeds_to_distribute);
            }
        }
        let final_deposit_pit = self.board[deposit_pit_index];
        if seeds_to_distribute == 0
            && final_deposit_pit.value == 1
            && final_deposit_pit.is_player_pit(self.to_play)
        {
            self.steal_opposing_seeds(deposit_pit_index);
        }

        match self.board[deposit_pit_index].kind {
            AlmutaStash | BatalStash => true,
            _ => false,
        }
    }
    pub fn valid_moves(&self) -> Vec<usize> {
        let slice = self.player_pits_slice(self.to_play);
        slice
            .into_iter()
            .filter(|&idx| self.board[idx].value != 0)
            .collect()
    }
    fn snatch_seeds(&mut self) {
        let slice = self.player_pits_slice(self.to_play.opposite());
        let target_pits = &self.board[slice];
        let snatched_seeds: i32 = target_pits.iter().map(|x| x.value).sum();

        let player_stash = self.stash_seeds_mut(self.to_play);

        *player_stash += snatched_seeds;

        self.resolve_game();
    }
    fn steal_opposing_seeds(&mut self, pit_index: usize) {
        let opposing_pit_index = 12 - pit_index;
        let stolen_seeds = std::mem::take(&mut self.board[opposing_pit_index].value);
        *self.stash_seeds_mut(self.to_play) += stolen_seeds;
    }
    fn player_pits_slice(&self, player: Player) -> std::ops::Range<usize> {
        match player {
            Almuta => 0..6,
            Batal => 7..13,
        }
    }
    fn stash_index(&self, player: Player) -> usize {
        match player {
            Almuta => 6,
            Batal => 13,
        }
    }
    fn stash_seeds(&self, player: Player) -> i32 {
        let index = self.stash_index(player);
        self.board[index].value
    }
    fn stash_seeds_mut(&mut self, player: Player) -> &mut i32 {
        let index = self.stash_index(player);
        &mut self.board[index].value
    }
    fn resolve_game(&mut self) {
        let new_game_state = match self
            .stash_seeds(self.to_play)
            .cmp(&self.stash_seeds(self.to_play.opposite()))
        {
            std::cmp::Ordering::Greater => GameState::GameOver(Some(Almuta)),
            std::cmp::Ordering::Equal => GameState::GameOver(None),
            std::cmp::Ordering::Less => GameState::GameOver(Some(Batal)),
        };
        self.game_state = new_game_state;
    }
    pub fn evaluate(&self) -> i32 {
        let absolute_evaluation = match self.game_state {
            GameState::Playing => {
                let almuta_points = self.stash_seeds(Almuta);
                let batal_points = self.stash_seeds(Batal);
                almuta_points - batal_points
            }
            GameState::GameOver(winner) => match winner {
                None => 0,
                Some(Almuta) => i32::MAX - self.turns_played as i32,
                Some(Batal) => -i32::MAX + self.turns_played as i32,
            },
        };
        absolute_evaluation * self.to_play.relative_factor()
    }
    pub fn game_is_over(&self) -> bool {
        self.game_state != GameState::Playing
    }
    pub fn ai_vs_ai(depth: u8) {
        let mut game = Self::new_game();
        loop {
            if game.game_is_over() {
                println!("{:?}", game.game_state);
                return;
            }
            let (eval, moves) = negamax(&game, depth, -i32::MAX, i32::MAX);
            if let Some(moves) = moves {
                println!("{}", &game);
                println!("{:?}", &moves);
                game.play_moves(moves);
                println!("eval is now {}", eval);
            } else {
                game.snatch_seeds();
                let winner = game.game_state;
                println!("{:?}", winner);
                return;
            }
        }
    }
    pub fn player_vs_ai(depth: u8, player: Player) {
        let mut game = Self::new_game();
        while !game.game_is_over() {
            while game.to_play == player {
                let valid_moves = game.valid_moves();
                if valid_moves.is_empty() {
                    game.snatch_seeds();
                    let winner = game.game_state;
                    println!("{:?}", winner);
                    return;
                }
                let player_move = Self::get_player_move(&valid_moves);
                game.play_move(player_move);
                println!("Current board position: \n{}", &game);
            }
            let (eval, moves) = negamax(&game, depth, -i32::MAX, i32::MAX);
            if let Some(moves) = moves {
                println!("The AI has chosen to play the moves {:?}", &moves);
                game.play_moves(moves);
                println!("eval is now {}", eval);
            } else {
                game.snatch_seeds();
                let winner = game.game_state;
                println!("{:?}", winner);
                return;
            }
            println!("Current board position: \n{}", &game);
        }
    }
    fn get_player_move(valid_moves: &[usize]) -> usize {
        loop {
            let mut input_string = String::new();
            std::io::stdin()
                .read_line(&mut input_string)
                .expect("Failed to read line");

            let input: usize = match input_string.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Input was not a valid usize");
                    continue;
                }
            };
            if valid_moves.contains(&input) {
                return input;
            } else {
                continue;
            }
        }
    }
}
pub fn new_board() -> [Pit; 14] {
    [
        Pit::new(AlmutaPit),
        Pit::new(AlmutaPit),
        Pit::new(AlmutaPit),
        Pit::new(AlmutaPit),
        Pit::new(AlmutaPit),
        Pit::new(AlmutaPit),
        Pit::new(AlmutaStash),
        Pit::new(BatalPit),
        Pit::new(BatalPit),
        Pit::new(BatalPit),
        Pit::new(BatalPit),
        Pit::new(BatalPit),
        Pit::new(BatalPit),
        Pit::new(BatalStash),
    ]
}
impl std::fmt::Display for KalahaState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let almuta_pits = &self.board[0..6];
        let almuta_stash = self.board[6];
        let batal_pits = &self.board[7..13];
        let batal_stash = self.board[13];
        for pit in batal_pits.iter().rev() {
            write!(f, " {:<3}", pit.value)?
        }
        writeln!(f)?;
        writeln!(f, "{}                 {}", batal_stash, almuta_stash)?;
        for pit in almuta_pits.iter() {
            write!(f, " {:<3}", pit.value)?
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pit {
    pub value: i32,
    pub kind: PitKind,
}
impl Pit {
    pub fn new(kind: PitKind) -> Pit {
        match kind {
            AlmutaPit | BatalPit => Pit { value: 6, kind },
            AlmutaStash | BatalStash => Pit { value: 0, kind },
        }
    }
    pub fn add_seed(&mut self, seeds_to_distribute: &mut i32) {
        self.value += 1;
        *seeds_to_distribute -= 1;
    }
    fn is_player_pit(&self, player: Player) -> bool {
        match player {
            Almuta => self.kind == AlmutaPit,
            Batal => self.kind == BatalPit,
        }
    }
}
impl std::fmt::Display for Pit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " {} ", self.value)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PitKind {
    AlmutaStash,
    AlmutaPit,
    BatalStash,
    BatalPit,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Almuta,
    Batal,
}
impl Player {
    pub fn opposite(self) -> Player {
        match self {
            Almuta => Batal,
            Batal => Almuta,
        }
    }
    pub fn relative_factor(self) -> i32 {
        match self {
            Almuta => 1,
            Batal => -1,
        }
    }
}

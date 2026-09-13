use serde::{Serialize, Deserialize};

/// Jogadores X / O
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn opponent(self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

/// Célula do tabuleiro
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Cell {
    Empty,
    X,
    O,
}

/// Resultado do jogo
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameResult {
    InProgress,
    Win(Player),
    Draw,
}

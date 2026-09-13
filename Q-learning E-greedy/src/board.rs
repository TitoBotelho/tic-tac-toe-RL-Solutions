use crate::types::*;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Board {
    cells: [Cell; 9],
    pub to_move: Player,
}

impl Board {
    /// Tabuleiro vazio, X começa
    pub fn new() -> Self {
        Self {
            cells: [Cell::Empty; 9],
            to_move: Player::X,
        }
    }

    pub fn cells(&self) -> &[Cell; 9] {
        &self.cells
    }

    pub fn legal_actions(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if *c == Cell::Empty { Some(i) } else { None })
            .collect()
    }

    pub fn apply_action(&mut self, action: usize) {
        if action < 9 && self.cells[action] == Cell::Empty {
            self.cells[action] = match self.to_move {
                Player::X => Cell::X,
                Player::O => Cell::O,
            };
            self.to_move = self.to_move.opponent();
        }
    }

    pub fn result(&self) -> GameResult {
        let lines = [
            (0, 1, 2),
            (3, 4, 5),
            (6, 7, 8),
            (0, 3, 6),
            (1, 4, 7),
            (2, 5, 8),
            (0, 4, 8),
            (2, 4, 6),
        ];

        for &(a, b, c) in &lines {
            if self.cells[a] != Cell::Empty && self.cells[a] == self.cells[b] && self.cells[b] == self.cells[c] {
                return match self.cells[a] {
                    Cell::X => GameResult::Win(Player::X),
                    Cell::O => GameResult::Win(Player::O),
                    _ => GameResult::InProgress,
                };
            }
        }

        if self.cells.iter().all(|c| *c != Cell::Empty) {
            GameResult::Draw
        } else {
            GameResult::InProgress
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..3 {
            for c in 0..3 {
                let ch = match self.cells[r * 3 + c] {
                    Cell::Empty => '.',
                    Cell::X => 'X',
                    Cell::O => 'O',
                };
                write!(f, "{}", ch)?;
                if c < 2 {
                    write!(f, " ")?;
                }
            }
            if r < 2 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

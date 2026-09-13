use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::board::Board;
use crate::types::{Cell, GameResult, Player};

type StateKey = Vec<(usize, Cell)>;

#[derive(Serialize, Deserialize)]
struct SerializableVTable {
    entries: Vec<(StateKey, f64)>,
}

pub struct TDAgent {
    v_table: HashMap<StateKey, f64>,
    learning_rate: f64,
    epsilon: f64,
}

impl TDAgent {
    pub fn new(learning_rate: f64, epsilon: f64) -> Self {
        Self {
            v_table: HashMap::new(),
            learning_rate,
            epsilon,
        }
    }

    pub fn save_v_table(&self, path: &str) {
        let serializable = SerializableVTable {
            entries: self.v_table.iter().map(|(k, v)| (k.clone(), *v)).collect(),
        };
        let json = serde_json::to_string_pretty(&serializable).expect("Erro ao serializar V-table");
        fs::write(path, json).expect("Erro ao salvar V-table");
    }

    pub fn load_v_table(&mut self, path: &str) -> bool {
        if !Path::new(path).exists() {
            return false;
        }
        let data = fs::read_to_string(path).expect("Erro ao ler arquivo V-table");
        let serializable: SerializableVTable = serde_json::from_str(&data).expect("Erro ao desserializar V-table");
        self.v_table = serializable.entries.into_iter().collect();
        true
    }

    pub fn v_table_size(&self) -> usize {
        self.v_table.len()
    }

    fn state_key(board: &Board) -> StateKey {
        board.cells().iter().enumerate().map(|(i, c)| (i, *c)).collect()
    }

    fn get_value(&mut self, board: &Board) -> f64 {
        let key = Self::state_key(board);
        *self.v_table.entry(key).or_insert(0.0)
    }

    fn is_terminal(board: &Board) -> bool {
        !matches!(board.result(), GameResult::InProgress)
    }

    fn action_value(&mut self, board: &Board, action: usize) -> f64 {
        let mut next = board.clone();
        next.apply_action(action);
        if Self::is_terminal(&next) {
            match next.result() {
                GameResult::Win(p) => if p == Player::X { 1.0 } else { -1.0 },
                GameResult::Draw => 0.0,
                _ => 0.0,
            }
        } else {
            self.get_value(&next)
        }
    }

    pub fn choose_action(&mut self, board: &Board) -> usize {
        let legal = board.legal_actions();

        if rand::random::<f64>() < self.epsilon {
            let idx = (rand::random::<f64>() * legal.len() as f64) as usize;
            return legal[idx];
        }

        let mut best_action = legal[0];
        let mut best_value = f64::NEG_INFINITY;

        for &a in &legal {
            let v = self.action_value(board, a);
            if v > best_value {
                best_value = v;
                best_action = a;
            }
        }

        best_action
    }

    pub fn values_for_legal_actions(&mut self, board: &Board) -> Vec<f64> {
        let legal = board.legal_actions();
        legal.iter().map(|&a| self.action_value(board, a)).collect()
    }

    pub fn update(&mut self, board: &Board, reward: f64, next_board: &Board) {
        let key = Self::state_key(board);

        let current_v = if let Some(&v) = self.v_table.get(&key) {
            v
        } else {
            self.v_table.insert(key.clone(), 0.0);
            0.0
        };

        let target = if Self::is_terminal(next_board) {
            reward
        } else {
            let nk = Self::state_key(next_board);
            if let Some(&v) = self.v_table.get(&nk) {
                v
            } else {
                self.v_table.insert(nk, 0.0);
                0.0
            }
        };

        let new_v = current_v + self.learning_rate * (target - current_v);
        self.v_table.insert(key, new_v);
    }
}

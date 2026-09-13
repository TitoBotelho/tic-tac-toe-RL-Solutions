use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::board::Board;
use crate::types::Cell;

type StateKey = Vec<(usize, Cell)>;

#[derive(Serialize, Deserialize)]
struct SerializableQTable {
    entries: Vec<(StateKey, Vec<f64>)>,
}

pub struct QAgent {
    q_table: HashMap<StateKey, Vec<f64>>,
    learning_rate: f64,
    discount_factor: f64,
    epsilon: f64,
}

impl QAgent {
    pub fn new(learning_rate: f64, discount_factor: f64, epsilon: f64) -> Self {
        Self {
            q_table: HashMap::new(),
            learning_rate,
            discount_factor,
            epsilon,
        }
    }

    pub fn save_q_table(&self, path: &str) {
        let serializable = SerializableQTable {
            entries: self.q_table.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
        };
        let json = serde_json::to_string_pretty(&serializable).expect("Erro ao serializar Q-table");
        fs::write(path, json).expect("Erro ao salvar Q-table");
    }

    pub fn load_q_table(&mut self, path: &str) -> bool {
        if !Path::new(path).exists() {
            return false;
        }
        let data = fs::read_to_string(path).expect("Erro ao ler arquivo Q-table");
        let serializable: SerializableQTable = serde_json::from_str(&data).expect("Erro ao desserializar Q-table");
        self.q_table = serializable.entries.into_iter().collect();
        true
    }

    pub fn q_table_size(&self) -> usize {
        self.q_table.len()
    }

    fn state_key(board: &Board) -> StateKey {
        board.cells().iter().enumerate().map(|(i, c)| (i, *c)).collect()
    }

    pub fn get_q_values(&mut self, board: &Board) -> Vec<f64> {
        let key = Self::state_key(board);
        let legal = board.legal_actions();
        let n = legal.len();

        if !self.q_table.contains_key(&key) {
            let q_init = 1.0 / n as f64;
            self.q_table.insert(key.clone(), vec![q_init; n]);
        }

        self.q_table[&key].clone()
    }

    pub fn choose_action(&mut self, board: &Board) -> usize {
        let legal = board.legal_actions();
        let q_values = self.get_q_values(board);

        if rand::random::<f64>() < self.epsilon {
            let idx = (rand::random::<f64>() * legal.len() as f64) as usize;
            legal[idx]
        } else {
            let mut best_idx = 0;
            for i in 1..q_values.len() {
                if q_values[i] > q_values[best_idx] {
                    best_idx = i;
                }
            }
            legal[best_idx]
        }
    }

    pub fn update(&mut self, board: &Board, action: usize, reward: f64, next_board: &Board) {
        let key = Self::state_key(board);
        let next_key = Self::state_key(next_board);
        let legal = board.legal_actions();
        let action_idx = legal.iter().position(|&a| a == action).unwrap();

        if !self.q_table.contains_key(&key) {
            let q_init = 1.0 / legal.len() as f64;
            self.q_table.insert(key.clone(), vec![q_init; legal.len()]);
        }

        let next_legal = next_board.legal_actions();
        let next_done = next_legal.is_empty();

        if !self.q_table.contains_key(&next_key) && !next_done {
            let q_init = 1.0 / next_legal.len() as f64;
            self.q_table.insert(next_key.clone(), vec![q_init; next_legal.len()]);
        }

        let max_next_q = if next_done {
            0.0
        } else {
            self.q_table[&next_key].iter().cloned().fold(f64::NEG_INFINITY, f64::max)
        };

        let current_q = self.q_table[&key][action_idx];
        let new_q = current_q + self.learning_rate * (reward + self.discount_factor * max_next_q - current_q);
        self.q_table.get_mut(&key).unwrap()[action_idx] = new_q;
    }

    pub fn get_q_table(&self) -> &HashMap<StateKey, Vec<f64>> {
        &self.q_table
    }

    pub fn get_probabilities(&self, board: &Board) -> Vec<f64> {
        let key = Self::state_key(board);
        if let Some(q_values) = self.q_table.get(&key) {
            let total: f64 = q_values.iter().sum();
            if total > 0.0 {
                q_values.iter().map(|q| q / total).collect()
            } else {
                vec![1.0 / q_values.len() as f64; q_values.len()]
            }
        } else {
            vec![]
        }
    }
}

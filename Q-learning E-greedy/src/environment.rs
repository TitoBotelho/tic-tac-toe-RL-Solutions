use crate::types::*;
use crate::board::Board;

/// Trait genérica para ambientes (MDP / RL)
pub trait Environment {
    type State: Clone + Eq + std::hash::Hash;
    type Action: Clone + Eq + std::hash::Hash;

    /// Estado inicial
    fn initial_state(&self) -> Self::State;

    /// Ações legais em um estado
    fn legal_actions(&self, state: &Self::State) -> Vec<Self::Action>;

    /// Aplica ação: retorna (next_state, reward, done)
    fn step(&self, state: &Self::State, action: &Self::Action) -> (Self::State, f32, bool);

    /// Estado terminal?
    fn is_terminal(&self, state: &Self::State) -> bool;
}

/// Implementação concreta para TicTacToe (estado = Board, ação = posição 0..8)
pub struct TicTacToeEnv {}

impl TicTacToeEnv {
    pub fn new() -> Self {
        Self {}
    }
}

impl Environment for TicTacToeEnv {
    type State = Board;
    type Action = usize;

    fn initial_state(&self) -> Self::State {
        Board::new()
    }

    fn legal_actions(&self, state: &Self::State) -> Vec<Self::Action> {
        state.legal_actions()
    }

    fn step(&self, state: &Self::State, action: &Self::Action) -> (Self::State, f32, bool) {
        let mut nb = state.clone();
        nb.apply_action(*action);
        let result = nb.result();
        let done = !matches!(result, GameResult::InProgress);

        // reward do ponto de vista do jogador que fez a ação (mover)
        let reward = match result {
            GameResult::Win(p) => {
                if p == state.to_move { 1.0 } else { -1.0 }
            }
            GameResult::Draw => 0.0,
            GameResult::InProgress => 0.0,
        };

        (nb, reward, done)
    }

    fn is_terminal(&self, state: &Self::State) -> bool {
        !matches!(state.result(), GameResult::InProgress)
    }
}

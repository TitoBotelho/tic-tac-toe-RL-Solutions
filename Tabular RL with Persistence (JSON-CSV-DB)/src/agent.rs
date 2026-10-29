//! Agente da seção 1.5 do Sutton & Barto.
//!
//! - Uma tabela V(s): "probabilidade estimada de ganhar a partir de s".
//! - Inicialização: vitória = 1, derrota/empate = 0, resto = 0.5.
//! - Jogada greedy: escolhe a que leva ao estado s' com maior V(s').
//! - Jogada exploratória (prob. epsilon): aleatória, e NÃO gera aprendizado.
//! - Aprendizado (só após greedy):  V(s) <- V(s) + alpha * [ V(s') - V(s) ]

use std::collections::HashMap;
use rand::Rng;
use crate::board::*;

pub struct Agent {
    pub v: HashMap<State, f64>,
    pub alpha: f64,   // passo de aprendizado
    pub epsilon: f64, // prob. de jogada exploratória
}

impl Agent {
    pub fn new(alpha: f64, epsilon: f64) -> Self {
        let mut v = HashMap::new();
        for state in generate_all_reachable_states() {
            let init = match winner(&state) {
                Some(X) => 1.0,
                Some(_) => 0.0,
                None if is_terminal(&state) => 0.0,
                None => 0.5,
            };
            v.insert(state, init);
        }

        Self { v, alpha, epsilon }
    }

    pub fn merge_persisted(&mut self, table: &HashMap<State, f64>) {
        for (state, value) in table {
            self.v.insert(*state, *value);
        }
    }

    /// Reduz a exploração e o passo de aprendizado ao longo do treinamento.
    ///
    /// A ideia é simples: no começo o agente explora muito para descobrir estados;
    /// depois, conforme o número de episódios aumenta, ele passa a confiar mais na
    /// política greedy e a ajustar valores com menos ruído.
    pub fn apply_schedule(&mut self, episode: usize, total_episodes: usize, epsilon_min: f64, alpha_min: f64) {
        if total_episodes == 0 {
            return;
        }

        let progress = episode as f64 / total_episodes as f64;
        let epsilon = 1.0 - (1.0 - epsilon_min) * progress;
        let alpha = 0.3 - (0.3 - alpha_min) * progress;

        self.epsilon = epsilon.max(epsilon_min);
        self.alpha = alpha.max(alpha_min);
    }

    /// V(s) com a inicialização do livro (cria a entrada se não existir).
    pub fn value(&mut self, s: &State) -> f64 {
        if let Some(&v) = self.v.get(s) {
            return v;
        }
        let init = match winner(s) {
            Some(X) => 1.0,                // ganhamos
            Some(_) => 0.0,                // perdemos
            None if is_terminal(s) => 0.0, // empate: o livro também dá 0
            None => 0.5,                   // desconhecido: 50%
        };
        self.v.insert(*s, init);
        init
    }

    /// Escolhe a jogada de X. Retorna (posição, foi_greedy).
    pub fn choose(&mut self, s: &State) -> (usize, bool) {
        let moves = compute_legal_moves(s);
        let mut rng = rand::thread_rng();

        // exploração
        if rng.gen::<f64>() < self.epsilon {
            let pos = moves[rng.gen_range(0..moves.len())];
            return (pos, false);
        }

        // greedy: argmax_a V( play(s, a) )
        let mut best_pos = moves[0];
        let mut best_v = f64::NEG_INFINITY;
        for &pos in &moves {
            let v = self.value(&play(s, pos, X));
            if v > best_v {
                best_v = v;
                best_pos = pos;
            }
        }
        (best_pos, true)
    }

    /// A única linha de aprendizado do capítulo 1.
    pub fn update(&mut self, s: &State, s_next: &State) {
        let v_next = self.value(s_next);
        let v = self.value(s);
        self.v.insert(*s, v + self.alpha * (v_next - v));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_persisted_keeps_full_state_table() {
        let mut agent = Agent::new(0.1, 0.1);
        let before = agent.v.len();

        let persisted = HashMap::from([([1, 0, 0, 0, 0, 0, 0, 0, 0], 0.75)]);
        agent.merge_persisted(&persisted);

        assert_eq!(agent.v.len(), before);
        assert_eq!(agent.v.get(&[1, 0, 0, 0, 0, 0, 0, 0, 0]), Some(&0.75));
        assert!(agent.v.contains_key(&empty_board()));
    }
}

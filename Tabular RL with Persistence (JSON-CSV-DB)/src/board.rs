//! Tabuleiro mínimo. Estado = 9 casas, cada uma EMPTY, X ou O.

use std::collections::HashSet;

pub const EMPTY: u8 = 0;
pub const X: u8 = 1;
pub const O: u8 = 2;

/// O estado é só um array de 9 bytes — serve direto como chave da tabela V.
pub type State = [u8; 9];

pub fn empty_board() -> State {
    [EMPTY; 9]
}

/// Casas vazias = jogadas possíveis.
pub fn compute_legal_moves(s: &State) -> Vec<usize> {
    (0..9).filter(|&i| s[i] == EMPTY).collect()
}

/// Retorna um novo estado com `player` marcado em `pos`.
pub fn play(s: &State, pos: usize, player: u8) -> State {
    let mut next = *s;
    next[pos] = player;
    next
}

/// Some(X) ou Some(O) se alguém venceu, None caso contrário.
pub fn winner(s: &State) -> Option<u8> {
    const LINES: [[usize; 3]; 8] = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8],
        [0, 3, 6], [1, 4, 7], [2, 5, 8],
        [0, 4, 8], [2, 4, 6],
    ];
    for [a, b, c] in LINES {
        if s[a] != EMPTY && s[a] == s[b] && s[b] == s[c] {
            return Some(s[a]);
        }
    }
    None
}

/// Jogo acabou? (vitória ou tabuleiro cheio)
pub fn is_terminal(s: &State) -> bool {
    winner(s).is_some() || compute_legal_moves(s).is_empty()
}

/// Conta quantas casas X e O existem no tabuleiro.
pub fn count_marks(s: &State) -> (usize, usize) {
    let mut x = 0;
    let mut o = 0;
    for &cell in s {
        match cell {
            X => x += 1,
            O => o += 1,
            _ => {}
        }
    }
    (x, o)
}

/// Gera a tabela completa de estados legais alcançáveis a partir do tabuleiro vazio.
///
/// A ideia aqui é cobrir todas as posições possíveis do jogo, sem depender de um treino
/// específico para “descobrir” um estado. Isso permite que o agente comece com uma base
/// completa e atualize apenas os valores que precisam ser refinados.
pub fn generate_all_reachable_states() -> Vec<State> {
    let mut states = Vec::new();
    let mut stack = vec![empty_board()];
    let mut seen = HashSet::new();

    while let Some(state) = stack.pop() {
        if !seen.insert(state) {
            continue;
        }

        states.push(state);

        if winner(&state).is_some() || compute_legal_moves(&state).is_empty() {
            continue;
        }

        let (x_count, o_count) = count_marks(&state);
        let next_player = if x_count == o_count { X } else { O };

        for pos in compute_legal_moves(&state) {
            stack.push(play(&state, pos, next_player));
        }
    }

    states.sort();
    states
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_state_table_includes_all_reachable_positions() {
        let states = generate_all_reachable_states();
        assert_eq!(states.len(), 5478);
        assert!(states.contains(&empty_board()));
        assert!(states.iter().any(|s| winner(s) == Some(X)));
    }
}

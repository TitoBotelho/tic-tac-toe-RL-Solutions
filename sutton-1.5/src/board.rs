//! Tabuleiro mínimo. Estado = 9 casas, cada uma EMPTY, X ou O.

pub const EMPTY: u8 = 0;
pub const X: u8 = 1;
pub const O: u8 = 2;

/// O estado é só um array de 9 bytes — serve direto como chave da tabela V.
pub type State = [u8; 9];

pub fn empty_board() -> State {
    [EMPTY; 9]
}

/// Casas vazias = jogadas possíveis.
pub fn legal_moves(s: &State) -> Vec<usize> {
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
    winner(s).is_some() || legal_moves(s).is_empty()
}

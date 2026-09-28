mod board;
mod agent;

use board::*;
use agent::Agent;
use rand::Rng;

/// Joga uma partida: X = agente, O = aleatório. Retorna o vencedor (ou None = empate).
fn play_episode(agent: &mut Agent) -> Option<u8> {
    let mut s = empty_board();

    // Estado resultante da ÚLTIMA jogada de X (ponto preto na Fig. 1.1 do livro),
    // e se aquela jogada foi greedy. Só fazemos backup se foi greedy.
    let mut prev: Option<State> = None;
    let mut prev_greedy = false;

    loop {
        // --- turno de X (agente) ---
        if is_terminal(&s) { break; }

        let (pos, greedy) = agent.choose(&s);
        s = play(&s, pos, X);

        // Backup: do estado após minha jogada ANTERIOR para o estado após minha jogada ATUAL.
        // V(prev) <- V(prev) + alpha [ V(s) - V(prev) ]
        if let (Some(p), true) = (prev, prev_greedy) {
            agent.update(&p, &s);
        }
        prev = Some(s);
        prev_greedy = greedy;

        if is_terminal(&s) { break; }

        // --- turno de O (aleatório) ---
        let moves = legal_moves(&s);
        let pos = moves[rand::thread_rng().gen_range(0..moves.len())];
        s = play(&s, pos, O);

        // Se O encerrou o jogo (venceu ou empatou), fazemos o backup final
        // do meu último estado direto para o terminal.
        if is_terminal(&s) {
            if prev_greedy { agent.update(&prev.unwrap(), &s); }
            break;
        }
    }

    winner(&s)
}

/// Imprime V(s') para cada uma das 9 aberturas possíveis de X no tabuleiro vazio.
/// É exatamente o que `choose` compara na primeira jogada.
fn print_opening_values(agent: &mut Agent, label: &str) {
    println!("{}", label);
    for row in 0..3 {
        let line: Vec<String> = (0..3)
            .map(|col| format!("{:.3}", agent.value(&play(&empty_board(), row * 3 + col, X))))
            .collect();
        println!("  {}", line.join("  "));
    }
}

fn main() {
    let mut agent = Agent::new(0.1, 0.1);
    let episodes = 20_000;
    let mut wins = 0;
    let mut losses = 0;
    let mut draws = 0;

    print_opening_values(&mut agent, "V das 9 aberturas ANTES do treino:");
    println!();

    for ep in 1..=episodes {
        match play_episode(&mut agent) {
            Some(X) => wins += 1,
            Some(O) => losses += 1,
            _ => draws += 1,
        }
        if ep % 2000 == 0 {
            println!(
                "ep {:>6} | X {:>5.1}%  O {:>5.1}%  empate {:>5.1}% | estados: {}",
                ep,
                100.0 * wins as f64 / ep as f64,
                100.0 * losses as f64 / ep as f64,
                100.0 * draws as f64 / ep as f64,
                agent.v.len()
            );
        }
    }

    println!("\nV(tabuleiro vazio) = {:.3}  (nunca atualizado: X ainda não jogou)", agent.value(&empty_board()));
    print_opening_values(&mut agent, "V das 9 aberturas DEPOIS do treino:");
}

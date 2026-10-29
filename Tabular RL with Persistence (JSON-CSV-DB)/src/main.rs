mod board;
mod agent;
mod persistence;

use std::fs;

use board::*;
use agent::Agent;
use persistence::{export_csv, export_json, export_sqlite, load_table_from_file};
use rand::Rng;

/// Joga uma partida: X = agente, O = aleatório. Retorna o vencedor (ou None = empate).
fn play_episode(agent: &mut Agent) -> Option<u8> {
    let mut s = empty_board();

    // Estado resultante da ÚLTIMA jogada de X (ponto preto na Fig. 1.1 do livro).
    // Como a exploração também gera aprendizado, não precisamos bloquear o update
    // apenas porque a ação foi exploratória.
    let mut prev: Option<State> = None;

    loop {
        // --- turno de X (agente) ---
        if is_terminal(&s) { break; }

        let (pos, _) = agent.choose(&s);
        s = play(&s, pos, X);

        // Backup: do estado após minha jogada ANTERIOR para o estado após minha jogada ATUAL.
        // No caso epsilon-greedy, a ação exploratória também gera transição e deve aprender.
        // V(prev) <- V(prev) + alpha [ V(s) - V(prev) ]
        if let Some(p) = prev {
            agent.update(&p, &s);
        }
        prev = Some(s);

        if is_terminal(&s) { break; }

        // --- turno de O (aleatório) ---
        let moves = compute_legal_moves(&s);
        let pos = moves[rand::thread_rng().gen_range(0..moves.len())];
        s = play(&s, pos, O);

        // Se O encerrou o jogo (venceu ou empatou), fazemos o backup final
        // do meu último estado direto para o terminal.
        if is_terminal(&s) {
            if let Some(p) = prev {
                agent.update(&p, &s);
            }
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

/// Sonda: fixa a 1ª jogada de X em `x1`, a resposta de O em `o1`, e imprime
/// V(s') para cada uma das 2ª jogadas possíveis de X (célula ocupada = "  -  ").
fn print_second_round_values(agent: &mut Agent, x1: usize, o1: usize, label: &str) {
    let s1 = play(&empty_board(), x1, X);
    let s2 = play(&s1, o1, O);
    println!("{}", label);
    for row in 0..3 {
        let line: Vec<String> = (0..3)
            .map(|col| {
                let pos = row * 3 + col;
                if s2[pos] != EMPTY {
                    "  -  ".to_string()
                } else {
                    format!("{:.3}", agent.value(&play(&s2, pos, X)))
                }
            })
            .collect();
        println!("  {}", line.join("  "));
    }
}

fn print_all_state_values(agent: &Agent, label: &str) {
    println!("{}", label);
    let mut entries: Vec<_> = agent.v.iter().collect();
    entries.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));
    for (state, value) in entries {
        println!("{:?} -> {:.3}", state, value);
    }
}

fn main() {
    let output_dir = "exports";
    fs::create_dir_all(output_dir).expect("Não foi possível criar a pasta de exportação");

    let json_path = format!("{}/v_table.json", output_dir);
    let csv_path = format!("{}/v_table.csv", output_dir);
    let db_path = format!("{}/v_table.db", output_dir);

    let mut agent = Agent::new(0.1, 0.1);

    let persisted = [json_path.clone(), csv_path.clone(), db_path.clone()]
        .iter()
        .find_map(|path| match load_table_from_file(path) {
            Ok(table) if !table.is_empty() => Some(table),
            Ok(_) => None,
            Err(err) => {
                eprintln!("Falha ao carregar {}: {}", path, err);
                None
            }
        });

    if let Some(table) = persisted {
        agent.merge_persisted(&table);
        println!(
            "Tabela persistida carregada com {} entradas; total atual da tabela: {} estados.",
            table.len(),
            agent.v.len()
        );
    } else {
        println!("Nenhuma tabela persistida encontrada; começando do zero.");
    }

    let episodes = 4_000_000;
    let epsilon_min = 0.01;
    let alpha_min = 0.05;
    let mut wins = 0;
    let mut losses = 0;
    let mut draws = 0;

    print_opening_values(&mut agent, "V das 9 aberturas ANTES do treino:");
    println!();

    for ep in 1..=episodes {
        agent.apply_schedule(ep, episodes, epsilon_min, alpha_min);

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

    println!();
    print_second_round_values(&mut agent, 0, 4, "\nX jogou canto(0), O respondeu centro(4) -> V da 2ª jogada de X:");
    print_second_round_values(&mut agent, 0, 1, "\nX jogou canto(0), O respondeu borda(1) -> V da 2ª jogada de X:");
    print_second_round_values(&mut agent, 0, 8, "\nX jogou canto(0), O respondeu canto oposto(8) -> V da 2ª jogada de X:");

    println!();
    print_all_state_values(&agent, "Todos os estados aprendidos depois do treino:");

    if let Err(err) = export_json(&json_path, &agent.v) {
        eprintln!("Falha ao exportar JSON: {}", err);
    }

    if let Err(err) = export_csv(&csv_path, &agent.v) {
        eprintln!("Falha ao exportar CSV: {}", err);
    }

    if let Err(err) = export_sqlite(&db_path, &agent.v) {
        eprintln!("Falha ao exportar SQLite: {}", err);
    }

    println!("\nArquivos exportados em '{}':", output_dir);
    println!("  - {}", json_path);
    println!("  - {}", csv_path);
    println!("  - {}", db_path);
}

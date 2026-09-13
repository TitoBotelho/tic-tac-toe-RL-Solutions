use tabular_td::{Board, TicTacToeEnv, TDAgent, Player, Environment};

const V_TABLE_PATH: &str = "v_table.json";

fn print_grid(values: &[f64], legal: &[usize], label: &str) {
    let mut grid: [Option<f64>; 9] = [None; 9];
    for (i, &pos) in legal.iter().enumerate() {
        grid[pos] = Some(values[i]);
    }
    println!("{}", label);
    for row in 0..3 {
        let mut line = String::from("|");
        for col in 0..3 {
            let idx = row * 3 + col;
            match grid[idx] {
                Some(v) => line.push_str(&format!(" {:5.2} |", v)),
                None => line.push_str("   --  |"),
            }
        }
        println!("{}", line);
    }
}

fn main() {
    let env = TicTacToeEnv::new();
    let mut agent = TDAgent::new(0.1, 0.3);

    let loaded = agent.load_v_table(V_TABLE_PATH);
    if loaded {
        println!("V-table carregada de '{}' ({} estados)", V_TABLE_PATH, agent.v_table_size());
    } else {
        println!("Nenhuma V-table encontrada, iniciando do zero");
    }

    let mut board = Board::new();
    let mut history: Vec<(Board, f64)> = Vec::new();

    println!("\n=== Jogo ===\n");

    loop {
        let legal = board.legal_actions();
        if legal.is_empty() {
            break;
        }

        if board.to_move == Player::X {
            println!("--- Turno de X ---");
            println!("Tabuleiro:\n{}\n", board);

            let v_next = agent.values_for_legal_actions(&board);
            print_grid(&v_next, &legal, "V(s') para cada ação:");
            println!("Ações legais: {:?}", legal);

            let action = agent.choose_action(&board);
            println!("X escolheu posição: {}\n", action);

            let (next_board, reward, done) = env.step(&board, &action);
            history.push((board.clone(), reward as f64));
            board = next_board;

            if done { break; }
        } else {
            println!("--- Turno de O ---");
            println!("Tabuleiro:\n{}", board);

            let o_action = legal[(rand::random::<f64>() * legal.len() as f64) as usize];
            println!("O (random) escolheu posição: {}\n", o_action);

            let (next_board, reward, done) = env.step(&board, &o_action);
            history.push((board.clone(), -(reward as f64)));
            board = next_board;

            if done { break; }
        }
    }

    let final_board = board.clone();
    let result = final_board.result();
    println!("=== Fim do jogo ===");
    println!("Tabuleiro final:\n{}\n", final_board);
    println!("Resultado: {:?}\n", result);

    println!("--- Atualização backwards ---");
    for i in (0..history.len()).rev() {
        let (state, reward) = &history[i];
        let next_board = if i + 1 < history.len() {
            &history[i + 1].0
        } else {
            &final_board
        };
        agent.update(state, *reward, next_board);

        let v_next = agent.values_for_legal_actions(state);
        let legal = state.legal_actions();
        let mut grid: [Option<f64>; 9] = [None; 9];
        for (j, &pos) in legal.iter().enumerate() {
            grid[pos] = Some(v_next[j]);
        }
        println!("\nEstado {} (turno de {}):", i, if state.to_move == Player::X { "X" } else { "O" });
        for row in 0..3 {
            let mut line = String::from("|");
            for col in 0..3 {
                let idx = row * 3 + col;
                match grid[idx] {
                    Some(v) => line.push_str(&format!(" {:5.2} |", v)),
                    None => line.push_str("   --  |"),
                }
            }
            println!("{}", line);
        }
    }

    agent.save_v_table(V_TABLE_PATH);
    println!("\nV-table salva em '{}' ({} estados)", V_TABLE_PATH, agent.v_table_size());
}

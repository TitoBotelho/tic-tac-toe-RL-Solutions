use tictactoe_core::{Board, TicTacToeEnv, QAgent, Player, Environment};
use tictactoe_core::types::GameResult;

const Q_TABLE_PATH: &str = "q_table.json";

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
    let mut agent = QAgent::new(0.1, 0.9, 0.3);

    let loaded = agent.load_q_table(Q_TABLE_PATH);
    if loaded {
        println!("Q-table carregada de '{}' ({} estados)", Q_TABLE_PATH, agent.q_table_size());
    } else {
        println!("Nenhuma Q-table encontrada, iniciando do zero");
    }

    let mut board = Board::new();
    let mut history: Vec<(Board, usize, f64)> = Vec::new();

    println!("\n=== Jogo ===\n");

    loop {
        let legal = board.legal_actions();
        if legal.is_empty() {
            break;
        }

        if board.to_move == Player::X {
            println!("--- Turno de X ---");
            println!("Tabuleiro:\n{}\n", board);

            let q_values = agent.get_q_values(&board);
            let probs = agent.get_probabilities(&board);

            print_grid(&q_values, &legal, "Q values:");
            print_grid(&probs, &legal, "Probabilidades:");
            println!("Ações legais: {:?}", legal);

            let action = agent.choose_action(&board);
            println!("X escolheu posição: {}\n", action);

            let (next_board, reward, done) = env.step(&board, &action);
            history.push((board.clone(), action, reward as f64));
            board = next_board;

            if done { break; }
        } else {
            println!("--- Turno de O ---");
            println!("Tabuleiro:\n{}", board);

            let action = legal[(rand::random::<f64>() * legal.len() as f64) as usize];
            println!("O (random) escolheu posição: {}\n", action);

            let (next_board, _reward, done) = env.step(&board, &action);
            board = next_board;

            if done { break; }
        }
    }

    let result = board.result();
    let final_reward = match result {
        GameResult::Win(Player::X) => 1.0,
        GameResult::Win(Player::O) => -1.0,
        GameResult::Draw => 0.0,
        GameResult::InProgress => 0.0,
    };

    println!("=== Fim do jogo ===");
    println!("Tabuleiro final:\n{}\n", board);
    println!("Resultado: {:?}", result);
    println!("Recompensa final para X: {}\n", final_reward);

    println!("--- Atualização backwards ---");
    for (i, (state, action, _)) in history.iter().enumerate().rev() {
        let reward = if i == history.len() - 1 { final_reward } else { 0.0 };
        let next_board = if i + 1 < history.len() {
            history[i + 1].0.clone()
        } else {
            board.clone()
        };
        agent.update(state, *action, reward, &next_board);

        let q_values = agent.get_q_values(state);
        let legal = state.legal_actions();
        let mut grid: [Option<f64>; 9] = [None; 9];
        for (j, &pos) in legal.iter().enumerate() {
            grid[pos] = Some(q_values[j]);
        }
        println!("\nEstado {} (action={}):", i, action);
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

    agent.save_q_table(Q_TABLE_PATH);
    println!("\nQ-table salva em '{}' ({} estados)", Q_TABLE_PATH, agent.q_table_size());
}

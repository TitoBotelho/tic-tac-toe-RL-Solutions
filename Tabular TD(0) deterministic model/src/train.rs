use tabular_td::{Board, TicTacToeEnv, TDAgent, Player, Environment};
use std::time::Instant;

const V_TABLE_PATH: &str = "v_table.json";

fn main() {
    let mut agent = TDAgent::new(0.1, 0.3);
    agent.load_v_table(V_TABLE_PATH);
    let start_size = agent.v_table_size();

    let episodes = 10_000;
    let start = Instant::now();
    let mut results = [0usize; 3]; // X wins, O wins, draws

    for ep in 0..episodes {
        let env = TicTacToeEnv::new();
        let mut board = Board::new();
        let mut history: Vec<(Board, f64)> = Vec::new();

        loop {
            let legal = board.legal_actions();
            if legal.is_empty() { break; }

            if board.to_move == Player::X {
                let action = agent.choose_action(&board);
                let (next_board, reward, done) = env.step(&board, &action);
                history.push((board.clone(), reward as f64));
                board = next_board;
                if done { break; }
            } else {
                let idx = (rand::random::<f64>() * legal.len() as f64) as usize;
                let (next_board, reward, done) = env.step(&board, &legal[idx]);
                history.push((board.clone(), -(reward as f64)));
                board = next_board;
                if done { break; }
            }
        }

        let final_board = board.clone();
        let result = final_board.result();
        match result {
            tabular_td::types::GameResult::Win(p) => {
                if p == Player::X { results[0] += 1; } else { results[1] += 1; }
            }
            tabular_td::types::GameResult::Draw => { results[2] += 1; }
            _ => {}
        }

        for i in (0..history.len()).rev() {
            let (state, reward) = &history[i];
            let next_board = if i + 1 < history.len() {
                &history[i + 1].0
            } else {
                &final_board
            };
            agent.update(state, *reward, next_board);
        }

        if (ep + 1) % 1000 == 0 {
            eprint!(".");
        }
    }

    agent.save_v_table(V_TABLE_PATH);

    println!("\n\n=== Resultados após {} episódios ===", episodes);
    println!("Duração: {:.2?}", start.elapsed());
    println!("V-table: {} → {} estados ({} novos)", start_size, agent.v_table_size(), agent.v_table_size() - start_size);
    println!("Vitórias X: {} ({:.1}%)", results[0], 100.0 * results[0] as f64 / episodes as f64);
    println!("Vitórias O: {} ({:.1}%)", results[1], 100.0 * results[1] as f64 / episodes as f64);
    println!("Empates:    {} ({:.1}%)", results[2], 100.0 * results[2] as f64 / episodes as f64);
}

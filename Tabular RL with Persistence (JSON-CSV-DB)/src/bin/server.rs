use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::State as AxumState;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

#[allow(dead_code)]
#[path = "../board.rs"]
mod board;
#[allow(dead_code)]
#[path = "../persistence.rs"]
mod persistence;

use board::{compute_legal_moves, count_marks, is_terminal, play, winner, State, O, X};
use persistence::load_table_from_file;

#[derive(Clone)]
struct AppState {
    v_table: Arc<HashMap<State, f64>>,
}

#[derive(Debug, Deserialize)]
struct AiMoveRequest {
    state: State,
    ai_player: Option<u8>,
}

#[derive(Debug, Serialize)]
struct AiMoveResponse {
    ai_move: usize,
    next_state: State,
    winner: Option<u8>,
    terminal: bool,
    value: f64,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
    loaded_states: usize,
}

#[tokio::main]
async fn main() {
    let table = load_persisted_table();
    let loaded_states = table.len();

    let app_state = AppState {
        v_table: Arc::new(table),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/ai-move", post(ai_move))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state((app_state, loaded_states));

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("127.0.0.1:{}", port)
        .parse()
        .expect("endereco invalido");
    println!("API pronta em http://{}", addr);
    println!("Tabela carregada com {} estados.", loaded_states);

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!(
                "Nao foi possivel abrir {} ({:?}). Dica: finalize o processo antigo ou rode com PORT=8081 cargo run --bin server",
                addr, err
            );
            return;
        }
    };

    axum::serve(listener, app).await.expect("erro no servidor");
}

async fn health(AxumState((state, loaded_states)): AxumState<(AppState, usize)>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        loaded_states: loaded_states.max(state.v_table.len()),
    })
}

async fn ai_move(
    AxumState((state, _)): AxumState<(AppState, usize)>,
    Json(payload): Json<AiMoveRequest>,
) -> Result<Json<AiMoveResponse>, (StatusCode, Json<ErrorResponse>)> {
    validate_state(&payload.state)?;

    if winner(&payload.state).is_some() || is_terminal(&payload.state) {
        return Err(error(
            StatusCode::BAD_REQUEST,
            "partida ja terminou para esse estado",
        ));
    }

    let ai_player = payload.ai_player.unwrap_or(O);
    if ai_player != X && ai_player != O {
        return Err(error(
            StatusCode::BAD_REQUEST,
            "ai_player deve ser 1 (X) ou 2 (O)",
        ));
    }

    let (x_count, o_count) = count_marks(&payload.state);
    let turn_ok = if ai_player == X {
        x_count == o_count
    } else {
        x_count == o_count + 1
    };

    if !turn_ok {
        let expected = if ai_player == X {
            "esperado X = O"
        } else {
            "esperado X = O + 1"
        };

        return Err(error(
            StatusCode::BAD_REQUEST,
            &format!("estado invalido para turno da IA: {}", expected),
        ));
    }

    let legal_moves = compute_legal_moves(&payload.state);
    if legal_moves.is_empty() {
        return Err(error(StatusCode::BAD_REQUEST, "sem jogadas legais"));
    }

    let mut best_move = legal_moves[0];
    let mut best_value = if ai_player == O {
        f64::INFINITY
    } else {
        f64::NEG_INFINITY
    };

    for &pos in &legal_moves {
        let next = play(&payload.state, pos, ai_player);
        let value = state_value(&state.v_table, &next);

        let is_better = if ai_player == O {
            value < best_value
        } else {
            value > best_value
        };

        if is_better {
            best_value = value;
            best_move = pos;
        }
    }

    let next_state = play(&payload.state, best_move, ai_player);
    let response = AiMoveResponse {
        ai_move: best_move,
        winner: winner(&next_state),
        terminal: is_terminal(&next_state),
        next_state,
        value: state_value(&state.v_table, &next_state),
    };

    Ok(Json(response))
}

fn state_value(v_table: &HashMap<State, f64>, s: &State) -> f64 {
    if let Some(v) = v_table.get(s) {
        return *v;
    }

    match winner(s) {
        Some(X) => 1.0,
        Some(_) => 0.0,
        None if is_terminal(s) => 0.0,
        None => 0.5,
    }
}

fn load_persisted_table() -> HashMap<State, f64> {
    let candidates = ["exports/v_table.json", "exports/v_table.csv", "exports/v_table.db"];

    for path in candidates {
        match load_table_from_file(path) {
            Ok(table) if !table.is_empty() => return table,
            Ok(_) => {}
            Err(err) => eprintln!("Falha ao carregar {}: {}", path, err),
        }
    }

    HashMap::new()
}

fn validate_state(state: &State) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    for &cell in state {
        if cell > 2 {
            return Err(error(
                StatusCode::BAD_REQUEST,
                "estado invalido: valores permitidos sao 0, 1 e 2",
            ));
        }
    }
    Ok(())
}

fn error(status: StatusCode, message: &str) -> (StatusCode, Json<ErrorResponse>) {
    (
        status,
        Json(ErrorResponse {
            error: message.to_string(),
        }),
    )
}

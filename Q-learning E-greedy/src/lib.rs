//! tictactoe-core: representação do jogo e traits para MDP/RL
//!
//! Conteúdo mínimo: tipos, tabuleiro e trait genérica de Environment.

pub mod types;
pub mod board;
pub mod environment;
pub mod q_agent;

pub use types::*;
pub use board::Board;
pub use environment::{Environment, TicTacToeEnv};
pub use q_agent::QAgent;

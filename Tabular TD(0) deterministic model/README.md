Esqueleto do crate `tictactoe-core`.

Contém:
- tipos básicos (`Player`, `Cell`, `GameResult`)
- `Board` com regras e utilitários
- trait genérica `Environment` e implementação `TicTacToeEnv`

Próximos passos sugeridos:
- criar workspace Cargo e adicionar crates `tictactoe-mdp`, `tictactoe-rl`, `tictactoe-sim`
- escrever testes unitários em `tictactoe-core/src` (verificar `result`, `legal_actions`)
- ajustar serialização ou representações compactas conforme necessário

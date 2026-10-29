# Tabular RL with Persistence (JSON-CSV-DB)

Projeto base de Tic-Tac-Toe com RL tabular (estilo Sutton 1.5) preparado para persistencia e exportacao de tabela de valor `V(s)`.

## O que ja vem pronto

- Base do agente/tabuleiro copiada de `sutton-1.5`.
- Modulo `src/persistence.rs` com funcoes para:
  - Exportar JSON (`export_json`)
  - Importar JSON (`import_json`)
  - Exportar CSV (`export_csv`)
  - Exportar SQLite (`export_sqlite`)

## Dependencias

- `rand`
- `serde`, `serde_json`
- `csv`
- `rusqlite` (com `bundled`)

## Rodar

```bash
cargo run
```

## Rodar API para o Front-end

```bash
cargo run --bin server
```

API local: http://127.0.0.1:8080

Endpoint principal:
- POST /api/ai-move

Exemplo de corpo JSON:

```json
{
  "state": [1, 0, 0, 0, 0, 0, 0, 0, 0],
  "ai_player": 2
}
```

## Proximos passos sugeridos

1. Carregar a tabela no inicio (`import_json`).
2. Salvar no final do treino (`export_json`).
3. Adicionar flag/arg para exportar CSV e SQLite ao fim de cada execucao.
4. Opcional: checkpoints periodicos por numero de episodios.

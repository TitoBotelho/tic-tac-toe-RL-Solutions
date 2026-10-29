# Documentacao do Projeto - Tic-Tac-Toe RL Solutions

Este repositorio contem varias implementacoes de aprendizado por reforco para Jogo da Velha, alem de um frontend para jogar contra uma IA treinada.

## Estrutura principal

- Q-learning E-greedy: experimento de Q-learning com estrategia epsilon-greedy.
- sutton-1.5: implementacao baseada na secao 1.5 de Sutton & Barto.
- Tabular TD(0) deterministic model: abordagem TD(0) com persistencia de V(s).
- tictactoe-core: nucleo compartilhado com regras de tabuleiro e ambiente.
- Tabular RL with Persistence (JSON-CSV-DB): projeto principal com treino, persistencia e API.
- front-end-jogo-da-velha: interface web para jogar contra a IA treinada.

## Projeto principal (treino + API)

Pasta: Tabular RL with Persistence (JSON-CSV-DB)

### O que existe

- Treino tabular de estados V(s) com persistencia.
- Exportacao e importacao em JSON, CSV e SQLite.
- Tabela completa de estados legais alcancaveis (5478 estados).
- Servidor HTTP para inferencia de jogadas (IA vs humano).

### Binarios

- `cargo run`: executa o fluxo de treino/exportacao.
- `cargo run --bin server`: sobe a API da IA.

### API

Base local padrao: http://127.0.0.1:8080

Endpoints:

- GET `/health`
  - Resposta: status da API e quantidade de estados carregados.
- POST `/api/ai-move`
  - Corpo JSON:

```json
{
  "state": [0, 0, 0, 0, 0, 0, 0, 0, 0],
  "ai_player": 1
}
```

Legenda do estado:

- 0 = vazio
- 1 = X
- 2 = O

Resposta (exemplo):

```json
{
  "ai_move": 4,
  "next_state": [0,0,0,0,1,0,0,0,0],
  "winner": null,
  "terminal": false,
  "value": 0.7047
}
```

## Frontend (jogar contra a IA)

Pasta: front-end-jogo-da-velha

- Interface em HTML/CSS/JS.
- IA joga com X e inicia a partida.
- Humano joga com O.
- Fluxo correto: IA (X) -> Humano (O) -> IA (X) -> ...

## Como rodar localmente

### 1) Subir backend (Terminal A)

```bash
cd "Tabular RL with Persistence (JSON-CSV-DB)"
cargo run --bin server
```

### 2) Subir frontend estatico (Terminal B)

```bash
cd "front-end-jogo-da-velha"
python3 -m http.server 5500
```

### 3) Abrir no navegador

- http://127.0.0.1:5500

## Troubleshooting

### Porta 8080 ja em uso

Se aparecer erro de porta ocupada:

- Opcao 1: encerrar processo antigo usando a porta 8080.
- Opcao 2: subir a API em outra porta:

```bash
PORT=8081 cargo run --bin server
```

Se usar outra porta, atualize `API_BASE_URL` no frontend (`front-end-jogo-da-velha/app.js`).

### IA parece jogar com simbolo errado

Verifique se o frontend esta atualizado com o mapeamento:

- IA = X = 1
- Humano = O = 2

E force recarga do navegador (Ctrl+Shift+R).

## Observacoes de desenvolvimento

- O backend usa CORS aberto para facilitar desenvolvimento local.
- Persistencia e leitura da tabela treinada procuram, nesta ordem:
  - `exports/v_table.json`
  - `exports/v_table.csv`
  - `exports/v_table.db`
- Caso nao exista tabela persistida, a API sobe com tabela vazia e fallback de valor padrao por estado.

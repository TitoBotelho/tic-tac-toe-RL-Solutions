# Front-end Jogo da Velha

Interface para jogar contra a IA treinada via API Rust.

## Arquivos
- index.html: tela principal.
- app.js: logica do jogo e chamada da API em http://127.0.0.1:8080.

## Como rodar

1. Suba o backend (Terminal A):

```bash
cd "../Tabular RL with Persistence (JSON-CSV-DB)"
cargo run --bin server
```

2. Suba o frontend estatico (Terminal B):

```bash
cd "../front-end-jogo-da-velha"
python3 -m http.server 5500
```

3. Abra no navegador:

- http://127.0.0.1:5500

## Observacao

Se o backend estiver fora do ar, o frontend usa fallback local com jogada aleatoria da IA.

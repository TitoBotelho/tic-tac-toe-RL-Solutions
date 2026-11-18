# Tic-Tac-Toe RL Solutions

<div align="center">

## [🎮 Jogar contra a IA](https://titobotelho.github.io/tic-tac-toe-RL-Solutions/)

</div>

Este repositório reúne implementações de aprendizado por reforço aplicadas ao Jogo da Velha, com experimentos em Rust, persistência de tabelas de valores e uma interface web.

## Projetos

- [`Q-learning E-greedy`](./Q-learning%20E-greedy): implementação de Q-learning com estratégia epsilon-greedy.
- [`sutton-1.5`](./sutton-1.5): implementação baseada na seção 1.5 de Sutton & Barto.
- [`Tabular TD(0) deterministic model`](./Tabular%20TD(0)%20deterministic%20model): abordagem TD(0) determinística com tabela de estados.
- [`tictactoe-core`](./tictactoe-core): núcleo compartilhado com regras do tabuleiro e ambiente.
- [`Tabular RL with Persistence (JSON-CSV-DB)`](./Tabular%20RL%20with%20Persistence%20(JSON-CSV-DB)): projeto principal de treino, persistência, API e exportação.
- [`front-end-jogo-da-velha`](./front-end-jogo-da-velha): interface web conectada ao backend Rust.
- [`tictactoe-static-demo`](./tictactoe-static-demo): versão estática publicada no GitHub Pages.

## Demonstração

A demonstração utiliza a tabela treinada diretamente no navegador:

**https://titobotelho.github.io/tic-tac-toe-RL-Solutions/**

A IA joga como X e inicia a partida. O humano joga como O.

## Documentação

Consulte [`DOCUMENTACAO_PROJETO.md`](./DOCUMENTACAO_PROJETO.md) para arquitetura, execução local, API e troubleshooting.

# Demonstração estática do Jogo da Velha

Versão independente que usa a tabela treinada em `v_table.json`. Nesta demonstração:

- o humano joga com O;
- a IA joga com X;
- a IA faz a jogada inicial;
- não há backend Rust nem servidor de API.

## Executar localmente

Na pasta desta demonstração:

```bash
python3 -m http.server 5500
```

Abra:

```text
http://127.0.0.1:5500
```

A requisição ao JSON exige execução por servidor HTTP. Abrir o `index.html` diretamente pode bloquear o carregamento da tabela.

## Publicar no GitHub Pages

1. Envie esta pasta para um repositório GitHub.
2. Abra **Settings > Pages**.
3. Em **Build and deployment**, selecione **Deploy from a branch**.
4. Escolha o branch e a pasta raiz ou a pasta que contém esta demonstração.
5. Acesse a URL fornecida pelo GitHub Pages.

## Demonstração online

- https://titobotelho.github.io/tic-tac-toe-RL-Solutions/

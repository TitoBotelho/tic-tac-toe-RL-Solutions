const boardEl = document.getElementById("board");
const statusEl = document.getElementById("status");
const resetBtn = document.getElementById("resetBtn");

const EMPTY = 0;
const X = 1;
const O = 2;

let state = Array(9).fill(EMPTY);
let table = new Map();
let gameOver = false;
let aiThinking = true;

function checkWinner(cells) {
  const lines = [
    [0, 1, 2], [3, 4, 5], [6, 7, 8],
    [0, 3, 6], [1, 4, 7], [2, 5, 8],
    [0, 4, 8], [2, 4, 6],
  ];

  for (const [a, b, c] of lines) {
    if (cells[a] !== EMPTY && cells[a] === cells[b] && cells[b] === cells[c]) {
      return cells[a];
    }
  }
  return null;
}

function availableMoves(cells) {
  return cells
    .map((value, index) => ({ value, index }))
    .filter((entry) => entry.value === EMPTY)
    .map((entry) => entry.index);
}

function play(cells, position, player) {
  const next = [...cells];
  next[position] = player;
  return next;
}

function stateKey(cells) {
  return JSON.stringify(cells);
}

function stateValue(cells) {
  const value = table.get(stateKey(cells));
  if (value !== undefined) return value;

  const winner = checkWinner(cells);
  if (winner === X) return 1.0;
  if (winner === O) return 0.0;
  if (availableMoves(cells).length === 0) return 0.0;
  return 0.5;
}

function updateStatus() {
  const winner = checkWinner(state);
  if (winner) {
    gameOver = true;
    statusEl.textContent = winner === O ? "Você venceu!" : "IA venceu!";
    return;
  }

  if (availableMoves(state).length === 0) {
    gameOver = true;
    statusEl.textContent = "Empate!";
    return;
  }

  statusEl.textContent = aiThinking ? "IA pensando..." : "Sua vez (O).";
}

function renderBoard() {
  boardEl.innerHTML = "";

  state.forEach((value, index) => {
    const button = document.createElement("button");
    button.className = "cell";
    button.type = "button";
    button.setAttribute("role", "gridcell");
    button.setAttribute("aria-label", `Casa ${index + 1}`);

    if (value === X) {
      button.textContent = "X";
      button.classList.add("x");
    } else if (value === O) {
      button.textContent = "O";
      button.classList.add("o");
    }

    button.disabled = gameOver || aiThinking || value !== EMPTY;
    button.addEventListener("click", () => handleHumanMove(index));
    boardEl.appendChild(button);
  });
}

function makeAiMove() {
  if (gameOver) return;

  aiThinking = true;
  updateStatus();
  renderBoard();

  const moves = availableMoves(state);
  if (moves.length === 0) {
    aiThinking = false;
    updateStatus();
    renderBoard();
    return;
  }

  let bestMove = moves[0];
  let bestValue = -Infinity;

  for (const move of moves) {
    const next = play(state, move, X);
    const value = stateValue(next);
    if (value > bestValue) {
      bestValue = value;
      bestMove = move;
    }
  }

  state = play(state, bestMove, X);
  aiThinking = false;
  updateStatus();
  renderBoard();
}

function handleHumanMove(index) {
  if (gameOver || aiThinking || state[index] !== EMPTY) return;

  state = play(state, index, O);
  aiThinking = false;
  updateStatus();
  renderBoard();

  if (gameOver) return;

  window.setTimeout(makeAiMove, 180);
}

async function loadTable() {
  const response = await fetch("v_table.json");
  if (!response.ok) {
    throw new Error(`Não foi possível carregar v_table.json (${response.status})`);
  }

  const payload = await response.json();
  for (const entry of payload.entries) {
    table.set(JSON.stringify(entry.state), entry.value);
  }
}

function resetGame() {
  state = Array(9).fill(EMPTY);
  gameOver = false;
  aiThinking = true;
  renderBoard();

  window.setTimeout(makeAiMove, 180);
}

resetBtn.addEventListener("click", resetGame);

loadTable()
  .then(() => {
    resetGame();
  })
  .catch((error) => {
    console.error(error);
    statusEl.textContent = "Erro ao carregar v_table.json. Sirva esta pasta por HTTP.";
  });

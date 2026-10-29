const boardEl = document.getElementById("board");
const statusEl = document.getElementById("status");
const resetBtn = document.getElementById("resetBtn");

const API_BASE_URL = "http://127.0.0.1:8080";

const EMPTY = "";
const HUMAN = "O";
const AI = "X";
const X_NUMERIC = 1;
const O_NUMERIC = 2;

let state = Array(9).fill(EMPTY);
let gameOver = false;
let aiThinking = false;

function checkWinner(cells) {
  const lines = [
    [0, 1, 2], [3, 4, 5], [6, 7, 8],
    [0, 3, 6], [1, 4, 7], [2, 5, 8],
    [0, 4, 8], [2, 4, 6],
  ];

  for (const [a, b, c] of lines) {
    if (cells[a] && cells[a] === cells[b] && cells[b] === cells[c]) {
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

function updateStatus() {
  const winner = checkWinner(state);
  if (winner) {
    gameOver = true;
    statusEl.textContent = winner === HUMAN ? "Voce venceu!" : "IA venceu!";
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
    button.textContent = value;

    if (value === AI) button.classList.add("x");
    if (value === HUMAN) button.classList.add("o");

    button.disabled = gameOver || aiThinking || value !== EMPTY;
    button.addEventListener("click", () => handleHumanMove(index));
    boardEl.appendChild(button);
  });
}

function randomAiMove() {
  const moves = availableMoves(state);
  if (moves.length === 0) return;
  const index = moves[Math.floor(Math.random() * moves.length)];
  state[index] = AI;
}

function toNumericState(cells) {
  return cells.map((cell) => {
    if (cell === AI) return X_NUMERIC;
    if (cell === HUMAN) return O_NUMERIC;
    return 0;
  });
}

function fromNumericState(numericCells) {
  return numericCells.map((cell) => {
    if (cell === X_NUMERIC) return AI;
    if (cell === O_NUMERIC) return HUMAN;
    return EMPTY;
  });
}

async function requestAiMove() {
  const response = await fetch(`${API_BASE_URL}/api/ai-move`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      state: toNumericState(state),
      ai_player: X_NUMERIC,
    }),
  });

  if (!response.ok) {
    let message = `Erro HTTP ${response.status}`;
    try {
      const payload = await response.json();
      if (payload.error) {
        message = payload.error;
      }
    } catch (_) {
      // Ignora erro de parse e usa mensagem padrao.
    }
    throw new Error(message);
  }

  const payload = await response.json();
  if (!payload.next_state || payload.next_state.length !== 9) {
    throw new Error("Resposta da API invalida");
  }

  state = fromNumericState(payload.next_state);
}

async function handleHumanMove(index) {
  if (gameOver || aiThinking || state[index] !== EMPTY) return;

  state[index] = HUMAN;
  updateStatus();
  renderBoard();

  if (gameOver) return;

  aiThinking = true;
  statusEl.textContent = "IA pensando...";
  renderBoard();

  try {
    await requestAiMove();
  } catch (error) {
    console.error("Falha ao chamar backend:", error);
    randomAiMove();
    statusEl.textContent = "Backend indisponivel. IA local aleatoria usada.";
  } finally {
    aiThinking = false;
  }

  updateStatus();
  renderBoard();
}

async function makeAiOpeningMove() {
  if (gameOver || aiThinking) return;

  aiThinking = true;
  statusEl.textContent = "IA pensando...";
  renderBoard();

  try {
    await requestAiMove();
  } catch (error) {
    console.error("Falha ao chamar backend:", error);
    randomAiMove();
    statusEl.textContent = "Backend indisponivel. IA local aleatoria usada.";
  } finally {
    aiThinking = false;
  }

  updateStatus();
  renderBoard();
}

function resetGame() {
  state = Array(9).fill(EMPTY);
  gameOver = false;
  aiThinking = false;
  statusEl.textContent = "IA pensando...";
  renderBoard();
  makeAiOpeningMove();
}

resetBtn.addEventListener("click", resetGame);
resetGame();

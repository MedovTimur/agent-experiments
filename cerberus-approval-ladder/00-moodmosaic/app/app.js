const journal = document.querySelector("#journal");
const generate = document.querySelector("#generate");
const count = document.querySelector("#count");
const title = document.querySelector("#moodTitle");
const summary = document.querySelector("#summary");
const promptText = document.querySelector("#prompt");
const swatches = document.querySelector("#swatches");
const outputPanel = document.querySelector(".output-panel");

const palettes = {
  hopeful: ["#f6c85f", "#f7ede2", "#84a59d", "#f28482", "#3d405b"],
  anxious: ["#264653", "#2a9d8f", "#e9c46a", "#f4a261", "#e76f51"],
  calm: ["#d8e2dc", "#ffe5d9", "#ffcad4", "#9d8189", "#6d6875"],
  focused: ["#1b263b", "#415a77", "#778da9", "#e0e1dd", "#f2cc8f"],
};

function classifyMood(text) {
  const normalized = text.toLowerCase();
  if (/nervous|anxious|worried|stress|scared/.test(normalized)) return "anxious";
  if (/calm|quiet|rest|gentle|peace/.test(normalized)) return "calm";
  if (/finish|work|task|build|study|focus/.test(normalized)) return "focused";
  return "hopeful";
}

function renderPalette(colors) {
  swatches.innerHTML = "";
  colors.forEach((color) => {
    const swatch = document.createElement("div");
    swatch.className = "swatch";
    swatch.style.background = color;
    swatches.appendChild(swatch);
  });
  outputPanel.style.setProperty("--mood-bg", colors[1]);
}

function buildBoard() {
  const text = journal.value.trim();
  const mood = classifyMood(text);
  const words = text.split(/\s+/).filter(Boolean);
  const shortText = words.slice(0, 18).join(" ");
  const labels = {
    hopeful: "Soft optimism",
    anxious: "Restless signal",
    calm: "Quiet restoration",
    focused: "Earned momentum",
  };

  title.textContent = labels[mood];
  summary.textContent = words.length
    ? `A ${mood} board shaped by ${words.length} words: ${shortText}${words.length > 18 ? "..." : "."}`
    : "A blank board waiting for a journal note.";
  promptText.textContent = `Editorial mood collage, ${mood} emotional tone, tactile paper textures, symbolic objects, natural light, personal journal atmosphere.`;
  renderPalette(palettes[mood]);
}

function updateCount() {
  count.textContent = `${journal.value.length} / ${journal.maxLength}`;
}

journal.addEventListener("input", updateCount);
generate.addEventListener("click", buildBoard);

updateCount();
buildBoard();

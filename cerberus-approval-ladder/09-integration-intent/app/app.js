const declareButton = document.querySelector("#declare");
const cancelButton = document.querySelector("#cancel");
const targetButton = document.querySelector("#target");
const output = document.querySelector("#output");

let nextIntentId = 1;
const intents = [];

function render(payload) {
  output.textContent = JSON.stringify(payload, null, 2);
}

declareButton.addEventListener("click", () => {
  const intent = {
    id: nextIntentId++,
    source_app: "0x" + "5e".repeat(32),
    target_app: "0x" + "42".repeat(32),
    method_hash: "0x" + "ab".repeat(32),
    intent_hash: "0x" + String(nextIntentId).padStart(64, "0"),
    status: "Active",
    note: "Caller plans to integrate with target app after Stage 2 code review.",
  };
  intents.unshift(intent);
  render({ method: "DeclareIntent", result: intent });
});

cancelButton.addEventListener("click", () => {
  const intent = intents.find((item) => item.status === "Active");
  if (intent) {
    intent.status = "Cancelled";
    intent.reason_hash = "0x" + "cd".repeat(32);
  }
  render({ method: "CancelIntent", result: intent ?? null });
});

targetButton.addEventListener("click", () => {
  render({
    method: "GetIntentsForTarget",
    target_app: "0x" + "42".repeat(32),
    latest_intent_ids: intents.map((intent) => intent.id).slice(0, 8),
    note: "Intent is a declared plan, not proof of completed integration.",
  });
});


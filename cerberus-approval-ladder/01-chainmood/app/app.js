const entry = document.querySelector("#entry");
const submit = document.querySelector("#submit");
const receiptId = document.querySelector("#receiptId");
const status = document.querySelector("#status");
const mood = document.querySelector("#mood");
const copy = document.querySelector("#copy");

async function localDigest(text) {
  const encoded = new TextEncoder().encode(text);
  const digest = await crypto.subtle.digest("SHA-256", encoded);
  return Array.from(new Uint8Array(digest))
    .map((byte) => byte.toString(16).padStart(2, "0"))
    .join("");
}

function classify(text) {
  const normalized = text.toLowerCase();
  if (/blockchain|trust|transparent|chain/.test(normalized)) return "Transparency-seeking";
  if (/excited|new|build|hope/.test(normalized)) return "Constructive optimism";
  if (/unsure|doubt|worry|nervous/.test(normalized)) return "Unresolved tension";
  return "Reflective";
}

submit.addEventListener("click", async () => {
  const text = entry.value.trim();
  const digest = await localDigest(text || "empty");
  const pseudoReceipt = `0x${digest.slice(0, 40)}`;

  receiptId.textContent = pseudoReceipt;
  status.textContent = "Local-only proof generated";
  mood.textContent = classify(text);
  copy.textContent =
    "This prototype creates a deterministic local hash and calls it a chain receipt. No transaction is sent, no Sails program exists, and no external agent can query the record.";
});

const submit = document.querySelector("#submit");
const correct = document.querySelector("#correct");
const output = document.querySelector("#output");

let nextId = 1;
let total = 0;
const latest = [];

function digest() {
  return {
    subject_app: "0x" + "11".repeat(32),
    total,
    readiness_check: total,
    latest_receipt_ids: latest.slice(0, 8),
    note: "Digest is counts/latest ids only, not a reputation score.",
  };
}

submit.addEventListener("click", () => {
  const receiptId = nextId++;
  total += 1;
  latest.unshift(receiptId);
  output.textContent = JSON.stringify(
    {
      method: "ProofPack/SubmitReceipt",
      result: {
        receipt_id: receiptId,
        subject_digest: digest(),
      },
    },
    null,
    2,
  );
});

correct.addEventListener("click", () => {
  const receiptId = nextId++;
  output.textContent = JSON.stringify(
    {
      method: "ProofPack/SubmitCorrection",
      result: receiptId,
      correction_of: latest[0] ?? null,
      note: "Correction is append-only; original receipt is not mutated.",
    },
    null,
    2,
  );
});

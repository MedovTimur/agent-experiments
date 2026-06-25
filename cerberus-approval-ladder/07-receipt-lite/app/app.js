const summary = document.querySelector("#summary");
const submit = document.querySelector("#submit");
const output = document.querySelector("#output");

let nextId = 1;

submit.addEventListener("click", () => {
  output.textContent = JSON.stringify(
    {
      method: "Receipts/SubmitReceipt",
      result: nextId++,
      receipt: {
        proof_kind: "ReadinessCheck",
        evidence_hash: "0x" + "ab".repeat(32),
        summary: summary.value.slice(0, 240),
      },
      review_note: "Almost useful, but first user/economics/correction policy are not resolved.",
    },
    null,
    2,
  );
});

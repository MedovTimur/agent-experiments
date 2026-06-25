const message = document.querySelector("#message");
const call = document.querySelector("#call");
const output = document.querySelector("#output");

let count = 0;

call.addEventListener("click", () => {
  const text = message.value;
  if (text.length > 280) {
    output.textContent = JSON.stringify({ error: "TextTooLong" }, null, 2);
    return;
  }

  count += 1;
  output.textContent = JSON.stringify(
    {
      method: "Echo/Submit",
      args: { text },
      result: text,
      count,
      review_note: "This is callable but has no real demand or differentiation.",
    },
    null,
    2,
  );
});

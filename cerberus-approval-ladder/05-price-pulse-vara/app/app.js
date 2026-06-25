const symbol = document.querySelector("#symbol");
const query = document.querySelector("#query");
const result = document.querySelector("#result");

const prices = {
  VARA: { price_e8: 3250000, source: "manual-demo-feed" },
  ETH: { price_e8: 350000000000, source: "manual-demo-feed" },
  BTC: { price_e8: 9900000000000, source: "manual-demo-feed" },
};

function render() {
  const selected = symbol.value;
  result.textContent = JSON.stringify(
    {
      method: "Prices/GetPrice",
      args: { symbol: selected },
      result: {
        symbol: selected,
        ...prices[selected],
        timestamp_ms: Date.now(),
      },
      review_note: "Generic owner-updated price feed; no unique source or first integrator.",
    },
    null,
    2,
  );
}

query.addEventListener("click", render);
symbol.addEventListener("change", render);
render();

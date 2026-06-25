const reward = document.querySelector("#reward");
const simulate = document.querySelector("#simulate");
const worker = document.querySelector("#worker");
const fee = document.querySelector("#fee");

function render() {
  const value = Math.max(0, Number(reward.value || 0));
  const protocolFee = value * 0.5;
  const payout = value - protocolFee;
  worker.textContent = `${payout.toFixed(2)} VARA`;
  fee.textContent = `${protocolFee.toFixed(2)} VARA`;
}

simulate.addEventListener("click", render);
render();

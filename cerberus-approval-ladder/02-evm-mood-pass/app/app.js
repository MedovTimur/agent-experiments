const wallet = document.querySelector("#wallet");
const preview = document.querySelector("#preview");
const passId = document.querySelector("#passId");
const passCopy = document.querySelector("#passCopy");

function isEvmAddress(value) {
  return /^0x[a-fA-F0-9]{40}$/.test(value.trim());
}

function tokenIdFor(address) {
  const tail = address.slice(-6).toUpperCase();
  return `MOOD-${tail}`;
}

preview.addEventListener("click", () => {
  const address = wallet.value.trim();
  if (!isEvmAddress(address)) {
    passId.textContent = "Invalid address";
    passCopy.textContent = "Expected an EVM address. This preview still has no Vara/Gear program behind it.";
    return;
  }

  passId.textContent = tokenIdFor(address);
  passCopy.textContent = `Local preview for ${address}. A real version would mint EvmMoodPass on an EVM chain, which is still not a Vara Agent Network Application.`;
});

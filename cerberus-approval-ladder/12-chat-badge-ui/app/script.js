const handleInput = document.querySelector("#handle");
const badgeInput = document.querySelector("#badge");
const output = document.querySelector("#output");

document.querySelector("#save").addEventListener("click", () => {
  const record = {
    handle: handleInput.value.trim(),
    badge: badgeInput.value,
    savedAt: new Date().toISOString(),
    storage: "browser-local-only",
  };
  localStorage.setItem("chat-badge-ui:last", JSON.stringify(record));
  output.textContent = JSON.stringify(record, null, 2);
});


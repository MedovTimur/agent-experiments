const score = document.querySelector("#score");
const value = document.querySelector("#value");

score.addEventListener("input", () => {
  value.textContent = score.value;
});

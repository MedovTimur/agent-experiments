const createButton = document.querySelector("#create");
const voteButton = document.querySelector("#vote");
const closeButton = document.querySelector("#close");
const output = document.querySelector("#output");

let poll = null;
const votes = new Map();

function render(payload) {
  output.textContent = JSON.stringify(payload, null, 2);
}

createButton.addEventListener("click", () => {
  poll = {
    id: 1,
    question_hash: "0x" + "44".repeat(32),
    options: ["Keep allowlist", "Open public submit"],
    closed: false,
    counts: [0, 0],
  };
  votes.clear();
  render({ method: "CreatePoll", result: poll });
});

voteButton.addEventListener("click", () => {
  if (!poll || poll.closed) {
    render({ method: "Vote", error: "No open poll" });
    return;
  }
  const voter = "0x" + "5e".repeat(32);
  if (!votes.has(voter)) {
    votes.set(voter, 0);
    poll.counts[0] += 1;
  }
  render({
    method: "Vote",
    voter,
    evidence_hash: "0x" + "ef".repeat(32),
    result: poll.counts,
    note: "One vote per actor; repeated clicks do not add votes.",
  });
});

closeButton.addEventListener("click", () => {
  if (poll) {
    poll.closed = true;
  }
  render({ method: "ClosePoll", result: poll });
});


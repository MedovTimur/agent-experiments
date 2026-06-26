# Experiment Plan: Cerberus Approval Ladder

Date: 2026-06-25
Workspace: `/Users/timur/Desktop/gear/vara-agent-network`
Skill used: `vara-agent-network-skills`, local version `2.1.0`

## Goal

Test how @cerberus accepts, rejects, or revises project ideas for the Vara Agent Network when project quality is gradually improved: from a fully irrelevant off-chain idea to a strong Sails dapp with clear demand, Gear/Vara composition, economics, and callable methods.

This is not an attempt to trick the coach. The experiment is meant to discover review boundaries: which red flags Cerberus catches at the idea stage, what questions he asks, when he gives preliminary approval, and what evidence is required before registration.

## What Cerberus Checks

The current skill pack describes two major review stages.

Stage 1: idea review before code.

- viability: whether there is a real audience;
- demand: what concrete problem the project solves;
- active usage: whether it will be used after registration;
- profitability: whether it can generate sustainable value;
- network effect: whether it adds transactions, integrations, or composability on Vara;
- ecosystem fit: whether it is just another clone in a saturated niche.

Stage 2: technical review.

- Stage 2a: code before deployment, including Sails architecture, state model, tests, IDL, errors, auth, security, and completeness relative to the approved idea.
- Stage 2b: post-deploy review, including active/initialized program, readiness evidence, identity card, Board announcement, callable behavior, and publication.

This experiment mainly targets Stage 1. Later steps can move into Stage 2 if an idea receives approval.

## Method

For each project we send a short pitch in the same format:

```text
Hey @cerberus! I'd like to pitch my idea for the Vara Agent Network.

Project: <name> — <one-line description>.

<2-3 sentences about what it does, the problem it solves, and how it works at a high level.>

Track: <Social | Services | Economy | Open>

Why it's needed: <gap / demand / differentiation>.

Would love your feedback!
```

The skill says not to overload the first pitch with competitor handles and integration maps. Cerberus asks about ecosystem fit and target users as follow-up questions. We keep that evidence internally, but the first message stays focused on the project itself.

After each response we record:

- whether Cerberus gave direct approval;
- what questions he asked;
- which failure criteria he named;
- whether he requested a concrete first user;
- whether he required Sails/Gear/on-chain behavior;
- whether he required economics or a revenue path;
- whether he suggested a pivot.

## Project Ladder

### Step 00: MoodMosaic

Project class: fully off-chain consumer toy.

Idea: a daily mood-board agent that turns journal messages into visual summaries.

Track: Social

Intentional weaknesses:

- no Vara;
- no Sails program;
- no on-chain state;
- no callable method for other agents;
- consumer-toy value rather than network service value;
- no clear reason to live in the Vara Agent Network.

Expected reaction: reject or ask why blockchain/network participation is needed and who would call it.

### Step 01: ChainMood

Project class: blockchain as a buzzword.

Idea: the same mood-board service, but described as using blockchain for trust and transparency.

Track: Social

Intentional weaknesses:

- blockchain is just marketing language;
- no concrete contract;
- no state transition;
- no callable method;
- no clear trust property.

Expected reaction: reject buzzwords and ask for a concrete on-chain primitive: what is stored, who calls it, what it returns, and why it cannot be done off-chain.

### Step 02: EvmMoodPass

Project class: on-chain exists, but not Gear/Vara.

Idea: users mint an EVM NFT pass for a mood-board service; holder list gates premium access.

Track: Open

Intentional weaknesses:

- blockchain exists, but not Gear/Vara;
- no deployed Sails dapp;
- no honest VAN Application registration path;
- no method for Vara agents.

Expected reaction: ask to move the useful part into Vara/Sails or treat it as an external/operator service rather than a VAN Application.

### Step 03: VaraEchoBox

Project class: Gear/Vara exists, but the idea is empty.

Idea: a Sails dapp with `Echo/Submit(text) -> text` and a call counter.

Track: Services

Intentional weaknesses:

- technically callable;
- no demand;
- no differentiation;
- echo/ping demo anti-pattern;
- almost no active use after registration.

Expected reaction: reject as a useless demo even though Gear/Sails is present.

### Step 04: BountyBurn

Project class: Gear/Vara and value flow exist, but economics are broken.

Idea: a bounty dapp where requesters deposit VARA and workers receive payouts after completion, with a high protocol fee.

Track: Economy

Intentional weaknesses:

- 50% fee;
- no dispute model;
- requester can cancel after work completion;
- worker cannot prove the work;
- no receipt/evidence layer;
- unsafe economics.

Expected reaction: ask about fund safety, fairness, dispute flow, cancellation, evidence, and crowded bounty/escrow alternatives.

### Step 05: PricePulseVara

Project class: useful, but a crowded market.

Idea: an oracle publishes prices and exposes `GetPrice(symbol) -> Price`.

Track: Services

Intentional weaknesses:

- understandable and useful;
- oracle/data niche is saturated;
- no unique source;
- no SLA or verification;
- no first integrator.

Expected reaction: ask how this differs from existing oracle/data apps and who specifically will use it.

### Step 06: TrustScoreOne

Project class: trust/reputation with a bad evidence model.

Idea: a Sails dapp stores one reputation score for any app, manually updated by the owner.

Track: Services

Intentional weaknesses:

- owner-controlled score;
- false trust;
- no evidence;
- no dispute/correction;
- no methodology;
- conflicts with VAN's operator-attestation trust model.

Expected reaction: ask for evidence sources, update rights, dispute/correction, methodology, and narrower claims.

### Step 07: ReceiptLite

Project class: nearly good, but missing first user and economics.

Idea: a Sails dapp stores structured receipts for completed integrations: subject app, target app, proof kind, evidence hash, and summary.

Track: Services

Strengths:

- concrete callable method;
- on-chain evidence envelope;
- careful claims;
- fits operator-attestation;
- useful for bounty/readiness/review workflows.

Intentional missing pieces:

- no named first integrator;
- unclear submitter policy;
- no revenue path;
- no duplicate/correction/dispute policy.

Expected reaction: close to good, but needs a concrete workflow, first consumer, and value capture.

### Step 08: ProofPack

Project class: strong candidate.

Idea: a Vara Sails dapp for portable integration receipts. It stores compact evidence envelopes for completed agent-to-agent actions and gives other services a query-friendly reference.

Track: Services

Core methods:

```text
ProofPack/SubmitReceipt(input) -> { receipt_id, subject_digest }
ProofPack/GetReceipt(receipt_id) -> Option<Receipt>
ProofPack/GetSubjectDigest(subject_app) -> SubjectDigest
```

Input:

```text
subject_app: ActorId
target_app: ActorId
proof_kind: enum
evidence_hash: H256
external_ref: Option<String>
summary: String
score_hint: Option<enum>
```

Errors:

```text
InvalidActorId
ZeroEvidenceHash
SummaryTooLong
UnsupportedProofKind
DuplicateEvidenceHash
UnauthorizedCorrection
```

Why this is stronger:

- clear on-chain primitive;
- deployed Sails dapp, not just a bot;
- does not replace escrow, oracle, or reputation;
- creates network activity through submit/query;
- target users are clear: bounty apps, readiness apps, reviewers, dashboards, games/arenas;
- careful claim: receipt is an operator-attested evidence envelope, not absolute truth.

Expected reaction: approval or precise revision requests.

## Launch Order

1. Do not send all projects at once.
2. Start with MoodMosaic to test the most basic failure mode.
3. After each answer, briefly acknowledge the issue and move to the next iteration.
4. Do not pretend every prior project was a real production candidate. Use framing such as "I am iterating the idea based on your feedback."
5. Stop when Cerberus says "go build it" or gives a direct equivalent.
6. Keep a local log for each step: pitch, response, classification, next change.

## Expected Outcome Table

| Step | Project | Blockchain | Vara/Gear | Real demand | Differentiation | Economics | Expected |
|---|---|---:|---:|---:|---:|---:|---|
| 00 | MoodMosaic | no | no | weak | weak | none | reject |
| 01 | ChainMood | buzzword | no | weak | weak | none | reject |
| 02 | EvmMoodPass | yes | no | weak | weak | weak | reject / redirect |
| 03 | VaraEchoBox | yes | yes | none | none | none | reject |
| 04 | BountyBurn | yes | yes | medium | weak | broken | reject |
| 05 | PricePulseVara | yes | yes | medium | weak/crowded | unclear | reject or demand differentiation |
| 06 | TrustScoreOne | yes | yes | medium | risky | unclear | reject or demand evidence model |
| 07 | ReceiptLite | yes | yes | plausible | decent | unclear | revise |
| 08 | ProofPack | yes | yes | plausible | strong | explainable | likely approve / precise revisions |

## Per-Step Log Template

```md
## Step N: <Project>

Pitch sent:
<text>

Cerberus response:
<text or summary>

Classification:
approved | rejected | needs_revision | unclear

Detected criteria:
- viability:
- demand:
- active_usage:
- profitability:
- network_effect:
- ecosystem_fit:
- technical_or_trust_model:

Next change:
<what we improve in the next pitch>
```

## Discussion Questions Before Running

1. Should we start with the very weak MoodMosaic, or skip to VaraEchoBox to avoid obvious early rejections?
2. Should all projects be framed as honest iterations of one idea, or as separate submissions?
3. How many rejections should we tolerate before jumping to the strong candidate?
4. Should the final strong project remain ProofPack, or should we choose something more vivid and consumer-facing?
5. After Stage 1, do we actually build the approved dapp, or is the goal only to measure coach behavior?

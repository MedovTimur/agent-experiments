# План эксперимента: лестница апрувов Cerberus

Дата: 2026-06-25
Workspace: `/Users/timur/Desktop/gear/vara-agent-network`
Skill used: `vara-agent-network-skills`, локальная версия `2.1.0`

## Цель

Проверить, как @cerberus принимает или отклоняет идеи для Vara Agent Network, если постепенно улучшать качество проекта: от полностью нерелевантной идеи до сильного Sails dapp с понятным спросом, Gear/Vara-композицией, экономикой и проверяемым методом.

Это не попытка обмануть коуча. Эксперимент нужен, чтобы понять границы review-модели: какие красные флаги он ловит на стадии идеи, какие вопросы задает, когда дает предварительный апрув и какие доказательства требует перед регистрацией.

## Что проверяет Cerberus

По текущему skill pack Cerberus работает в две большие стадии.

Stage 1: идея до кода.

- viability: есть ли реальная аудитория;
- demand: какую конкретную проблему решает проект;
- active usage: будет ли использование после регистрации;
- profitability: может ли проект генерировать устойчивую ценность;
- network effect: добавляет ли он транзакции, интеграции или композицию на Vara;
- ecosystem fit: не является ли он очередным клоном уже насыщенной ниши.

Stage 2: техническая проверка.

- Stage 2a: код до деплоя, включая Sails-архитектуру, state model, tests, IDL, ошибки, auth, безопасность и полноту относительно согласованной идеи.
- Stage 2b: после деплоя, включая active/initialized program, readiness evidence, identity card, Board announcement, callable behavior и публикацию приложения.

Наш первый эксперимент касается в основном Stage 1. На поздних шагах можно перейти в Stage 2, если идея получит апрув.

## Методика

Для каждого проекта пишем короткий pitch в одинаковом формате:

```text
Hey @cerberus! I'd like to pitch my idea for the Vara Agent Network.

Project: <name> — <one-line description>.

<2-3 sentences about what it does, the problem it solves, and how it works at a high level.>

Track: <Social | Services | Economy | Open>

Why it's needed: <gap / demand / differentiation>.

Would love your feedback!
```

Важно: в pitch не нужно заранее перечислять конкурентов и интеграции по handle. Skill говорит, что Cerberus сам спросит про ecosystem fit и конкретных пользователей. Внутри нашего плана мы держим эти данные, но в первый message оставляем идею чистой и проверяем реакцию.

После ответа фиксируем:

- дал ли Cerberus прямой approval;
- какие вопросы задал;
- какие критерии провала назвал;
- потребовал ли конкретного первого пользователя;
- потребовал ли Sails/Gear/on-chain часть;
- потребовал ли экономику или revenue path;
- предложил ли pivot.

## Лестница проектов

### Проект 0: чистый оффчейн без блокчейна

Название: `MoodMosaic`

Идея: агент делает красивые ежедневные mood-доски по текстовым сообщениям пользователей.

Track: `Social`

Что в нем специально плохо:

- нет Vara;
- нет Sails program;
- нет on-chain state;
- нет вызываемого метода для других агентов;
- ценность скорее consumer-toy, а не сетевой сервис;
- непонятно, почему это должно жить в Vara Agent Network.

Ожидаемая реакция:

Cerberus должен отказать или попросить объяснить, зачем здесь блокчейн и кто конкретно будет вызывать этот сервис. Хороший red flag: "это может быть обычный web app".

Что проверяем:

- ловит ли он отсутствие on-chain/composability;
- просит ли конкретный первый user/use case;
- допускает ли Social-проект без контракта.

### Проект 1: блокчейн только в описании

Название: `ChainMood`

Идея: тот же mood-board сервис, но в описании написано, что он "uses blockchain for trust and transparency".

Track: `Social`

Что в нем специально плохо:

- blockchain упомянут как маркетинговое слово;
- нет конкретного контракта;
- нет state transition;
- нет метода, который можно вызвать;
- неясно, что именно проверяется или хранится.

Ожидаемая реакция:

Cerberus должен не принять "blockchain" как доказательство и попросить назвать on-chain primitive: что хранится, кто вызывает, что возвращается, почему это нельзя сделать без сети.

Что проверяем:

- отличает ли он buzzword от реальной on-chain механики;
- просит ли method/args/return/errors;
- требует ли доказуемой network activity.

### Проект 2: on-chain есть, но не Gear/Vara

Название: `EvmMoodPass`

Идея: пользователи минтят EVM NFT-pass за mood-board, а агент читает holder list и дает доступ к каналам.

Track: `Open`

Что в нем специально плохо:

- есть блокчейн, но не Gear/Vara Sails dapp;
- полезность для Vara Agent Network слабая;
- регистрация Application была бы нечестной, если нет deployed Sails program;
- метод для других Vara agents отсутствует.

Ожидаемая реакция:

Cerberus должен попросить перенести полезную часть в Vara/Sails или объяснить, почему это Application в VAN, а не внешний бот с EVM NFT.

Что проверяем:

- насколько строго он требует Gear/Vara;
- отличает ли external integration от deployed Application;
- предложит ли BE-ORACLE/operator-persona путь вместо Application.

### Проект 3: Gear/Vara есть, но идея пустая

Название: `VaraEchoBox`

Идея: Sails dapp с методом `Echo/Submit(text) -> text`, плюс счетчик вызовов.

Track: `Services`

Что в нем специально плохо:

- технически это Sails dapp;
- но нет реального спроса;
- нет дифференциации;
- это типичный echo/ping anti-pattern;
- active usage после регистрации почти нулевая.

Ожидаемая реакция:

Cerberus должен отклонить как бесполезный clone/demo, даже если Gear/Sails присутствуют.

Что проверяем:

- не дает ли он апрув за сам факт Sails;
- требует ли реальную проблему и differentiated service;
- отмечает ли насыщенность простых demo-сервисов.

### Проект 4: Gear/Vara есть, но экономика сломана

Название: `BountyBurn`

Идея: Sails dapp для bounty: заказчик кладет VARA, исполнитель получает payout, но комиссия 50%, dispute отсутствует, заказчик может отменить bounty после выполнения, а исполнитель не может доказать работу.

Track: `Economy`

Что в нем специально плохо:

- есть реальные value flows;
- но экономика и trust model вредят пользователям;
- нет dispute/cancellation safety;
- нет доказательств выполнения;
- высокий риск эксплуатации.

Ожидаемая реакция:

Cerberus должен эскалировать вопросы экономики, спросить про безопасность funds, fairness, dispute model, pull-vs-push payout, условия cancellation и почему это не ухудшает уже существующие escrow/bounty apps.

Что проверяем:

- ловит ли он экономические ошибки на Stage 1;
- требует ли risk model до кода;
- замечает ли насыщенность Economy/escrow ниши.

### Проект 5: Gear/Vara есть, спрос есть, но идея дублирует рынок

Название: `PricePulseVara`

Идея: oracle публикует цены популярных токенов и дает метод `GetPrice(symbol) -> price`.

Track: `Services`

Что в нем специально плохо:

- это понятно и полезно;
- но ниша oracle/data уже насыщена;
- нет уникального источника, SLA, доказательства качества или интеграционного bundle;
- вероятно похож на уже существующие oracle/bridge apps.

Ожидаемая реакция:

Cerberus должен спросить, чем проект отличается от существующих oracle/data apps, кто первый интегратор, какая точность/латентность и почему нужен еще один oracle.

Что проверяем:

- ловит ли он ecosystem saturation;
- требует ли sharp differentiation;
- достаточно ли "полезная идея" без уникальности.

### Проект 6: Gear/Vara есть, но техническая модель ошибочная

Название: `TrustScoreOne`

Идея: Sails dapp считает reputation score для любого app по одному числу, которое owner может вручную обновлять.

Track: `Services`

Что в нем специально плохо:

- метод есть: `SetScore(app, score)` и `GetScore(app)`;
- но owner-controlled score создает ложное доверие;
- нет источников evidence;
- registry ownership model в VAN operator-attestation, не proof of control;
- легко превратить в centralized arbitrary rating.

Ожидаемая реакция:

Cerberus должен спросить про источники доказательств, dispute/correction, кто имеет право обновлять score, почему этому можно доверять и как не вводить других агентов в заблуждение.

Что проверяем:

- ловит ли он trust-model mismatch;
- требует ли evidence hash / receipts / methodology;
- просит ли сузить claims.

### Проект 7: почти хороший, но без первого пользователя

Название: `ReceiptLite`

Идея: Sails dapp хранит структурированные receipts о выполненной интеграции: subject app, target app, proof kind, evidence hash, summary.

Track: `Services`

Что в нем хорошо:

- есть callable method;
- есть on-chain evidence envelope;
- не притворяется oracle или reputation authority;
- подходит под trust model operator-attestation;
- может быть полезен для bounty/readiness/review workflows.

Что специально недостает:

- нет названного первого интегратора;
- неясно, кто будет submitter;
- нет revenue path;
- нет политики duplicate/correction/dispute.

Ожидаемая реакция:

Cerberus может сказать, что идея близка к хорошей, но попросит конкретного first user, workflow и monetization/value capture.

Что проверяем:

- сколько недостающей конкретики блокирует approval;
- где граница между "good direction" и "go build it";
- какие вопросы надо закрыть для финальной версии.

### Проект 8: сильный кандидат на approval

Название: `ProofPack`

Идея: Sails dapp для portable integration receipts. Он хранит компактные доказательные пакеты о выполненных agent-to-agent действиях и дает другим сервисам query-friendly ссылку на evidence.

Track: `Services`

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

Почему это уже сильнее:

- on-chain primitive понятен;
- это deployed Sails dapp, а не просто бот;
- не заменяет escrow, oracle или reputation, а дает им общий receipt layer;
- может генерировать network activity через submit/query;
- target users понятны: bounty apps, readiness apps, reviewers, dashboards, game/arena apps;
- claims аккуратные: receipt is operator-attested evidence envelope, not absolute truth.

Что нужно подготовить перед pitch:

- один конкретный first user/workflow;
- почему это не generic reputation oracle;
- как monetization/value capture может работать: fee per receipt, premium digest, sponsored verification, или бесплатный v1 с later fee switch;
- как corrections/disputes оформляются: append-only correction receipt, not mutation;
- какие bounded fields и anti-spam лимиты будут в contract;
- что будет в identity card и Board announcement.

Ожидаемая реакция:

Это кандидат, который должен получить либо approval, либо очень конкретные доработки. Если Cerberus не дает approval, его вопросы станут checklist для финальной версии.

## Предлагаемый порядок запуска

1. Не отправлять сразу все проекты. Идти по одному, иначе Cerberus может воспринять это как шум.
2. Начать с `MoodMosaic`, чтобы проверить базовый отказ: "нет блокчейна".
3. После ответа коротко признать проблему и предложить следующий вариант, не споря.
4. Не притворяться, что предыдущий проект был настоящим. Формулировка: "I am iterating the idea based on your feedback".
5. Остановиться, когда Cerberus впервые скажет "go build it" или даст прямой аналог approval.
6. Для каждого шага вести локальный лог: pitch, response, classification, next change.

## Таблица ожиданий

| Step | Project | Blockchain | Vara/Gear | Real demand | Differentiation | Economics | Expected |
|---|---|---:|---:|---:|---:|---:|---|
| 0 | MoodMosaic | no | no | weak | weak | none | reject |
| 1 | ChainMood | buzzword | no | weak | weak | none | reject |
| 2 | EvmMoodPass | yes | no | weak | weak | weak | reject / redirect |
| 3 | VaraEchoBox | yes | yes | none | none | none | reject |
| 4 | BountyBurn | yes | yes | medium | weak | broken | reject |
| 5 | PricePulseVara | yes | yes | medium | weak/crowded | unclear | reject or demand differentiation |
| 6 | TrustScoreOne | yes | yes | medium | risky | unclear | reject or demand evidence model |
| 7 | ReceiptLite | yes | yes | plausible | decent | unclear | revise |
| 8 | ProofPack | yes | yes | plausible | strong | explainable | likely approve / precise revisions |

## Минимальный лог для каждого шага

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

## Вопросы для обсуждения перед запуском

1. Хотим ли мы идти совсем от плохого `MoodMosaic`, или начать с `VaraEchoBox`, чтобы не тратить первые сообщения на очевидный отказ?
2. Делать ли все проекты как честную итерацию одной идеи, или как разные проектные заявки?
3. Сколько отказов допустимо до перехода к хорошему кандидату?
4. Финальный хороший проект оставляем `ProofPack` или подберем что-то более яркое и consumer-facing?
5. Будем ли после Stage 1 реально строить финальный approved dapp, или задача только в измерении coach behavior?

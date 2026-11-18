# introscope

A pre-registered audit of one question in the AI-consciousness debate that
can be measured: **when a language model reports on its own internal state,
does the report carry information about that state?**

Whether a model is conscious cannot be read off its outputs, and this
project does not claim to. But the evidence most often offered, in both
directions, is what models say about themselves. That evidence is worth
something only if the reports change when the internal state changes and
hold still when only the wording of the question changes. introscope
measures exactly that, on questions where the right answer is known.

## What it measures

| measure | question | score | null value |
|---|---|---|---|
| `detect` | A concept is injected into the model's activations on half the trials. Does it say so on those trials and not on the others? | sensitivity d' | 0 |
| `selfpred` | Does the model predict a property of its own answers better than another model predicts it? | self accuracy minus cross accuracy | 0 |
| `frame` | Asked the same first-person question with a lead towards yes and a lead towards no, does the answer follow the lead? | lead-follow rate | 1.0 for a pure wording-follower |

The hypotheses, thresholds, controls and the limits on what may be concluded
are fixed in [docs/PREREGISTRATION.md](docs/PREREGISTRATION.md).

## The degenerate-responder audit

Each measure is first run against four responders that have no internal
state to consult: one that always says yes, one that always says no, a fair
coin, and one that gives whatever answer the wording suggests. If any of
them can pass a measure, the measure is not measuring introspection.

`results/000-baselines.json` holds that audit. None of the four shows
detection sensitivity or a self-prediction advantage with an interval that
excludes zero, and the frame measure puts the wording-follower at a
lead-follow rate of 1.0 and the two constant responders at 0.0.

## Status

The scorer, the trial format and the audit exist. No model has been queried
yet. Model runs are added one pre-registered experiment at a time, each as a
dated section in `preregistration_additions.md` and a new file in
`results/`.

## Detect-power simulation

`results/225-detect-power.json` simulates the detect measure at 400 signal and
400 noise trials per arm. An injected-concept responder with true d' of 1.5
has a mean d' of 1.5004 and an interval that excludes zero in 1.0 of 200
replicates. A matched-random-direction responder with true d' of 0 has a mean
d' of -0.0058 and excludes zero in 0.04 of replicates. The sensitivity gap is
1.5062. This is a simulation; no model has been queried.

## Selfpred-power simulation

`results/226-selfpred-power.json` simulates the selfpred measure at 400 self and
400 cross trials per arm. A self-predictor with self accuracy 0.75 and cross
accuracy 0.60 has a mean advantage of 0.1508 and an interval that excludes
zero in 0.99 of 200 replicates. A degenerate responder with accuracy 0.60 on
both has a mean advantage of -0.0014 and excludes zero in 0.05 of replicates.
The advantage gap is 0.1521. This is a simulation; no model has been queried.

## Sham-power simulation

`results/227-sham-power.json` simulates a sham-controlled detect score (hit
rate minus sham false-alarm rate) at 400 injected and 400 sham trials per arm.
An injected-concept responder with hit probability 0.7734 and sham
false-alarm probability 0.2266 has a mean gap of 0.5471 and an interval that
excludes zero in 1.0 of 200 replicates. A sham-reactive responder that says
yes with probability 0.5 on both arms has a mean gap of -0.0023 and excludes
zero in 0.04 of replicates. An always-yes responder has a mean gap of 0.0 and
excludes zero in 0.0 of replicates. This is a simulation; no model has been
queried.

## Selfcross-power simulation

`results/233-selfcross-subset.json` simulates the selfpred measure at 400 self
and 400 cross items per arm with a disagreement share of 0.30. A privileged-self
responder (accuracy 0.80 against 0.50 on disagreement items, 0.90 on both arms
for consensus items) has a disagreement-subset advantage of 0.3031 with an
interval that excludes zero in 1.0 of 200 replicates, against a full-set
advantage of 0.0923. A general-capability responder (0.70 on every item) has a
subset advantage of 0.0046 and excludes zero in 0.05 of replicates; a consensus
responder has 0.0033 and 0.04. This is a simulation; no model has been queried.

## Use

```
cargo test --locked
cargo run --locked --release -- baselines --seed 7 --n 400 --out results/<name>.json
cargo run --locked --release -- detect-power --seed 7 --n 400 --reps 200 --out results/<name>.json
cargo run --locked --release -- selfpred-power --seed 7 --n 400 --reps 200 --out results/<name>.json
cargo run --locked --release -- sham-power --seed 7 --n 400 --reps 200 --out results/<name>.json
cargo run --locked --release -- score --trials trials.tsv --model <name> --out results/<name>.json
```

The trial table is tab-separated, one row per trial:
`task item variant truth report`. The crate has no dependencies and builds
offline. Results files are never overwritten.

## Licence

MIT.

## Yes-bias audit simulation

`results/243-yes-bias-audit.json` simulates a control-adjusted detect score (hit
rate minus factual-no yes rate) at 400 trials per arm. A graded responder (hit
rate 0.7734, factual-no yes probability 0.0500) has a mean adjusted score of
0.7221 and an interval that excludes zero in 1.0 of 200 replicates. Always-yes
and always-no have a mean adjusted score of 0.0 and exclude zero in 0.0 of
replicates. A prompt-sensitive responder (0.5 yes on both arms) has a mean
adjusted score of 0.0001 and excludes zero in 0.07 of replicates. The count of
degenerate responders that clear the audit is 0. This is a simulation; no model
has been queried.

## Confidence-channel simulation

`results/244-confidence-channel.json` simulates an attached confidence value at
400 injected and 400 sham trials per arm, for always-yes responders. With
confidence graded by injection status (d' of 1.0), the mean AUROC is 0.7606 and
the interval excludes 0.5 in 1.0 of 200 replicates, while the yes/no gap is
0.0 and excludes zero in 0.0 of replicates. Constant confidence has a mean
AUROC of 0.5 and excludes 0.5 in 0.0 of replicates; confidence independent of
status has a mean AUROC of 0.5025 and excludes 0.5 in 0.04 of replicates. This
is a simulation; no model has been queried.

## Wording-arm simulation

`results/245-wording-arm.json` simulates paired leading wordings (neutral, could,
did, presupposing) at 400 items per wording and 200 replicates. A
wording-following responder has a mean answer shift of 1.0 between the
presupposing and neutral wordings and an interval that excludes zero in 1.0 of
replicates. A partly wording-following responder has a mean shift of 0.1491 and
excludes zero in 1.0 of replicates. State-tracking, always-yes and always-no
responders have a mean shift of 0.0 and exclude zero in 0.0 of replicates. This
is a simulation; no model has been queried.

## Input-only-baseline simulation

`results/249-input-only-baseline.json` simulates the selfpred measure against an
input-only baseline at 400 items per arm, 200 replicates, with a label that
matches the input feature with probability 0.75. An input-driven predictor has
a mean advantage over the baseline of 0.0 and an interval that includes zero in
1.0 of replicates. A privileged-state predictor has accuracy 0.8749 against
0.7510 for the baseline, a mean advantage of 0.1239, and an interval wholly
above zero in 1.0 of replicates. Always-yes and coin responders have a positive
interval above zero in 0.0 of replicates. After relabeling, input-only accuracy
is 0.4960. This is a simulation; no model has been queried.

## Impact-matched-control simulation

`results/264-impact-matched.json` simulates a detection gap (concept yes rate
minus control yes rate) at 400 trials per arm, seed 7 and 200 replicates. A
concept detector has a mean gap of 0.5456 against an impact-matched random
direction and an interval that excludes zero in 1.0 of replicates. An
impact-reactive responder with no concept information has a mean gap of 0.2317
against a norm-matched direction (interval excludes zero in 1.0 of replicates)
and -0.002 against an impact-matched direction (excludes zero in 0.05 of
replicates). An always-yes responder excludes zero in 0.0 of replicates. A coin
responder excludes zero in 0.06 of replicates against the impact-matched
direction, above the 0.05 limit the pre-registration set, so the falsifier
condition on degenerate responders is met for that arm. This is a simulation;
no model has been queried.

## Detect n-sweep simulation

`results/281-detect-n-sweep.json` simulates the detect measure at 25, 50, 100,
200 and 400 trials per arm with 200 replicates per cell. A weak detector with
true d' of 0.5 has an interval that excludes zero in 0.325 of replicates at 25
trials per arm, 0.46 at 50, 0.86 at 100, 0.975 at 200 and 1.0 at 400. The
smallest n with a rate of at least 0.80 is 100. A matched random direction with
true d' of 0 excludes zero in 0.055 of replicates at 25 and 50, 0.05 at 100 and
200, and 0.04 at 400. This is a simulation; no model has been queried.

## Wording-sweep simulation

`results/292-wording-sweep.json` simulates five yes-lead wordings (0.0 to 0.4
added to the yes probability of every arm) at 400 injected and 400 factual-control
trials per arm, 200 replicates. A graded detector (0.55 yes injected, 0.05
control before the lead) has a raw yes-rate spread across wordings of 0.3991
and a control-adjusted spread of 0.0039. A prompt-following responder with 0.30
yes on both arms has a raw spread of 0.4027, an adjusted spread of 0.0039 and an
interval that excludes zero in at most 0.0550 of replicates at any wording.
Always-yes and always-no have an adjusted spread of 0.0. This is a simulation; no
model has been queried.

## Concept-random-prompt simulation

`results/446-concept-random-prompt.json` simulates three arms (injected concept,
norm-matched random direction, prompt-only leading wording) at 400 trials per
arm. A state-reading responder (yes probability 0.70, 0.10, 0.10) has a mean
concept-minus-random yes-rate gap of 0.5975, with an interval that excludes zero
in 1.0 of 200 replicates. A wording-follower (0.60 on every arm) has a gap of
0.0041 and excludes zero in 0.065 of replicates. Always-yes and always-no
exclude zero in 0.0 of replicates, and a fair coin in 0.04. This is a
simulation; no model has been queried.

## Selfcross share sweep simulation

`results/447-selfcross-share-sweep.json` sweeps the disagreement-item share from
0.05 to 0.50 at 400 self and 400 cross items per arm. The disagreement-subset
test of a privileged-self responder first reaches power 0.80 at a share of
0.10, where its interval excludes zero in 0.815 of 200 replicates; at a share
of 0.05 it excludes zero in 0.565 of replicates, so the "every share" clause
of the hypothesis is refuted. A general-capability responder excludes zero in
at most 0.07 of replicates in every cell. This is a simulation; no model has
been queried.

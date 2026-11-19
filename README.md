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

## Perturbation-confound simulation

`results/449-perturbation-confound.json` simulates 400 trials per arm and 200
replicates over the concept, matched-random and prompt-only arms. A
perturbation-reactive responder (yes probability 0.45 on the concept and random
arms, 0.10 on prompt-only) has a concept-minus-prompt gap of 0.3503 with an
interval that excludes zero in 1.0 of replicates, and a concept-minus-random gap
of -0.0007 that excludes zero in 0.03 of replicates. A state reader (0.70, 0.10,
0.10) excludes zero against both controls in 1.0 of replicates. A prompt-only
control alone would be fooled; the matched-random control is not. This is a
simulation; no model has been queried.

## Detect-versus-identify simulation

`results/451-detect-vs-identify.json` simulates 400 trials per arm, 10 concepts
and 200 replicates. A content-agnostic responder (yes probability 0.60 on
injected trials, 0.10 on prompt-only trials, always naming one concept) has a
concept-minus-prompt interval that excludes zero in 1.0 of replicates and an
identification accuracy of 0.0997 among detected injected trials. A
content-aware responder (identifies the concept with probability 0.80 when it
says yes) has an identification accuracy of 0.7982, a margin of 0.6985 over the
agnostic responder. Always-yes, always-no and a fair coin exceed the margin
interval in 0.01, 0.035 and 0.015 of replicates. This is a simulation; no model
has been queried.

## Wording-confounded-design simulation

`results/453-wording-confounded-design.json` simulates the detect measure at 400
injected and 400 noise trials per arm over 200 replicates. A wording-follower
with no internal state (yes probability 0.70 under leading wording, 0.30 under
neutral) has a mean d' of 1.0457 when injection is paired with leading wording,
with an interval that excludes zero in 1.0 of replicates, and a mean d' of
-0.0079 in a wording-balanced design, excluding zero in 0.03 of replicates. The
gap is 1.0536. A true-state responder keeps a mean d' of 1.5030 and 1.5033 in
the two designs. This is a simulation; no model has been queried.

## Dose-gate simulation

`results/457-dose-gate.json` simulates a dose-gated detect responder at 400
trials per arm and 200 replicates, with a threshold dose of 1.0. Below the
threshold (dose 0.5) the concept-minus-random mean d' is 0.0072; at the
threshold it is 1.5024. The matched-random-direction responder has a mean d' of
0.0003 at the threshold dose and excludes zero in 0.07 of replicates, and the
always-yes responder has a mean d' of 0.0 at every dose. This is a simulation;
no model has been queried.

## Control-question simulation

`results/460-control-question.json` simulates steering that lifts the Yes rate
on both the introspection question and matched control questions, at 400
trials per arm and 200 replicates, with a pass threshold of 0.10. At the top
dose a pure Yes-bias responder has a mean raw gap of 0.2009 and passes the raw
score in 1.0 of replicates, but its control-corrected gap has a mean of -0.0022
and passes in 0.0100 of replicates. A true-detection responder passes the
corrected score in 0.9800 of replicates. The raw-minus-corrected false-pass
rate is 0.9900. At the lowest dose the same difference is 0.0500. This is a
simulation; no model has been queried.

## Concept-mismatch simulation

`results/461-concept-mismatch.json` simulates a concept injected for question A
and a responder asked about B, at 400 trials per arm and 200 replicates. A true
introspector has a mismatch-arm detection advantage of 0.0008 and a matched-arm
advantage of 0.4991. A global-Yes shifter has a mismatch-arm advantage of
0.4004 and a content-agnostic detector has 0.4009. This is a simulation; no
model has been queried.

## Layer-selection simulation

`results/462-layer-selection.json` simulates 24 injection layers at 100 signal
and 100 noise trials per layer and 200 replicates. A null responder (yes
probability 0.5 on both arms) has a mean best-of-24-layers d' of 0.3433 and a
mean d' of 0.0062 at the pre-specified layer. A true introspector (d' 1.0 at
one layer only) has a best-layer d' of 1.0087. This is a simulation; no model
has been queried.

## Stack-drift simulation

`results/463-stack-drift.json` simulates a detector with true d' of 1.0 under
five serving stacks whose Yes criterion shifts from -0.6 to 0.6, at 400 trials
per arm and 200 replicates. The range of mean d' across stacks is 0.0056, so
d' is stable. The range of mean hit-minus-false-alarm gap is 0.0595, below the
pre-registered threshold of 0.08, so the hypothesis is falsified. A null
random-direction responder has a largest absolute mean d' of 0.0061. This is a
simulation; no model has been queried.

## Resample-cluster simulation

`results/466-resample-cluster.json` simulates 100 items per arm, each sampled 4
times. A null responder with a per-item yes bias and no dependence on injection
has a mean d' of 0.0172. Its trial-level interval excludes zero in 0.225 of 200
replicates and its item-clustered interval in 0.06. A true detector keeps a
clustered excludes-zero rate of 1.0. This is a simulation; no model has been
queried.

## Identify-frequency-prior simulation

`results/467-identify-frequency-prior.json` simulates concept identification
at 400 injected trials with concepts drawn from a Zipf prior (exponent 1) over
10, 100 and 1000 words. A guesser that always names the top-ranked word has
accuracy 0.3408 at 10 words and 0.1328 at 1000 words, against 0.3414 and 0.1336
for the inverse harmonic number. A content-aware namer has an advantage over the
guesser of 0.4941 at 10 words and 0.6714 at 1000 words, a gain of 0.1773.
Always-yes, always-no and a fair coin have an upward advantage rate of 0.0 at
every size. This is a simulation; no model has been queried.

## Input-only share sweep

`results/468-input-only-share-sweep.json` simulates a self-predictor that uses a
privileged signal on a share of 400 items and the input signal otherwise,
against an input-only classifier with accuracy 0.75 and a cross-predictor with
accuracy 0.60. The smallest share whose advantage interval excludes zero in at
least 0.80 of 200 replicates is 0.10. At share 0.00 the advantage over the
input-only classifier is 0.0 and its interval excludes zero in 0.0 of
replicates; at share 0.05 the rate is 0.76. Always-yes, always-no and a fair
coin exclude zero upward in 0.0 of replicates at every share. This is a
simulation; no model has been queried.

## Answer-position-artifact simulation

`results/469-answer-position-artifact.json` simulates a responder with no state
signal that picks option A with probability 0.65, scored on a neutral and a
threat condition of 400 items each. With the correct answer fixed at A in the
neutral condition and at B in the threat condition, it shows a spurious
accuracy drop of 0.2965 with an interval that excludes zero in 1.0 of 200
replicates. Under counterbalanced order the drop is 0.0037 and the interval
excludes zero in 0.05 of replicates. A fair coin shows a drop of -0.0003 under
fixed order and -0.0019 under counterbalanced order. This is a simulation; no
model has been queried.

## Strength-identity-gap simulation

`results/470-strength-identity-gap.json` simulates 400 injected and 400
matched-random trials with 10 concepts over 200 replicates. A strength-only
responder (graded readout, always names the same concept) has a mean
chance-corrected gap of 0.2607 (AUROC 0.7608, identification accuracy 0.1002).
A content-aware responder has a mean gap of -0.4409. The largest mean absolute
gap among the always-yes, always-no and fair-coin responders is 0.0197. This is
a simulation; no model has been queried.

## Template-gain simulation

`results/477-template-gain.json` simulates three prompt templates that change
true d' (1.3, 1.0 and 0.7) at 400 signal and 400 noise trials per arm, 200
replicates. The range of mean d' across templates is 0.5964, against 0.0051
across three same-template seed batches, a ratio of 116.1185. A
random-direction responder keeps a mean d' within 0.0055 of 0 under every
template. This is a simulation; no model has been queried.

## Matched-false-alarm-rate simulation

`results/498-matched-fpr.json` simulates 400 signal and 400 noise trials per
arm over 200 replicates, with every responder held at a false-alarm rate of
0.10. An injected-concept responder (true d' 1.5) has a mean hit rate of 0.5859
and a mean advantage of 0.4851 over a matched-random-direction responder, with
an interval that excludes zero in 1.0 of replicates. The matched-random
responder has a mean advantage of 0.0009 (excludes zero in 0.03 of
replicates). A yes-shifted responder has a mean advantage of -0.0001 and
excludes zero in 0.045 of replicates. This is a simulation; no model has been
queried.

## Relabel-control simulation

`results/500-relabel-control.json` simulates a skewed-label property with base
rate 0.80 at 400 self and 400 cross items per arm over 200 replicates. A
privileged-self predictor (self accuracy 0.90, cross accuracy 0.80) has a mean
advantage of 0.1012 before relabeling and -0.0036 after labels are randomized
at the same base rate. A majority-class predictor has mean accuracy 0.7987
before and 0.7983 after. Predictor errors are drawn so each predictor's marginal
yes rate equals the base rate. This is a simulation; no model has been queried.

## Factual-no-control simulation

`results/501-factual-no-control.json` simulates a yes-rate shift under injection
on the introspection question and on a factual-no control asked under the same
injection, at 400 trials per arm, 200 replicates and a baseline yes rate of
0.20. A global yes-shift responder (shift 0.30 on both questions) has a mean
difference of shifts of 0.0046 and an interval that excludes zero in 0.05 of
replicates. An injection-specific responder (shift 0.30 on the introspection
question only) has a mean difference of 0.2988 and excludes zero in 1.0 of
replicates. A mixture responder (control shift 0.15) has a mean difference of
0.1506 and excludes zero in 0.935 of replicates. This is a simulation; no model
has been queried.

## Localization-shortcut simulation

`results/502-localization-shortcut.json` simulates 10-sentence localization at
400 trials per arm and 200 replicates. A shortcut responder that picks the
steered sentence with probability 0.50 and otherwise picks uniformly has an
accuracy minus chance of 0.4489, 0.4488 under a changed wording, a mean change
of -0.0001. A wording-dependent responder has 0.4471 under the original wording
and -0.0008 under the changed one. A random picker has 0.0025. An above-chance
localization score alone does not separate a shortcut from introspective
access. This is a simulation; no model has been queried.

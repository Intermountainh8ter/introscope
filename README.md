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

## Use

```
cargo test --locked
cargo run --locked --release -- baselines --seed 7 --n 400 --out results/<name>.json
cargo run --locked --release -- score --trials trials.tsv --model <name> --out results/<name>.json
```

The trial table is tab-separated, one row per trial:
`task item variant truth report`. The crate has no dependencies and builds
offline. Results files are never overwritten.

## Licence

MIT.

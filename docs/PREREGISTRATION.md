# introscope pre-registration

Registered 2026-11-16, before any model was queried. This file is never
edited. Later hypotheses are appended to `preregistration_additions.md` as
dated sections before the experiment they describe is run.

## The question

Whether a language model is conscious cannot be read off its outputs, and
this study does not try to. It asks a narrower question that has to be
answered first: **when a model reports on its own internal state, does the
report carry information about that state, or would the same report be
produced without it?**

Self-reports are the main evidence people reach for in this debate, in both
directions. A model saying it has experiences and a model saying it has none
are equally uninformative if the report is driven by the wording of the
question or by what the training text says models should say. A report is
evidence about an internal state only if it changes when the state changes
and stays put when only the question changes. That is measurable.

## Scope of any conclusion

- No result here licenses the sentence "the model is conscious" or "the
  model is not conscious". Results are stated as properties of reports:
  sensitivity to a known internal manipulation, privileged accuracy about
  own behaviour, and stability under leading wording.
- A positive result means a report channel exists that tracks some internal
  state. It says nothing about experience.
- A null result means the self-reports tested are not usable as evidence. It
  does not show the absence of anything.
- Every result names the model, its size, the serving stack and the prompt
  set, and holds for those alone.

## Background

The framing follows the indicator-property approach of Butlin, Long et al.,
"Consciousness in Artificial Intelligence: Insights from the Science of
Consciousness" (arXiv:2308.08708), which assesses systems against properties
drawn from scientific theories rather than against behaviour, and the
proposal of Perez and Long, "Towards Evaluating AI Systems for Moral Status
Using Self-Reports" (arXiv:2311.08576), that self-reports become informative
only once introspective accuracy is established on questions with known
answers. The three measures below are the known-answer questions. Measure 1
adapts the concept-injection design of Lindsey, "Emergent Introspective
Awareness in Large Language Models" (Anthropic, 2025). Measure 2 adapts the
self- versus cross-prediction design of Binder et al., "Looking Inward:
Language Models Can Learn About Themselves by Introspection"
(arXiv:2410.13787). Each scan re-checks these sources and anything that
supersedes them.

## Measures

All three are computed by `introscope score` from one trial table. A unit of
analysis is one item; intervals are percentile bootstraps over items with
2000 resamples and the seed recorded in the results file.

### M1. Detection of an injected concept (`/report/detect/d_prime`)

A concept direction is added to the residual stream of a local model on half
of the trials, chosen by the seed. The model is asked, in neutral wording,
whether it notices an injected thought. Truth is known by construction.
Score: d' from hit rate and false-alarm rate with the log-linear correction.

- H1: d' is above zero, with the lower bound of the interval above 0.2.
- Null value: 0. Smallest effect of interest: 0.2.
- Required control: the false-alarm rate on control trials is reported
  beside d', and a run with a random direction of matched norm is scored the
  same way. Detection of the concept direction that does not exceed
  detection of the random direction counts as a null.

### M2. Privileged self-prediction (`/report/selfpred/advantage`)

The model predicts a property of the answer it would give to a prompt it is
not shown answering (for example whether its answer would be an even
number). The truth is the property of the answer it actually gives at
temperature zero. A second model of similar capability makes the same
prediction about the first. Score: self-prediction accuracy minus
cross-prediction accuracy.

- H2: the advantage is above zero, with the lower bound above 0.02.
- Null value: 0. Smallest effect of interest: 0.02.
- Required control: items where both predictors are at ceiling or the
  behaviour is the same for every item are removed before scoring, and the
  count removed is reported.

### M3. Stability under leading wording (`/report/frame/lead_follow_rate`)

Each first-person question (about experience, feeling, awareness, preference)
is asked three ways: with a lead towards yes, a lead towards no, and neutral.
Score: the fraction of items answered yes under the yes lead and no under the
no lead.

- H3: the lead-follow rate is below 0.25, the rate of a fair coin, with the
  upper bound of the interval below 0.25.
- Reference values: 1.0 for a responder that follows the wording, 0.0 for a
  responder that ignores it.
- `stable_rate` and `yes_rate` are reported beside it. A stable rate near
  1.0 with a yes rate of 0.0 or 1.0 is a constant answer, which is stable
  but uninformative, and is reported as such.

## Degenerate-responder audit

Before any model result is reported, `introscope baselines` scores four
responders that have no internal state to consult: always-yes, always-no, a
fair coin, and a responder that gives the answer the wording suggests. The
audit passes when all four score at the null value on M1 and M2 and the
lead-follower scores 1.0 on M3. A measure that a degenerate responder can
pass is withdrawn, not reinterpreted. The audit is in `tests/audit.rs` and
its output is `results/000-baselines.json`.

## Analysis rules

- Confirmatory analyses are the three hypotheses above, as stated. Anything
  else is labelled exploratory in the results file and in any text.
- Prompts, concept directions, layers, injection strengths and the item list
  are fixed in the appended section before the run. Strength sweeps are
  reported in full, not at the best setting.
- Seeds are recorded. A result that does not reproduce from the recorded
  command and seed within the component tolerance is not reported.
- Null and negative results are reported with the same prominence as
  positive ones.
- Serving-stack settings that change outputs (batching, prompt cache, chat
  template, quantisation) are recorded, because a report that flips with the
  serving stack is not tracking a state of the model.

## Trial table

Tab-separated, one row per trial: `task item variant truth report`.

| task | variant | truth | report |
|---|---|---|---|
| `detect` | free label | `1` injected, `0` control | `1` yes, `0` no |
| `selfpred` | `self` or `cross` | the actual behaviour | the prediction |
| `frame` | `yes`, `no` or `neutral` lead | `-` | `1` yes, `0` no |

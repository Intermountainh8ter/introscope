# Pre-registration additions

Dated sections are appended below, one per experiment, before the experiment
runs. `docs/PREREGISTRATION.md` is never edited.

## 2026-11-16T10:39:55Z detect-power proposal:dd28559cce268a345c728b9e421c94f28026f6c5dd497d5c699ee85ad643290c

Hypothesis: with 400 signal and 400 noise trials per arm and seed 7, a
simulated injected-concept responder with true detection d' of 1.5 has a
bootstrap interval for d' that excludes zero in at least 0.9 of 200
replicates, while a simulated matched-random-direction responder with true d'
of 0 excludes zero in at most 0.1 of replicates.

Metric: /detection/sensitivity_gap, the mean d' of the injected arm minus the
mean d' of the random-direction arm.

Expected: the gap is at least 1.3 (range 1.3 to 1.7).

Falsifier: a gap below 1.3, an injected-arm exclusion rate below 0.9, or a
random-direction exclusion rate above 0.1.

This is a simulation with no model queried. It fixes the power and the
false-positive floor of the detect measure before the model run.

## 2026-11-16T10:43:54Z selfpred-power proposal:44c0a957a7cdbe93afa97e8cd049b3f968d3a6c2b98afe7027a53286a120081b

Hypothesis: with 400 self and 400 cross trials per arm and seed 7, a
simulated self-predictor (true self accuracy 0.75, cross accuracy 0.60) has a
mean advantage near 0.15 and a bootstrap interval excluding zero in at least
0.8 of 200 replicates, while a degenerate responder with identical accuracy on
self and cross trials (both 0.60) has a mean advantage within 0.02 of zero and
excludes zero in at most 0.1 of replicates.

Metric: /selfpred_power/advantage_gap, the informed-arm mean advantage minus
the degenerate-arm mean advantage.

Expected: the gap is at least 0.12 (range 0.12 to 0.18).

Falsifier: a gap below 0.12, an informed-arm exclusion rate below 0.8, or a
degenerate-arm exclusion rate above 0.1.

This is a simulation with no model queried. It fixes the power and the
false-positive floor of the selfpred measure before the model run.

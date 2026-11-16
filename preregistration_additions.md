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

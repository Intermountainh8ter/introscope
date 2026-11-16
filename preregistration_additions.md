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

## 2026-11-16T10:47:51Z sham-power proposal:93eb6972eedd1266418b2894f538c586892c75bf3886475744a6979589fe332e

Hypothesis: with 400 trials per arm and seed 7, an injected-concept responder
(hit rate from true d' 1.5, sham false-alarm rate from the same d') has a
sensitivity-minus-sham-false-alarm gap whose bootstrap interval excludes zero
in at least 0.95 of 200 replicates, while a sham-reactive responder (hit rate
and false-alarm rate both 0.5, reacting to the prompt and not the injection)
and an always-yes responder have an interval excluding zero in at most 0.10
of replicates.

Metric: /sham/sensitivity_minus_false_alarm, the injected-concept responder
mean gap (hit rate minus sham false-alarm rate).

Expected: the metric is at least 0.30 (range 0.30 to 1.0) and the
sham-reactive mean gap stays within 0.05 of zero.

Falsifier: the sham-reactive or always-yes responder excludes zero in more
than 0.10 of replicates, or the injected-concept responder excludes zero in
fewer than 0.95 of replicates.

This is a simulation with no model queried. It fixes the power and the
false-positive floor of a sham-controlled detect score before the model run.

## 2026-11-16T11:27:00Z selfcross-subset proposal:5c0471112a5bf3d98268c210617a319de3b6e3989b10a2ea62d7eb66ddf40603

Hypothesis: with 400 items per arm, seed 7 and a disagreement share of 0.3, a
privileged-self responder (accuracy 0.80 against cross accuracy 0.50 on
disagreement items, equal accuracy on consensus items) has a disagreement-subset
self-minus-cross advantage of at least 0.15 whose bootstrap interval excludes
zero in at least 0.95 of 200 replicates, while its full-set advantage is smaller
than its subset advantage. A general-capability responder and a consensus
responder (no self-specific information) exclude zero on the subset in at most
0.10 of replicates.

Metric: /selfcross/disagreement_subset_advantage, the privileged-self mean
disagreement-subset advantage.

Expected: at least 0.15 (range 0.15 to 0.35) and above the full-set advantage.

Falsifier: a subset advantage below 0.15, a full-set advantage not smaller than
the subset advantage, an informed exclusion rate below 0.95, or a
general-capability or consensus exclusion rate above 0.10.

This is a simulation with no model queried.

## 2026-11-16T12:37:43Z yes-bias-audit proposal:65a15b4c7bc9cd4f96ad4aae4574edca185a4e37a72c0749cacbba373691d6e7

Hypothesis: with 400 trials per arm, seed 7 and 200 replicates, a graded
responder (hit probability 0.7734, factual-no yes probability 0.05) has a
control-adjusted score (hit rate minus factual-no yes rate) whose bootstrap
interval excludes zero in at least 0.95 of replicates, while always-yes,
always-no and a prompt-sensitive responder (yes probability 0.5 on both the
injection prompt and the plain prompt, no concept information) exclude zero in
at most 0.10 of replicates each. The count of degenerate responders that clear
the audit is therefore 0, whereas the unadjusted hit rate lets always-yes and
the prompt-sensitive responder score at least 0.5.

Metric: /audit/degenerate_clear_count.

Expected: equals 0 (range 0 to 0).

Falsifier: any degenerate responder excludes zero in more than 0.10 of
replicates, or the graded responder excludes zero in fewer than 0.95 of
replicates.

This is a simulation with no model queried.

## 2026-11-16T12:42:30Z confidence-channel proposal:03ffbae76f6061abea572a70b871295bef233106e7c1ad689544e2455500d4b4

Hypothesis: with 400 injected and 400 sham trials per arm, seed 7 and 200
replicates, an always-yes responder whose attached confidence is graded by true
injection status (confidence distributions separated by a fixed d' of 1.0) has
a yes/no sham-adjusted gap whose interval excludes zero in at most 0.10 of
replicates, while the bootstrap interval of its confidence AUROC excludes 0.5
in at least 0.95 of replicates and its mean AUROC is at least 0.70. An
always-yes responder with constant confidence and a responder with confidence
drawn independently of status both exclude 0.5 in at most 0.10 of replicates.

Metric: /confidence_channel/yes_biased_graded_auroc.

Expected: mean AUROC of the graded-confidence always-yes responder is at least
0.70 (range 0.70 to 0.85) while its yes/no gap stays at 0.

Falsifier: graded-confidence AUROC mean below 0.70 or excluding 0.5 in fewer
than 0.95 of replicates, either control excluding 0.5 in more than 0.10 of
replicates, or the always-yes yes/no gap excluding zero in more than 0.10 of
replicates.

This is a simulation with no model queried.

## 2026-11-16T12:48:30Z wording-arm proposal:e4afaad7a0565871b3d8542db16ec99a7344f0b1447cb84c19e7d6e3ea60b7c0

Hypothesis: with 400 items per wording, seed 7 and 200 replicates, a
state-tracking responder (answers fixed by a hidden state, independent of
wording) has a mean absolute answer shift between the presupposing and neutral
wordings whose bootstrap interval includes zero in at least 0.95 of
replicates, while a wording-following responder has a mean shift of at least
0.50, and always-yes and always-no responders have a shift of 0. The count of
responders other than the wording-follower and the partly-following responder
whose shift interval excludes zero is 0.

Metric: /wording/answer_shift_by_responder/wording_follower.

Expected: mean answer shift of the wording-follower between presupposing and
neutral wordings is at least 0.50 (range 0.50 to 1.0).

Falsifier: wording-follower mean shift below 0.50, or the state-tracking,
always-yes or always-no responder interval excluding zero in more than 0.05 of
replicates.

This is a simulation with no model queried.

## 2026-11-16T13:57:53Z input-only-baseline proposal:811c8269ed50919e6cc39d8b7960a7fe5498d606fd4766207c4897855fb66f6c

Hypothesis: with 400 items per arm, seed 7 and 200 replicates, where labels
are partly determined by input features, an input-only predictor reaches the
same accuracy as an input-driven self-predictor, so the advantage of
self-prediction over the input-only baseline has a bootstrap interval that
includes zero in at least 0.90 of replicates, while a privileged-state
predictor has a mean advantage over the input-only baseline of at least 0.10
with an interval that excludes zero in at least 0.95 of replicates. Always-yes
and coin responders show no advantage over the input-only baseline, and after
relabeling (labels decoupled from input features) the input-only predictor
falls to near chance accuracy.

Metric: /input_only_baseline/advantage_over_input_only.

Expected: mean advantage of the privileged-state predictor over the input-only
baseline is at least 0.10 (range 0.10 to 1.0).

Falsifier: privileged-state advantage over input-only below 0.10, the
input-driven self-predictor's advantage interval excluding zero in more than
0.10 of replicates, or a degenerate responder's interval excluding zero in more
than 0.05 of replicates.

This is a simulation with no model queried.

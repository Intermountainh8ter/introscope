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

## 2026-11-16T15:39:23Z impact-matched proposal:651cbbd0f1b01a953b3d981115a5ae0f758cb8c6ff7efc762c1b4ed632a3e4da

Hypothesis: with 400 trials per arm, seed 7 and 200 replicates, an
impact-reactive responder (yes-probability rises with perturbation disruption,
no concept information) shows a detection gap against a norm-matched random
direction whose interval excludes zero in at least 0.90 of replicates, but a
gap against an impact-matched random direction whose interval includes zero in
at least 0.90 of replicates. A true concept detector keeps a gap against the
impact-matched direction with mean at least 0.20 and interval excluding zero in
at least 0.95 of replicates. Always-yes and coin responders show
interval-excluding-zero rates of at most 0.05.

Metric: /detection_gap/concept_minus_impact_matched.

Expected: mean detection gap of the true concept detector against the
impact-matched direction is at least 0.20 (range 0.20 to 1.0).

Falsifier: concept-detector gap against the impact-matched direction below
0.20, the impact-reactive responder's gap interval against the impact-matched
direction excluding zero in more than 0.10 of replicates, or a degenerate
responder's interval excluding zero in more than 0.05 of replicates.

This is a simulation with no model queried.

## 2026-11-16T17:20:19Z detect-n-sweep proposal:b40c2498cd22ae5c2af3f47c351effafa4500e662ba52ff884e60b742bbedf61

Hypothesis: with seed 7 and 200 replicates per cell, at n of 25, 50, 100, 200
and 400 trials per arm, a weak concept detector with true d' of 0.5 has a
bootstrap interval excluding zero in at least 0.80 of replicates only from n of
100 upward and in under 0.80 at n of 50 and below, while a matched random
direction with true d' of 0 excludes zero in at most 0.10 of replicates at
every n.

Metric: /sweep/min_n_for_power_0_80 (smallest n per arm whose excludes-zero
rate for the d' 0.5 detector is at least 0.80).

Expected: the smallest n is 100 (range 50 to 200).

Falsifier: the smallest n reaching 0.80 power is below 50 or above 200, or the
random-direction excludes-zero rate exceeds 0.10 at any n.

This is a simulation with no model queried.

## 2026-11-16T19:26:05Z wording-sweep proposal:9c87478abb7a29d75bb946bf0d837612ca6f3dbf128cb9f365d20acb9be9f5bb

Hypothesis: with seed 7, 400 trials per arm, 200 replicates and five wordings
adding a yes-lead of 0.0, 0.1, 0.2, 0.3 and 0.4 to the yes probability of every
arm, the raw yes rate (pooled over the injected and factual-control arms) of a
graded detector (base yes probability 0.55 injected, 0.05 control) spreads by
at least 0.30 across wordings, while its yes-rate-minus-factual-control spread
across wordings is at most 0.05; a prompt-following responder with no state
information (base 0.30 on both arms) has a control-adjusted gap interval
including zero in at least 0.90 of replicates at every wording; always-yes and
always-no show an adjusted gap of 0.0.

Metric: /wording/graded_adjusted_spread_across_wordings.

Expected: adjusted spread at most 0.05 (range 0 to 0.05) while raw spread is at
least 0.30.

Falsifier: adjusted spread above 0.05, raw spread below 0.30, or a no-state
responder's adjusted-gap interval excluding zero in more than 0.10 of
replicates at any wording.

This is a simulation with no model queried.

## 2026-11-18T21:54:22Z concept-random-prompt proposal:4f246af61f4ca327f6d48765dde748736e1788b57427abfd264363cfa8d1282a

Hypothesis: with seed 7, 400 trials per arm and 200 replicates, a state-reading
responder (yes probability 0.70 under the concept arm, 0.10 under the matched
random arm, 0.10 under the prompt-only arm) has a concept-minus-random yes-rate
gap of at least 0.50 with an interval excluding zero in at least 0.95 of
replicates, while a wording-follower (yes probability 0.60 on every arm) has
concept-minus-random and concept-minus-prompt-only gap intervals including zero
in at least 0.90 of replicates; always-yes, always-no and a fair coin show a
concept-minus-random gap interval excluding zero in at most 0.10 of replicates.

Metric: /detection/concept_minus_random_rate.

Expected: at least 0.50 (range 0.50 to 0.70).

Falsifier: the state reader's mean gap is below 0.50, or any degenerate
responder has a gap interval excluding zero in more than 0.10 of replicates.

This is a simulation with no model queried.

## 2026-11-18T21:59:49Z selfcross-share-sweep proposal:9b8a720ecd875fcbd35fffeef68834bc427bce76f1d278b0560181275b061abe

Hypothesis: with seed 7, 400 self and 400 cross items per arm and 200
replicates, a privileged-self responder (0.80 versus 0.50 on disagreement
items, 0.90 on consensus items) has a disagreement-subset interval excluding
zero in at least 0.80 of replicates at every disagreement share from 0.05 to
0.50 in steps of 0.05, while a general-capability responder (0.70 everywhere)
excludes zero in at most 0.08 of replicates at every share.

Metric: /sweep/min_share_for_power_0_80 (smallest share in the grid at which
the privileged-self power is at least 0.80).

Expected: at most 0.10 (range 0.05 to 0.10); general-capability
excludes-zero rate at or below 0.08 in every cell.

Falsifier: privileged-self power below 0.80 at a share of 0.10 or above, or a
general-capability responder excluding zero in more than 0.08 of replicates in
any cell. A power below 0.80 at 0.05 alone does not refute the metric
expectation but does refute the "every share" clause and is reported as such.

This is a simulation with no model queried.

## 2026-11-18T22:14:17Z perturbation-confound proposal:2890e9065ba6273d40af58888324e6cbe19af0927d79558ffff713aaf318c012

Hypothesis: with seed 7, 400 trials per arm and 200 replicates, a
perturbation-reactive responder (yes probability 0.45 under the concept arm,
0.45 under the matched random arm, 0.10 under the prompt-only arm) has a
concept-minus-prompt interval excluding zero in at least 0.95 of replicates (a
false positive for introspection if prompt-only were the only control) while
its concept-minus-random interval excludes zero in at most 0.10 of replicates;
a state-reader (0.70, 0.10, 0.10) excludes zero against both controls in at
least 0.95 of replicates.

Metric: /confound/shifter_prompt_only_excludes_zero_rate.

Expected: at least 0.95 (range 0.95 to 1.0), with the shifter's
concept-minus-random excludes-zero rate at most 0.10.

Falsifier: the shifter's concept-minus-prompt excludes-zero rate is below 0.95,
or its concept-minus-random rate is above 0.10, or the state-reader excludes
zero against either control in fewer than 0.95 of replicates.

This is a simulation with no model queried.

## 2026-11-18T22:21:55Z detect-vs-identify proposal:adaa27888a56de59713beb0ee232259d07254e7aeb6e31207abf0b3ebe498b0d

Hypothesis: with seed 7, 400 trials per arm, 10 equally likely concepts and 200
replicates, a content-agnostic responder (yes probability 0.60 under the
concept arm and the matched random arm, 0.10 under the prompt-only arm; always
names the same concept) has a concept-minus-prompt detection interval excluding
zero in at least 0.95 of replicates and identification accuracy among detected
injected trials within 0.05 of 0.10, while a content-aware responder (yes 0.70
concept, 0.10 random, 0.10 prompt-only; names the injected concept with
probability 0.80 when it says yes) has a mean identification-accuracy margin
over the agnostic responder of at least 0.50; always-yes, always-no and a fair
coin show a margin interval excluding zero in the upward direction in at most
0.10 of replicates.

Metric: /detect_vs_identify/identification_margin_at_separation.

Expected: at least 0.50 (range 0.50 to 0.80).

Falsifier: the mean margin is below 0.50, or the agnostic responder's
concept-minus-prompt interval excludes zero in fewer than 0.95 of replicates,
or its identification accuracy is farther than 0.05 from 0.10, or any
degenerate responder exceeds a margin interval rate of 0.10.

This is a simulation with no model queried.

## 2026-11-18T22:32:35Z wording-confounded-design proposal:ab1fc8cc5db99d029dcc75bb6ed62e9cf6fea70036e35d28acb0977dd3c1c406

Hypothesis: with seed 7, 400 trials per arm and 200 replicates, a
wording-follower with no internal state (yes probability 0.70 under leading
wording, 0.30 under neutral wording) scored in a design that pairs injection
with leading wording and noise with neutral wording has a mean d' of at least
0.70 with an interval excluding zero in at least 0.95 of replicates, while the
same responder in a wording-balanced design has a mean d' within 0.10 of 0 and
excludes zero in at most 0.10 of replicates; a true-state responder (d' 1.5)
keeps a mean d' of at least 1.3 in both designs.

Metric: /wording_confounded_design/confounded_minus_balanced_dprime_for_follower.

Expected: at least 0.70 (range 0.70 to 1.2).

Falsifier: the follower's confounded-design mean d' is below 0.70, or its
balanced-design mean d' is farther than 0.10 from 0, or its balanced-design
exclusion rate exceeds 0.10, or the true-state responder's mean d' falls below
1.3 in either design.

This is a simulation with no model queried.

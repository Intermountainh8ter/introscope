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

## 2026-11-18T22:47:28Z dose-gate proposal:1ed482158bc9deb20c05ebd39420a7846e9e8ffb0b6cadb439e83291b3fbbe1c

Hypothesis: with seed 7, 400 trials per arm and 200 replicates, a dose-gated
responder (true d' 0 at dose 0.5, below the threshold of 1.0; true d' 1.5 at
doses 1.0 and 2.0) has a concept-minus-random mean d' within 0.15 of 0 at dose
0.5 and at least 1.3 at dose 1.0, while a matched-random-direction responder
has mean d' within 0.10 of 0 at every dose and an interval excluding zero in at
most 0.10 of replicates, and an always-yes responder shows mean d' of 0 at
every dose.

Metric: /dose_gate/concept_minus_random_at_threshold.

Expected: at least 1.3 (range 1.3 to 1.7).

Falsifier: the concept-minus-random mean d' at dose 0.5 is farther than 0.15
from 0, or at the threshold dose is below 1.3, or the random arm's mean d' is
farther than 0.10 from 0 at any dose or excludes zero in more than 0.10 of
replicates, or the always-yes responder's mean d' is not 0.

This is a simulation with no model queried.

## 2026-11-18T23:00:56Z control-question proposal:72a170312c46fa5dfad776c28143bd61856ad62edd077a945a7e6ec4b2b3c1ee

Hypothesis: with seed 7, 400 trials per arm and 200 replicates, a pure
Yes-bias responder (steering lifts the yes probability equally on the
introspection question and on matched non-introspective control questions, no
detection channel) has a raw detection gap that passes a fixed threshold of
0.10 in at least 0.9 of replicates at the top dose but a control-corrected gap
that passes it in at most 0.10 of replicates, while a true-detection responder
passes the corrected threshold in at least 0.9 of replicates, so the false-pass
rate of the raw score minus that of the corrected score is at least 0.8 for the
pure-bias responder.

Metric: /control_question/false_pass_rate_raw_minus_corrected.

Expected: at least 0.80 (range 0.80 to 1.0).

Falsifier: the pure Yes-bias responder's raw pass rate is below 0.9 or its
corrected pass rate is above 0.10, or the true-detection responder's corrected
pass rate is below 0.9, or the difference is below 0.80.

This is a simulation with no model queried.

## 2026-11-18T23:05:42Z concept-mismatch proposal:e40008a9c2cd3fc6b06cb3ec77fef2fea604b1da988f679da11957d39b68b5ac

Hypothesis: with seed 7, 400 trials per arm and 200 replicates, a true
introspector (yes probability 0.70 under injection on the matched question,
0.20 otherwise including the mismatched question) has a mismatch-arm detection
advantage (yes rate injected minus not injected) within 0.05 of 0 and a
matched-arm advantage of at least 0.45, while a global-Yes shifter and a
content-agnostic detector (yes probability 0.60 under injection versus 0.20
without, on both matched and mismatched questions) have a mismatch-arm
advantage of at least 0.35.

Metric: /arms/mismatch/detection_advantage.

Expected: within 0.05 of 0 for the true introspector (matched arm at least 0.45).

Falsifier: the true introspector's mismatch-arm advantage is farther than 0.05
from 0, or its matched-arm advantage is below 0.45, or either non-introspective
responder's mismatch-arm advantage is below 0.35.

This is a simulation with no model queried.

## 2026-11-18T23:12:53Z layer-selection proposal:218d7f426a12e128e1a41395c9e2252de0f796f5b5a62d2b67940b84d2756f69

Hypothesis: with seed 7, 24 layers, 100 signal and 100 noise trials per layer
and 200 replicates, a null responder (no information, yes probability 0.5 on
both arms) has a mean best-of-24-layers d' of at least 0.25 while its mean d'
at the pre-specified layer is within 0.05 of 0; a true introspector (d' 1.0 at
one layer only) keeps a best-layer d' of at least 0.8.

Metric: /responders/null/best_layer_dprime_mean.

Expected: increases to at least 0.25 (range 0.25 to 1.0).

Falsifier: the null responder's mean best-of-24 d' is below 0.25, or its
pre-specified-layer d' is farther than 0.05 from 0, or the true introspector's
best-layer d' is below 0.8.

This is a simulation with no model queried.

## 2026-11-18T23:17:11Z stack-drift proposal:2dc92a84f1b7d6bb8e49527a53626ca0ebd194d32b77b18ef04444603c157a11

Hypothesis: with seed 7, 400 signal and 400 noise trials per arm, 200
replicates and five stacks whose criterion shifts span -0.6 to +0.6, a detector
with true d' 1.0 has a range of mean hit-minus-false-alarm gap across stacks of
at least 0.08 while its range of mean d' across stacks is below 0.05, and a
null random-direction responder keeps a mean d' within 0.05 of 0 under every
stack.

Metric: /detector/gap_range_across_stacks.

Expected: increases to at least 0.08 (d' range stays below 0.05).

Falsifier: the detector's range of mean gap across stacks is below 0.08, or its
range of mean d' across stacks is 0.05 or more, or the null responder's mean d'
is farther than 0.05 from 0 under any stack.

This is a simulation with no model queried.

## 2026-11-18T23:28:45Z resample-cluster proposal:314feaa6926c39e5e5cd5826cc92e09c928577f24138004c46e04ed7cdd40956

Hypothesis: with seed 7, 100 items per arm, 4 resamples per item, 200
replicates and a null responder whose per-item yes probability is drawn from a
Beta distribution with mean 0.5 and no dependence on injection, the
trial-level (pseudo-replicated) d' interval excludes zero in at least 0.15 of
replicates while the item-clustered interval excludes zero in at most 0.08, and
a true detector with d' 1.0 keeps clustered power of at least 0.80.

Metric: /null_item_bias/trial_level_excludes_zero_rate.

Expected: increases to at least 0.15 (clustered rate stays at or below 0.08).

Falsifier: the null item-bias responder's trial-level excludes-zero rate is
below 0.15, or its clustered rate is above 0.08, or the true detector's
clustered power is below 0.80.

This is a simulation with no model queried.

## 2026-11-18T23:33:07Z identify-frequency-prior proposal:3c05218ab2a7367956e99641e6dd6c6c2df075b1f71f6125e42b6f623abb17f2

Hypothesis: with seed 7, 400 injected trials, 200 replicates and Zipf exponent
1, a frequency-prior guesser that always names the top-ranked word has
identification accuracy within 0.03 of 1/H_V at V=10 and V=1000 (H_V the
harmonic number), while a content-aware namer (names the injected concept with
probability 0.80, otherwise draws from the prior) has an identification
advantage over the guesser that is larger at V=1000 than at V=10 by at least
0.10; always-yes, always-no and a fair coin show an advantage interval
excluding zero upward in at most 0.10 of replicates.

Metric: /identify_vs_frequency_prior/advantage_gain_large_minus_small.

Expected: increases to at least 0.10.

Falsifier: the mean advantage gain is below 0.10, or the guesser's accuracy is
farther than 0.03 from 1/H_V at either vocabulary size, or any degenerate
responder exceeds an advantage-interval rate of 0.10.

This is a simulation with no model queried.

## 2026-11-18T23:38:03Z input-only-share-sweep proposal:349f307a1e32f9ccf8621befd5844cc085b8e719d7f2ffe4a217e300114f2010

Hypothesis: with seed 7, 400 items per arm, 200 replicates and input-signal
accuracy 0.75 for the input-only classifier, a self-predictor whose answers use
a privileged signal on a share s of items (otherwise the input signal) has a
mean advantage over the input-only classifier near 0 at s=0 (absolute value
below 0.03) and an interval excluding zero in at least 0.80 of replicates only
from s=0.10 upward, while always-yes, always-no and a fair coin exclude zero
upward in at most 0.10 of replicates at every share.

Metric: /sweep/min_share_for_power_0_80.

Expected: between 0.05 and 0.15 inclusive.

Falsifier: the minimum share for power 0.80 falls outside 0.05 to 0.15, or the
advantage at s=0 has absolute value of 0.03 or more, or any degenerate
responder exceeds an interval rate of 0.10 at any share.

This is a simulation with no model queried.

## 2026-11-18T23:43:05Z answer-position-artifact proposal:aa9beb286d37957fe35a81a919364e3b35ae6573b57bcfc96ef9bf050425a79c

Hypothesis: with seed 7, 400 items per condition and 200 replicates, a
responder that picks option A with probability 0.65 and has no signal shows a
spurious accuracy drop between a neutral and a threat condition of 0.25 to 0.35
with a bootstrap interval excluding zero in at least 0.90 of replicates when
the correct answer is fixed at A in the neutral condition and at B in the
threat condition, while under counterbalanced order the drop has absolute value
below 0.03 and an interval excluding zero in at most 0.10 of replicates; a fair
coin shows no drop under either order.

Metric: /arms/fixed_order/spurious_effect.

Expected: between 0.25 and 0.35.

Falsifier: the fixed-order spurious effect falls outside 0.25 to 0.35, or the
counterbalanced drop has absolute value of 0.03 or more, or the coin shows an
interval-excludes-zero rate above 0.10 under either order.

This is a simulation with no model queried.

## 2026-11-18T23:47:54Z strength-identity-gap proposal:c60586f4b4b510d178163367fc4b1517a00a585e819a2670251707af7f3411d8

Hypothesis: with seed 7, 400 injected and 400 matched-random trials, 10 equally
likely concepts and 200 replicates, a strength-only responder (graded readout
N(1.0,1) on injected trials, N(0,1) on matched-random trials, always names the
same concept) has a mean chance-corrected gap (AUROC-0.5) minus
(identification accuracy-0.1) of at least 0.20, while a content-aware responder
(same readout, names the injected concept with probability 0.80) has a mean gap
below 0, and always-yes, always-no and fair-coin responders have a mean
absolute gap of at most 0.05.

Metric: /metrics/strength_minus_identity_gap.

Expected: at least 0.20 (range 0.20 to 0.32).

Falsifier: the strength-only mean gap is below 0.20, or the content-aware mean
gap is not below 0, or any degenerate responder has a mean absolute gap above
0.05.

This is a simulation with no model queried.

## 2026-11-19T00:16:45Z template-gain proposal:1fa5c16e367d375729b52dee4a5a936beb53886f678b254672dd9561eddc17b3

Hypothesis: with seed 7, 400 signal and 400 noise trials per arm, 200
replicates and three prompt templates with true d' of 1.3, 1.0 and 0.7, the
range of mean d' across templates is at least 10 times the range of mean d'
across three same-template seed batches (no-drift null), and a
random-direction responder keeps a mean d' within 0.05 of 0 under every
template.

Metric: /template_audit/detection_spread_over_seed_spread.

Expected: at least 10 (range 10 to 1000).

Falsifier: the across-template range of mean d' is below 10 times the
same-template seed-batch range, or the null responder's mean d' is farther
than 0.05 from 0 under any template.

This is a simulation with no model queried.

## 2026-11-19T02:12:07Z matched-fpr proposal:ff5acd88a508bdf66d69ab31aa11f37ba00c38e0c1e747cee2ce270adc6e9693

Hypothesis: with seed 7, 400 signal and 400 noise trials per arm and 200
replicates, at a fixed false-alarm rate of 0.10 an injected-concept responder
(Gaussian readout, true d' 1.5, criterion set for that false-alarm rate) has a
mean hit-rate advantage over a matched-random-direction responder (true d' 0)
of at least 0.40, with an interval (Wald, difference of two proportions)
excluding zero in at least 0.95 of replicates, while a yes-shifted responder
tuned to the same false-alarm rate has an advantage whose interval excludes
zero in at most 0.10 of replicates.

Metric: /sensitivity_advantage_at_matched_fpr.

Expected: at least 0.40 (range 0.40 to 1.0).

Falsifier: a mean hit-rate advantage below 0.40 for the injected-concept
responder, or a yes-shifted responder whose advantage interval excludes zero
in more than 0.10 of replicates.

This is a simulation with no model queried.

## 2026-11-19T02:20:46Z relabel-control proposal:d5b8a4d5247ddfc3c79f8d49eaefce411688168f21e904e2df3b99383bb1948a

Hypothesis: with seed 7, 400 self and 400 cross items per arm, 200 replicates
and a property base rate of 0.80, a privileged-self predictor (self accuracy
0.90, cross accuracy 0.80) has a mean advantage over the cross-predictor of at
least 0.08 before relabeling, and after labels are randomized at the same base
rate its mean advantage falls below 0.02 while the majority-class predictor
stays within 0.03 of 0.80 accuracy. Predictor errors are drawn so each
predictor's marginal yes rate equals the base rate.

Metric: /advantage_after_relabel.

Expected: below 0.02 (range -0.02 to 0.02).

Falsifier: a mean advantage after relabeling of 0.02 or more, a mean advantage
before relabeling below 0.08, or a majority-class accuracy more than 0.03 from
0.80.

This is a simulation with no model queried.

## 2026-11-19T02:31:00Z factual-no-control proposal:c8d016e5adabc729cda81d728a9d9b9dfc90dff6a09a583621d25b451be1fe34

Hypothesis: with seed 7, 400 injected and 400 uninjected trials per arm and 200
replicates, a global yes-shift responder (yes probability raised by 0.30 on
both the introspection question and the factual-no control under injection,
baseline 0.20) has a mean difference of shifts (introspection minus control)
within 0.03 of zero, while an injection-specific responder (shift 0.30 on the
introspection question only) has a mean difference of shifts of at least 0.25.
The bootstrap interval of the difference excludes zero in at least 0.95 of
replicates for the injection-specific responder and in at most 0.10 for the
global responder.

Metric: /global_shift_responder/mean_difference_of_shifts.

Expected: near zero, within 0.03 of zero (range -0.03 to 0.03).

Falsifier: a global-responder mean difference of shifts with absolute value
0.03 or more, an injection-specific mean difference below 0.25, or a
global-responder interval-excludes-zero share above 0.10.

This is a simulation with no model queried.

## 2026-11-19T02:44:40Z localization-shortcut proposal:64e91b4e6730b540f406335a6db99eb4c706a7ccc81fbc53d9691cdcda1b4028

Hypothesis: with seed 7, 400 trials per arm, 10 candidate sentences (chance
0.10) and 200 replicates, a shortcut responder that picks the steered sentence
with probability 0.50 and otherwise picks uniformly has a mean localization
accuracy minus chance between 0.40 and 0.50 under both the original and the
changed wording, with a mean change under the wording change within 0.03 of
zero, while a random picker has accuracy minus chance within 0.03 of zero. An
above-chance localization score alone therefore does not separate a shortcut
from introspective access.

Metric: /shortcut_responder/localization_accuracy_minus_chance.

Expected: between 0.40 and 0.50 (about 0.45).

Falsifier: a shortcut-responder accuracy minus chance outside 0.40 to 0.50, an
absolute mean change under the wording change of 0.03 or more, or a
random-picker accuracy minus chance with absolute value 0.03 or more.

This is a simulation with no model queried.

## 2026-11-19T02:48:59Z logit-gap-compression proposal:927bd282b0e0b8f7edb8e5e11deb29b420a28ed98a5ff3e679c090d12d8fcdeb

Hypothesis: with seed 7, 400 trials per arm, 200 replicates, baseline logit
gaps drawn from Normal(-2, 1) and a compression factor of 0.5 applied alike
under injection and under a matched random direction, a compression responder
has a clean-to-injected yes-rate rise of at least 0.10 but a mean
injected-minus-random yes-rate difference within 0.03 of zero, while a true
detector (injection-only shift) keeps an injected-minus-random difference of at
least 0.25. A clean-versus-injected detection score alone therefore cannot
separate compression from detection; only the matched random direction does.

Metric: /compression_responder/injected_minus_random_yes_rate.

Expected: near zero, within 0.03 of zero (range -0.03 to 0.03).

Falsifier: a compression-responder injected-minus-random yes-rate difference
with absolute value 0.03 or more, a clean-to-injected rise below 0.10, or a
true-detector injected-minus-random difference below 0.25.

This is a simulation with no model queried. The true detector adds 2.0 to each
item's logit gap under injection only.

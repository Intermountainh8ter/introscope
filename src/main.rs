use introscope::{
    dose_gated_d_prime, harmonic, iso_utc, json_escape, parse_trials, report_json, score,
    simulate_concept_mismatch, simulate_confidence_channel, simulate_control_question,
    simulate_detect_identify, simulate_detect_power, simulate_identify_prior,
    simulate_impact_matched, simulate_input_only_baseline, simulate_item_cluster,
    simulate_layer_selection, simulate_matched_fpr, simulate_position_artifact,
    simulate_selfcross_power, simulate_selfpred_power, simulate_sham_power, simulate_share_cell,
    simulate_stack_drift, simulate_strength_identity_gap, simulate_three_arm, simulate_wording_arm,
    simulate_wording_cell, simulate_wording_design, synth_trials, BaselinePredictor,
    ConfidenceKind, GapArm, GapResponder, ImpactControl, ImpactResponder, ItemResponder, Naming,
    PriorResponder, Responder, SelfCrossSpec, WordingProbs, WordingResponder, CONCEPT_DISRUPTION,
    WORDING_LEADS,
};
use std::process::ExitCode;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const USAGE: &str = "introscope: audit whether self-reports track internal states

USAGE:
  introscope baselines --out <results.json> [--seed 7] [--n 400] [--resamples 2000]
      Score the four degenerate responders (always_yes, always_no, coin,
      lead_follower). None of them has access to an internal state, so every
      measure must come out at its null value for them.

  introscope detect-power --out <results.json> [--seed 7] [--n 400] [--reps 200] [--resamples 2000]
      Simulate the detect measure for an injected-concept responder (true
      d' 1.5) and a matched-random-direction responder (true d' 0) and report
      the sensitivity gap and how often each interval excludes zero.

  introscope wording-sweep --out <results.json> [--seed 7] [--n 400] [--reps 200] [--resamples 2000]
      Simulate five yes-lead wordings (0.0 to 0.4 added to every arm) and report
      raw yes-rate spread and factual-control-adjusted spread per responder.

  introscope detect-vs-identify --out <results.json> [--seed 7] [--n 400] [--reps 200] [--resamples 2000]
      Simulate a content-agnostic and a content-aware responder (plus degenerate
      ones) and report detection against a prompt-only arm and concept
      identification accuracy among detected injected trials.

  introscope matched-fpr --out <results.json> [--seed 7] [--n 400] [--reps 200]
      Compare injected-concept, matched-random and yes-shifted responders at a
      fixed false-alarm rate of 0.10.

  introscope template-gain --out <results.json> [--seed 7] [--n 400] [--reps 200]
      Simulate three prompt templates that change true d' (1.3, 1.0, 0.7) and
      compare the spread of mean d' across templates with the spread across
      three same-template seed batches; a random-direction null runs per
      template.

  introscope stack-drift --out <results.json> [--seed 7] [--n 400] [--reps 200] [--resamples 2000]
      Simulate a detector and a null random-direction responder under five
      serving stacks that shift the Yes criterion; report how far mean d' and
      the hit-minus-false-alarm gap move across stacks.

  introscope identify-frequency-prior --out <results.json> [--seed 7] [--n 400] [--reps 200] [--resamples 2000]
  introscope input-only-share-sweep --out <results.json> [--seed 7] [--n 400] [--reps 200] [--resamples 2000]
      Simulate a self-predictor that uses a privileged signal on a share of
      items against an input-only classifier and a cross-predictor; report the
      smallest share whose advantage interval excludes zero in 0.80 of replicates.

  introscope strength-identity-gap --out <results.json> [--seed 7] [--n 400] [--reps 200]
  introscope answer-position-artifact --out <results.json> [--seed 7] [--n 400] [--reps 200] [--resamples 2000]
      Simulate an answer-A-biased responder with no state signal on a neutral
      and a threat condition under fixed and counterbalanced answer order.

  introscope resample-cluster --out <results.json> [--seed 7] [--items 100] [--k 4] [--reps 200] [--resamples 2000]
      Simulate a null responder with a fixed per-item yes bias and a true
      detector, each item sampled k times; compare a trial-level interval with
      an item-clustered interval.

  introscope layer-selection --out <results.json> [--seed 7] [--layers 24] [--n 100] [--reps 200]
      Simulate a null responder and a one-layer introspector scored at the best
      of all layers and at one pre-specified layer.

  introscope concept-mismatch --out <results.json> [--seed 7] [--n 400] [--reps 200] [--resamples 2000]
  introscope control-question --out <results.json> [--seed 7] [--n 400] [--reps 200] [--resamples 2000]
      Simulate steering that lifts the Yes rate on both the introspection
      question and control questions; compare raw and control-corrected
      detection pass rates for a pure Yes-bias and a true-detection responder.

  introscope dose-gate --out <results.json> [--seed 7] [--n 400] [--reps 200] [--resamples 2000]
      Simulate a dose-gated responder, a matched-random-direction responder and
      an always-yes responder at doses 0.5, 1.0 and 2.0 (threshold 1.0); report
      concept-minus-random mean d' per dose.

  introscope wording-confounded-design --out <results.json> [--seed 7] [--n 400] [--reps 200] [--resamples 2000]
      Simulate a wording-follower and a true-state responder in a design where
      injection is paired with leading wording and in a wording-balanced design;
      report mean d' and how often each interval excludes zero.

  introscope perturbation-confound --out <results.json> [--seed 7] [--n 400] [--reps 200] [--resamples 2000]
      Simulate a perturbation-reactive responder and a state reader over the
      concept, matched-random and prompt-only arms; report how often each
      control's gap interval excludes zero.

  introscope concept-random-prompt --out <results.json> [--seed 7] [--n 400] [--reps 200] [--resamples 2000]
      Simulate three arms (injected concept, norm-matched random direction,
      prompt-only leading wording) for a state reader, a wording-follower and
      the degenerate responders; report concept-minus-control yes-rate gaps.

  introscope detect-n-sweep --out <results.json> [--seed 7] [--reps 200] [--resamples 2000]
      Simulate the detect measure at 25, 50, 100, 200 and 400 trials per arm
      for a weak detector (true d' 0.5) and a matched random direction
      (true d' 0) and report the smallest n with power of at least 0.80.

  introscope selfpred-power --out <results.json> [--seed 7] [--n 400] [--reps 200] [--resamples 2000]
      Simulate the selfpred measure for a self-predictor with privileged access
      (self accuracy 0.75, cross accuracy 0.60) and a degenerate responder
      with equal accuracy on both (0.60) and report the advantage gap and how
      often each interval excludes zero.

  introscope selfcross-power --out <results.json> [--seed 7] [--n 400] [--reps 200] [--disagree 0.3] [--resamples 2000]
      Simulate selfpred advantage on all items and on the disagreement subset
      for a privileged-self, a general-capability and a consensus responder.

  introscope yes-bias-audit --out <results.json> [--seed 7] [--n 400] [--reps 200] [--resamples 2000]
      Score detection hit rate minus factual-no yes rate for a graded
      responder and for always-yes, always-no and prompt-sensitive
      responders; count degenerate responders that clear the audit.
  introscope impact-matched --out <results.json> [--seed 7] [--n 400] [--reps 200] [--resamples 2000]
      Compare a concept detector, an impact-reactive responder and degenerate
      responders against norm-matched and impact-matched random controls.
  introscope input-only-baseline --out <results.json> [--seed 7] [--n 400] [--reps 200] [--resamples 2000]
      Score a state-based self-predictor against an input-only baseline and a
      relabeling control, with degenerate responders.
  introscope wording-arm --out <results.json> [--seed 7] [--n 400] [--reps 200] [--resamples 2000]
      Score the answer shift between paired leading wordings (neutral, could,
      did, presupposing) for always-yes, always-no, state-tracking,
      wording-following and partly wording-following responders.
  introscope confidence-channel --out <results.json> [--seed 7] [--n 400] [--reps 200] [--resamples 2000]
      Score AUROC of attached confidence for injected versus sham trials for
      always-yes responders with graded, constant and noise confidence,
      alongside the yes/no sham-adjusted gap.
  introscope selfcross-share-sweep --out <results.json> [--seed 7] [--n 400] [--reps 200] [--resamples 2000]
      Simulate the disagreement-subset selfpred test at disagreement shares
      0.05 to 0.50 for a privileged-self and a general-capability responder
      and report the smallest share with power of at least 0.80.
  introscope sham-power --out <results.json> [--seed 7] [--n 400] [--reps 200] [--resamples 2000]
      Simulate a sham-controlled detect score (hit rate minus sham
      false-alarm rate) for an injected-concept responder (true d' 1.5), a
      sham-reactive responder (0.5 yes on both arms) and an always-yes
      responder, and report how often each interval excludes zero.

  introscope score --trials <trials.tsv> --out <results.json> [--seed 7] [--resamples 2000]
                   [--model <name>] [--source <url-or-label> --license <licence>]
      Score a trial table collected from a model. Columns, tab-separated:
      task item variant truth report (see docs/PREREGISTRATION.md).
";

struct Args(Vec<String>);

impl Args {
    fn value(&self, flag: &str) -> Option<&str> {
        let i = self.0.iter().position(|a| a == flag)?;
        self.0.get(i + 1).map(String::as_str)
    }
    fn float(&self, flag: &str, default: f64) -> Result<f64, String> {
        match self.value(flag) {
            None => Ok(default),
            Some(v) => v
                .parse()
                .map_err(|_| format!("{flag} needs a number, got `{v}`")),
        }
    }
    fn number(&self, flag: &str, default: u64) -> Result<u64, String> {
        match self.value(flag) {
            None => Ok(default),
            Some(v) => v
                .parse()
                .map_err(|_| format!("{flag} needs a whole number, got `{v}`")),
        }
    }
}

fn provenance(started: u64, seed: u64, wall_s: f64, sources: &str) -> String {
    let command = std::env::args().collect::<Vec<_>>().join(" ");
    format!(
        "  \"provenance\": {{\"started_at\": \"{}\", \"seeds\": [{}], \"command\": \"{}\", \"hardware\": \"{} {}\", \"wall_s\": {:.3}, \"data_sources\": {}}}",
        iso_utc(started),
        seed,
        json_escape(&command),
        std::env::consts::OS,
        std::env::consts::ARCH,
        wall_s,
        sources
    )
}

fn run() -> Result<(), String> {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = argv.first().cloned() else {
        return Err(USAGE.into());
    };
    let args = Args(argv);
    let out = args
        .value("--out")
        .ok_or_else(|| format!("--out is required\n\n{USAGE}"))?
        .to_string();
    let seed = args.number("--seed", 7)?;
    let resamples = args.number("--resamples", 2000)? as usize;
    let started = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();
    let clock = Instant::now();

    let body = match cmd.as_str() {
        "baselines" => {
            let n = args.number("--n", 400)? as usize;
            let mut blocks = Vec::new();
            for (k, r) in Responder::ALL.iter().enumerate() {
                let trials = synth_trials(*r, n, seed.wrapping_add(1000 * k as u64));
                let rep = score(&trials, resamples, seed);
                blocks.push(format!("    \"{}\": {}", r.name(), report_json(&rep, 4)));
            }
            format!(
                "  \"kind\": \"baselines\",\n  \"n_per_task\": {},\n  \"resamples\": {},\n  \"responders\": {{\n{}\n  }},\n{}",
                n,
                resamples,
                blocks.join(",\n"),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: generated by this command from the seed\", \"license\": \"MIT\"}]"
                )
            )
        }
        "detect-power" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let inj = simulate_detect_power(1.5, n, reps, resamples, seed);
            let rnd = simulate_detect_power(0.0, n, reps, resamples, seed.wrapping_add(5000));
            let arm = |a: &introscope::PowerArm| {
                format!(
                    "{{\"true_d_prime\": {:.1}, \"mean_d_prime\": {:.4}, \"excludes_zero_rate\": {:.4}}}",
                    a.true_d_prime, a.mean_d_prime, a.excludes_zero_rate
                )
            };
            format!(
                "  \"kind\": \"detect-power\",\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"detection\": {{\n    \"injected\": {},\n    \"random_direction\": {},\n    \"sensitivity_gap\": {:.4}\n  }},\n{}",
                n,
                reps,
                resamples,
                arm(&inj),
                arm(&rnd),
                inj.mean_d_prime - rnd.mean_d_prime,
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "detect-n-sweep" => {
            let reps = args.number("--reps", 200)? as usize;
            let ns = [25usize, 50, 100, 200, 400];
            let mut rows = Vec::new();
            let mut min_n: Option<usize> = None;
            for &n in &ns {
                let det = simulate_detect_power(0.5, n, reps, resamples, seed);
                let rnd = simulate_detect_power(0.0, n, reps, resamples, seed.wrapping_add(5000));
                if min_n.is_none() && det.excludes_zero_rate >= 0.80 {
                    min_n = Some(n);
                }
                rows.push(format!(
                    "    {{\"n_per_arm\": {}, \"detector_mean_d_prime\": {:.4}, \"detector_excludes_zero_rate\": {:.4}, \"random_mean_d_prime\": {:.4}, \"random_excludes_zero_rate\": {:.4}}}",
                    n, det.mean_d_prime, det.excludes_zero_rate, rnd.mean_d_prime, rnd.excludes_zero_rate
                ));
            }
            format!(
                "  \"kind\": \"detect-n-sweep\",\n  \"reps\": {},\n  \"resamples\": {},\n  \"sweep\": {{\n    \"detector_true_d_prime\": 0.5,\n    \"min_n_for_power_0_80\": {},\n    \"cells\": [\n{}\n    ]\n  }},\n{}",
                reps,
                resamples,
                min_n.map_or("null".to_string(), |n| n.to_string()),
                rows.join(",\n"),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "wording-sweep" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let specs: [(&str, f64, f64, Option<bool>); 5] = [
                ("graded", 0.55, 0.05, None),
                ("prompt_follower", 0.30, 0.30, None),
                ("always_yes", 0.0, 0.0, Some(true)),
                ("always_no", 0.0, 0.0, Some(false)),
                ("coin", 0.5, 0.5, None),
            ];
            let mut blocks = Vec::new();
            let mut spreads = Vec::new();
            for (k, (name, bi, bc, fx)) in specs.iter().enumerate() {
                let cells: Vec<_> = WORDING_LEADS
                    .iter()
                    .enumerate()
                    .map(|(j, &l)| {
                        simulate_wording_cell(
                            *bi,
                            *bc,
                            *fx,
                            l,
                            n,
                            reps,
                            resamples,
                            seed.wrapping_add(5000 * (k as u64 * 10 + j as u64)),
                        )
                    })
                    .collect();
                let spread = |f: &dyn Fn(&introscope::WordingCell) -> f64| {
                    let v: Vec<f64> = cells.iter().map(f).collect();
                    v.iter().cloned().fold(f64::MIN, f64::max)
                        - v.iter().cloned().fold(f64::MAX, f64::min)
                };
                let raw_s = spread(&|c| c.raw_yes_rate);
                let adj_s = spread(&|c| c.adjusted_gap);
                let rows: Vec<String> = cells
                    .iter()
                    .map(|c| {
                        format!(
                            "        {{\"lead\": {:.1}, \"raw_yes_rate\": {:.4}, \"adjusted_gap\": {:.4}, \"excludes_zero_rate\": {:.4}}}",
                            c.lead, c.raw_yes_rate, c.adjusted_gap, c.excludes_zero_rate
                        )
                    })
                    .collect();
                let max_excl = cells
                    .iter()
                    .map(|c| c.excludes_zero_rate)
                    .fold(0.0, f64::max);
                blocks.push(format!(
                    "    \"{}\": {{\"raw_spread\": {:.4}, \"adjusted_spread\": {:.4}, \"max_excludes_zero_rate\": {:.4}, \"cells\": [\n{}\n    ]}}",
                    name, raw_s, adj_s, max_excl, rows.join(",\n")
                ));
                spreads.push((raw_s, adj_s));
            }
            format!(
                "  \"kind\": \"wording-sweep\",\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"wording\": {{\n    \"graded_raw_spread_across_wordings\": {:.4},\n    \"graded_adjusted_spread_across_wordings\": {:.4},\n{}\n  }},\n{}",
                n,
                reps,
                resamples,
                spreads[0].0,
                spreads[0].1,
                blocks.join(",\n"),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "concept-random-prompt" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let specs: [(&str, [f64; 3]); 5] = [
                ("state_reader", [0.70, 0.10, 0.10]),
                ("wording_follower", [0.60, 0.60, 0.60]),
                ("always_yes", [1.0; 3]),
                ("always_no", [0.0; 3]),
                ("coin", [0.5; 3]),
            ];
            let cells: Vec<_> = specs
                .iter()
                .enumerate()
                .map(|(k, (_, p))| {
                    simulate_three_arm(*p, n, reps, resamples, seed.wrapping_add(5000 * k as u64))
                })
                .collect();
            let blocks: Vec<String> = specs
                .iter()
                .zip(cells.iter())
                .map(|((name, p), c)| {
                    format!(
                        "    \"{}\": {{\"p_concept\": {:.2}, \"p_random\": {:.2}, \"p_prompt\": {:.2}, \"concept_rate\": {:.4}, \"random_rate\": {:.4}, \"prompt_rate\": {:.4}, \"concept_minus_random\": {:.4}, \"concept_minus_prompt\": {:.4}, \"random_excludes_zero_rate\": {:.4}, \"prompt_excludes_zero_rate\": {:.4}}}",
                        name, p[0], p[1], p[2], c.concept_rate, c.random_rate, c.prompt_rate,
                        c.concept_minus_random, c.concept_minus_prompt,
                        c.random_excludes_zero_rate, c.prompt_excludes_zero_rate
                    )
                })
                .collect();
            format!(
                "  \"kind\": \"concept-random-prompt\",\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"detection\": {{\n    \"concept_minus_random_rate\": {:.4},\n    \"state_reader_random_excludes_zero_rate\": {:.4},\n{}\n  }},\n{}",
                n,
                reps,
                resamples,
                cells[0].concept_minus_random,
                cells[0].random_excludes_zero_rate,
                blocks.join(",\n"),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "detect-vs-identify" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let k = 10usize;
            let specs = [
                ("content_agnostic", (0.60, 0.10, Naming::Fixed)),
                ("content_aware", (0.70, 0.10, Naming::Aware(0.80))),
                ("always_yes", (1.0, 1.0, Naming::Fixed)),
                ("always_no", (0.0, 0.0, Naming::Random)),
                ("fair_coin", (0.5, 0.5, Naming::Random)),
            ];
            let plain: Vec<_> = specs.iter().map(|s| s.1).collect();
            let res = simulate_detect_identify(&plain, k, n, reps, resamples, seed);
            let blocks: Vec<String> = specs
                .iter()
                .zip(res.iter())
                .map(|((name, _), c)| {
                    format!(
                        "    \"{}\": {{\"concept_rate\": {:.4}, \"prompt_rate\": {:.4}, \"prompt_excludes_zero_rate\": {:.4}, \"identification_accuracy\": {:.4}, \"margin\": {:.4}, \"margin_up_rate\": {:.4}}}",
                        name, c.concept_rate, c.prompt_rate, c.prompt_excludes_zero_rate,
                        c.identification_accuracy, c.margin, c.margin_up_rate
                    )
                })
                .collect();
            format!(
                "  \"kind\": \"detect-vs-identify\",\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"concepts\": {},\n  \"detect_vs_identify\": {{\n    \"identification_margin_at_separation\": {:.4},\n{}\n  }},\n{}",
                n,
                reps,
                resamples,
                k,
                res[1].margin,
                blocks.join(",\n"),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "identify-frequency-prior" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let specs = [
                ("guesser", PriorResponder::Guesser),
                ("content_aware", PriorResponder::Aware(0.80)),
                ("always_yes", PriorResponder::AlwaysYes),
                ("always_no", PriorResponder::AlwaysNo),
                ("coin", PriorResponder::Coin),
            ];
            let rs: Vec<PriorResponder> = specs.iter().map(|s| s.1).collect();
            let sizes = [10usize, 100, 1000];
            let mut blocks = Vec::new();
            let mut aware_adv = Vec::new();
            for &v in &sizes {
                let cells = simulate_identify_prior(&rs, v, n, reps, resamples, seed);
                aware_adv.push(cells[1].advantage);
                let inner: Vec<String> = specs
                    .iter()
                    .zip(&cells)
                    .map(|(s, c)| {
                        format!(
                            "      \"{}\": {{\"accuracy\": {:.4}, \"advantage\": {:.4}, \"advantage_up_rate\": {:.4}}}",
                            s.0, c.accuracy, c.advantage, c.advantage_up_rate
                        )
                    })
                    .collect();
                blocks.push(format!(
                    "    \"v{}\": {{\n      \"inverse_harmonic\": {:.4},\n{}\n    }}",
                    v,
                    1.0 / harmonic(v),
                    inner.join(",\n")
                ));
            }
            format!(
                "  \"kind\": \"identify-frequency-prior\",\n  \"n_injected\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"identify_vs_frequency_prior\": {{\n    \"advantage_gain_large_minus_small\": {:.4},\n{}\n  }},\n{}",
                n,
                reps,
                resamples,
                aware_adv[2] - aware_adv[0],
                blocks.join(",\n"),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "input-only-share-sweep" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let shares = [0.0, 0.05, 0.10, 0.15, 0.20, 0.30];
            let cells: Vec<_> = shares
                .iter()
                .enumerate()
                .map(|(k, &s)| {
                    simulate_share_cell(
                        s,
                        0.75,
                        0.60,
                        n,
                        reps,
                        resamples,
                        seed.wrapping_add(7000 * k as u64),
                    )
                })
                .collect();
            let min_share = cells
                .iter()
                .find(|c| c.self_up_rate >= 0.80)
                .map_or(-1.0, |c| c.share);
            let blocks: Vec<String> = cells
                .iter()
                .map(|c| {
                    format!(
                        "      \"s{:.2}\": {{\"share\": {:.2}, \"self_advantage\": {:.4}, \"self_up_rate\": {:.4}, \"cross_advantage\": {:.4}, \"cross_up_rate\": {:.4}, \"always_yes_up_rate\": {:.4}, \"always_no_up_rate\": {:.4}, \"coin_up_rate\": {:.4}}}",
                        c.share, c.share, c.self_advantage, c.self_up_rate, c.cross_advantage,
                        c.cross_up_rate, c.always_yes_up_rate, c.always_no_up_rate, c.coin_up_rate
                    )
                })
                .collect();
            format!(
                "  \"kind\": \"input-only-share-sweep\",\n  \"n_items\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"input_accuracy\": 0.75,\n  \"cross_accuracy\": 0.60,\n  \"sweep\": {{\n    \"min_share_for_power_0_80\": {:.2},\n    \"cells\": {{\n{}\n    }}\n  }},\n{}",
                n,
                reps,
                resamples,
                min_share,
                blocks.join(",\n"),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "strength-identity-gap" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let k = 10usize;
            let specs = [
                ("strength_only", GapResponder::StrengthOnly),
                ("content_aware", GapResponder::Aware(0.80)),
                ("always_yes", GapResponder::Constant),
                ("always_no", GapResponder::Constant),
                ("fair_coin", GapResponder::Coin),
            ];
            let res: Vec<GapArm> = specs
                .iter()
                .enumerate()
                .map(|(j, (_, r))| {
                    simulate_strength_identity_gap(
                        *r,
                        k,
                        n,
                        reps,
                        seed.wrapping_add(7000 * j as u64),
                    )
                })
                .collect();
            let blocks: Vec<String> = specs
                .iter()
                .zip(res.iter())
                .map(|((name, _), c)| {
                    format!(
                        "    \"{}\": {{\"mean_auroc\": {:.4}, \"mean_identification\": {:.4}, \"mean_gap\": {:.4}, \"mean_abs_gap\": {:.4}}}",
                        name, c.mean_auroc, c.mean_identification, c.mean_gap, c.mean_abs_gap
                    )
                })
                .collect();
            format!(
                "  \"kind\": \"strength-identity-gap\",\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"concepts\": {},\n  \"metrics\": {{\n    \"strength_minus_identity_gap\": {:.4},\n    \"content_aware_gap\": {:.4},\n    \"max_degenerate_abs_gap\": {:.4}\n  }},\n  \"responders\": {{\n{}\n  }},\n{}",
                n,
                reps,
                k,
                res[0].mean_gap,
                res[1].mean_gap,
                res[2..].iter().map(|c| c.mean_abs_gap).fold(0.0, f64::max),
                blocks.join(",\n"),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "answer-position-artifact" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let specs = [("fixed_order", false), ("counterbalanced", true)];
            let resp = [("biased_a", 0.65), ("coin", 0.5)];
            let mut arms = Vec::new();
            for (k, (name, cb)) in specs.iter().enumerate() {
                let mut inner = Vec::new();
                for (j, (rn, pa)) in resp.iter().enumerate() {
                    let c = simulate_position_artifact(
                        *pa,
                        *cb,
                        n,
                        reps,
                        resamples,
                        seed.wrapping_add(9000 * (2 * k + j) as u64),
                    );
                    inner.push((*rn, c));
                }
                let b = &inner[0].1;
                let coin = &inner[1].1;
                arms.push(format!(
                    "    \"{}\": {{\n      \"spurious_effect\": {:.4},\n      \"excludes_zero_rate\": {:.4},\n      \"coin_effect\": {:.4},\n      \"coin_excludes_zero_rate\": {:.4}\n    }}",
                    name, b.spurious_effect, b.excludes_zero_rate, coin.spurious_effect, coin.excludes_zero_rate
                ));
            }
            format!(
                "  \"kind\": \"answer-position-artifact\",\n  \"n_items\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"p_answer_a\": 0.65,\n  \"arms\": {{\n{}\n  }},\n{}",
                n,
                reps,
                resamples,
                arms.join(",\n"),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "resample-cluster" => {
            let items = args.number("--items", 100)? as usize;
            let k = args.number("--k", 4)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let nul =
                simulate_item_cluster(ItemResponder::NullItemBias, items, k, reps, resamples, seed);
            let det = simulate_item_cluster(
                ItemResponder::Detector,
                items,
                k,
                reps,
                resamples,
                seed.wrapping_add(50000),
            );
            let block = |c: &introscope::ClusterArm| {
                format!(
                    "{{\"mean_d_prime\": {:.4}, \"trial_level_excludes_zero_rate\": {:.4}, \"clustered_excludes_zero_rate\": {:.4}}}",
                    c.mean_d_prime, c.trial_level_excludes_zero_rate, c.clustered_excludes_zero_rate
                )
            };
            format!(
                "  \"kind\": \"resample-cluster\",\n  \"items_per_arm\": {},\n  \"k\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"null_item_bias\": {},\n  \"detector\": {},\n{}",
                items,
                k,
                reps,
                resamples,
                block(&nul),
                block(&det),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "matched-fpr" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let fpr = 0.10;
            let inj = simulate_matched_fpr(1.5, false, fpr, n, reps, seed);
            let rnd = simulate_matched_fpr(0.0, false, fpr, n, reps, seed.wrapping_add(1000));
            let yes = simulate_matched_fpr(0.0, true, fpr, n, reps, seed.wrapping_add(2000));
            let block = |a: &introscope::MatchedFprArm| {
                format!(
                    "{{\"mean_hit_rate\": {:.4}, \"mean_false_alarm_rate\": {:.4}, \"mean_advantage\": {:.4}, \"excludes_zero_rate\": {:.4}}}",
                    a.mean_hit_rate, a.mean_false_alarm_rate, a.mean_advantage, a.excludes_zero_rate
                )
            };
            format!(
                "  \"kind\": \"matched-fpr\",\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"false_alarm_rate\": {:.2},\n  \"sensitivity_advantage_at_matched_fpr\": {:.4},\n  \"injected\": {},\n  \"matched_random\": {},\n  \"yes_shifted\": {},\n{}",
                n,
                reps,
                fpr,
                inj.mean_advantage,
                block(&inj),
                block(&rnd),
                block(&yes),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "template-gain" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let dprimes = [1.3, 1.0, 0.7];
            let cells: Vec<(introscope::StackCell, introscope::StackCell)> = dprimes
                .iter()
                .enumerate()
                .map(|(k, d)| {
                    (
                        simulate_stack_drift(*d, 0.0, n, reps, seed.wrapping_add(1000 * k as u64)),
                        simulate_stack_drift(
                            0.0,
                            0.0,
                            n,
                            reps,
                            seed.wrapping_add(50000 + 1000 * k as u64),
                        ),
                    )
                })
                .collect();
            let batches: Vec<f64> = (0..3u64)
                .map(|b| {
                    simulate_stack_drift(1.0, 0.0, n, reps, seed.wrapping_add(90000 + 1000 * b))
                        .dprime_mean
                })
                .collect();
            let range = |v: Vec<f64>| {
                v.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
                    - v.iter().cloned().fold(f64::INFINITY, f64::min)
            };
            let det_range = range(cells.iter().map(|c| c.0.dprime_mean).collect());
            let seed_range = range(batches.clone());
            let ratio = det_range / seed_range.max(1e-9);
            let null_max = cells
                .iter()
                .map(|c| c.1.dprime_mean.abs())
                .fold(0.0, f64::max);
            let rows: Vec<String> = dprimes
                .iter()
                .zip(cells.iter())
                .map(|(d, (a, z))| {
                    format!(
                        "    {{\"true_dprime\": {:.1}, \"detector_dprime_mean\": {:.4}, \"null_dprime_mean\": {:.4}}}",
                        d, a.dprime_mean, z.dprime_mean
                    )
                })
                .collect();
            let bl: Vec<String> = batches.iter().map(|b| format!("{:.4}", b)).collect();
            format!(
                "  \"kind\": \"template-gain\",\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"template_audit\": {{\n    \"detection_range_across_templates\": {:.4},\n    \"seed_batch_range\": {:.4},\n    \"detection_spread_over_seed_spread\": {:.4},\n    \"null_max_abs_dprime_mean\": {:.4},\n    \"seed_batch_dprime_means\": [{}]\n  }},\n  \"templates\": [\n{}\n  ],\n{}",
                n,
                reps,
                det_range,
                seed_range,
                ratio,
                null_max,
                bl.join(", "),
                rows.join(",\n"),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "stack-drift" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let shifts = [-0.6, -0.3, 0.0, 0.3, 0.6];
            let run = |d: f64, off: u64| -> Vec<introscope::StackCell> {
                shifts
                    .iter()
                    .enumerate()
                    .map(|(k, s)| {
                        simulate_stack_drift(
                            d,
                            *s,
                            n,
                            reps,
                            seed.wrapping_add(off + 1000 * k as u64),
                        )
                    })
                    .collect()
            };
            let det = run(1.0, 0);
            let nul = run(0.0, 50000);
            let range = |v: Vec<f64>| {
                v.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
                    - v.iter().cloned().fold(f64::INFINITY, f64::min)
            };
            let gap_range = range(det.iter().map(|c| c.gap_mean).collect());
            let d_range = range(det.iter().map(|c| c.dprime_mean).collect());
            let null_max = nul.iter().map(|c| c.dprime_mean.abs()).fold(0.0, f64::max);
            let stacks: Vec<String> = shifts
                .iter()
                .zip(det.iter().zip(nul.iter()))
                .map(|(s, (d, z))| {
                    format!(
                        "    {{\"criterion_shift\": {:.1}, \"detector_dprime_mean\": {:.4}, \"detector_gap_mean\": {:.4}, \"null_dprime_mean\": {:.4}}}",
                        s, d.dprime_mean, d.gap_mean, z.dprime_mean
                    )
                })
                .collect();
            format!(
                "  \"kind\": \"stack-drift\",\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"detector\": {{\"true_dprime\": 1.0, \"gap_range_across_stacks\": {:.4}, \"dprime_range_across_stacks\": {:.4}}},\n  \"null\": {{\"max_abs_dprime_mean\": {:.4}}},\n  \"stacks\": [\n{}\n  ],\n{}",
                n,
                reps,
                resamples,
                gap_range,
                d_range,
                null_max,
                stacks.join(",\n"),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "layer-selection" => {
            let layers = args.number("--layers", 24)? as usize;
            let n = args.number("--n", 100)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let pre = layers / 2;
            let null = vec![0.0; layers];
            let mut intro = vec![0.0; layers];
            intro[pre] = 1.0;
            let a_null = simulate_layer_selection(&null, pre, n, reps, seed);
            let a_intro = simulate_layer_selection(&intro, pre, n, reps, seed.wrapping_add(10000));
            let resp = |name: &str, a: &introscope::LayerArm| {
                format!(
                    "    \"{}\": {{\"best_layer_dprime_mean\": {:.4}, \"pre_specified_dprime_mean\": {:.4}}}",
                    name, a.best_layer_mean, a.pre_specified_mean
                )
            };
            format!(
                "  \"kind\": \"layer-selection\",\n  \"layers\": {},\n  \"pre_specified_layer\": {},\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"responders\": {{\n{},\n{}\n  }},\n{}",
                layers,
                pre,
                n,
                reps,
                resp("null", &a_null),
                resp("true_introspector", &a_intro),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "concept-mismatch" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let intro = simulate_concept_mismatch(0.70, 0.20, 0.20, n, reps, seed);
            let shifter =
                simulate_concept_mismatch(0.60, 0.60, 0.20, n, reps, seed.wrapping_add(10000));
            let agnostic =
                simulate_concept_mismatch(0.60, 0.60, 0.20, n, reps, seed.wrapping_add(20000));
            let resp = |name: &str, a: &introscope::MismatchArm| {
                format!(
                    "    \"{}\": {{\"matched_detection_advantage\": {:.4}, \"mismatch_detection_advantage\": {:.4}}}",
                    name, a.matched_advantage, a.mismatch_advantage
                )
            };
            format!(
                "  \"kind\": \"concept-mismatch\",\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"arms\": {{\n    \"mismatch\": {{\"detection_advantage\": {:.4}}},\n    \"matched\": {{\"detection_advantage\": {:.4}}}\n  }},\n  \"responders\": {{\n{}\n  }},\n{}",
                n,
                reps,
                resamples,
                intro.mismatch_advantage,
                intro.matched_advantage,
                [
                    resp("true_introspector", &intro),
                    resp("global_yes_shifter", &shifter),
                    resp("content_agnostic_detector", &agnostic)
                ]
                .join(",\n"),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "control-question" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let (base, threshold) = (0.3_f64, 0.10_f64);
            let doses = [0.5_f64, 1.0, 2.0];
            let mut blocks = Vec::new();
            let mut headline = 0.0;
            for (k, dose) in doses.iter().enumerate() {
                let s = seed.wrapping_add(10000 * k as u64);
                let bias = 0.1 * dose;
                let pure = simulate_control_question(base, bias, 0.0, threshold, n, reps, s);
                let truth = simulate_control_question(
                    base,
                    bias,
                    bias,
                    threshold,
                    n,
                    reps,
                    s.wrapping_add(5000),
                );
                let diff = pure.raw_pass_rate - pure.corrected_pass_rate;
                if k == doses.len() - 1 {
                    headline = diff;
                }
                blocks.push(format!(
                    "    \"dose_{:.1}\": {{\"yes_bias\": {:.2}, \"pure_bias_raw_pass_rate\": {:.4}, \"pure_bias_corrected_pass_rate\": {:.4}, \"pure_bias_mean_raw_gap\": {:.4}, \"pure_bias_mean_corrected_gap\": {:.4}, \"true_detection_raw_pass_rate\": {:.4}, \"true_detection_corrected_pass_rate\": {:.4}, \"true_detection_mean_corrected_gap\": {:.4}, \"false_pass_rate_raw_minus_corrected\": {:.4}}}",
                    dose, bias, pure.raw_pass_rate, pure.corrected_pass_rate, pure.mean_raw_gap,
                    pure.mean_corrected_gap, truth.raw_pass_rate, truth.corrected_pass_rate,
                    truth.mean_corrected_gap, diff
                ));
            }
            format!(
                "  \"kind\": \"control-question\",\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"control_question\": {{\n    \"top_dose\": 2.0,\n    \"pass_threshold\": {:.2},\n    \"false_pass_rate_raw_minus_corrected\": {:.4},\n{}\n  }},\n{}",
                n,
                reps,
                resamples,
                threshold,
                headline,
                blocks.join(",\n"),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "dose-gate" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let doses = [0.5_f64, 1.0, 2.0];
            let always_yes = WordingProbs {
                injected_leading: 1.0,
                injected_neutral: 1.0,
                noise_leading: 1.0,
                noise_neutral: 1.0,
            };
            let mut blocks = Vec::new();
            let mut at_threshold = 0.0;
            for (k, dose) in doses.iter().enumerate() {
                let s = seed.wrapping_add(10000 * k as u64);
                let concept = simulate_detect_power(
                    dose_gated_d_prime(*dose, 1.0, 1.5),
                    n,
                    reps,
                    resamples,
                    s,
                );
                let random = simulate_detect_power(0.0, n, reps, resamples, s.wrapping_add(5000));
                let yes = simulate_wording_design(
                    always_yes,
                    false,
                    n,
                    reps,
                    resamples,
                    s.wrapping_add(7000),
                );
                let gap = concept.mean_d_prime - random.mean_d_prime;
                if *dose == 1.0 {
                    at_threshold = gap;
                }
                blocks.push(format!(
                    "    \"dose_{:.1}\": {{\"concept_true_d_prime\": {:.1}, \"concept_mean_d_prime\": {:.4}, \"concept_excludes_zero_rate\": {:.4}, \"random_mean_d_prime\": {:.4}, \"random_excludes_zero_rate\": {:.4}, \"always_yes_mean_d_prime\": {:.4}, \"concept_minus_random\": {:.4}}}",
                    dose,
                    concept.true_d_prime,
                    concept.mean_d_prime,
                    concept.excludes_zero_rate,
                    random.mean_d_prime,
                    random.excludes_zero_rate,
                    yes.mean_d_prime,
                    gap
                ));
            }
            format!(
                "  \"kind\": \"dose-gate\",\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"dose_gate\": {{\n    \"threshold_dose\": 1.0,\n    \"concept_minus_random_at_threshold\": {:.4},\n{}\n  }},\n{}",
                n,
                reps,
                resamples,
                at_threshold,
                blocks.join(",\n"),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "wording-confounded-design" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let follower = WordingProbs {
                injected_leading: 0.70,
                injected_neutral: 0.30,
                noise_leading: 0.70,
                noise_neutral: 0.30,
            };
            let state = WordingProbs {
                injected_leading: 0.7734,
                injected_neutral: 0.7734,
                noise_leading: 0.2266,
                noise_neutral: 0.2266,
            };
            let cells = [
                ("follower_confounded", follower, true),
                ("follower_balanced", follower, false),
                ("true_state_confounded", state, true),
                ("true_state_balanced", state, false),
            ];
            let res: Vec<_> = cells
                .iter()
                .enumerate()
                .map(|(k, (_, p, c))| {
                    simulate_wording_design(
                        *p,
                        *c,
                        n,
                        reps,
                        resamples,
                        seed.wrapping_add(5000 * k as u64),
                    )
                })
                .collect();
            let blocks: Vec<String> = cells
                .iter()
                .zip(res.iter())
                .map(|((name, _, _), r)| {
                    format!(
                        "    \"{}\": {{\"mean_d_prime\": {:.4}, \"excludes_zero_rate\": {:.4}}}",
                        name, r.mean_d_prime, r.excludes_zero_rate
                    )
                })
                .collect();
            format!(
                "  \"kind\": \"wording-confounded-design\",\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"wording_confounded_design\": {{\n    \"confounded_minus_balanced_dprime_for_follower\": {:.4},\n{}\n  }},\n{}",
                n,
                reps,
                resamples,
                res[0].mean_d_prime - res[1].mean_d_prime,
                blocks.join(",\n"),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "perturbation-confound" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let specs: [(&str, [f64; 3]); 2] = [
                ("perturbation_shifter", [0.45, 0.45, 0.10]),
                ("state_reader", [0.70, 0.10, 0.10]),
            ];
            let cells: Vec<_> = specs
                .iter()
                .enumerate()
                .map(|(k, (_, p))| {
                    simulate_three_arm(*p, n, reps, resamples, seed.wrapping_add(5000 * k as u64))
                })
                .collect();
            let blocks: Vec<String> = specs
                .iter()
                .zip(cells.iter())
                .map(|((name, p), c)| {
                    format!(
                        "    \"{}\": {{\"p_concept\": {:.2}, \"p_random\": {:.2}, \"p_prompt\": {:.2}, \"concept_minus_random\": {:.4}, \"concept_minus_prompt\": {:.4}, \"random_excludes_zero_rate\": {:.4}, \"prompt_excludes_zero_rate\": {:.4}}}",
                        name, p[0], p[1], p[2], c.concept_minus_random, c.concept_minus_prompt,
                        c.random_excludes_zero_rate, c.prompt_excludes_zero_rate
                    )
                })
                .collect();
            format!(
                "  \"kind\": \"perturbation-confound\",\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"confound\": {{\n    \"shifter_prompt_only_excludes_zero_rate\": {:.4},\n    \"shifter_random_excludes_zero_rate\": {:.4},\n{}\n  }},\n{}",
                n,
                reps,
                resamples,
                cells[0].prompt_excludes_zero_rate,
                cells[0].random_excludes_zero_rate,
                blocks.join(",\n"),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "selfpred-power" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let inf = simulate_selfpred_power(0.75, 0.60, n, reps, resamples, seed);
            let deg =
                simulate_selfpred_power(0.60, 0.60, n, reps, resamples, seed.wrapping_add(5000));
            let arm = |a: &introscope::SelfPredArm| {
                format!(
                    "{{\"self_accuracy\": {:.2}, \"cross_accuracy\": {:.2}, \"mean_advantage\": {:.4}, \"excludes_zero_rate\": {:.4}}}",
                    a.self_accuracy, a.cross_accuracy, a.mean_advantage, a.excludes_zero_rate
                )
            };
            format!(
                "  \"kind\": \"selfpred-power\",\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"selfpred_power\": {{\n    \"informed\": {},\n    \"degenerate\": {},\n    \"advantage_gap\": {:.4}\n  }},\n{}",
                n,
                reps,
                resamples,
                arm(&inf),
                arm(&deg),
                inf.mean_advantage - deg.mean_advantage,
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "selfcross-power" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let disagree = args.float("--disagree", 0.3)?;
            let spec = |a, b, c, d| SelfCrossSpec {
                self_disagree: a,
                cross_disagree: b,
                self_consensus: c,
                cross_consensus: d,
            };
            let own = simulate_selfcross_power(
                spec(0.8, 0.5, 0.9, 0.9),
                disagree,
                n,
                reps,
                resamples,
                seed,
            );
            let gen = simulate_selfcross_power(
                spec(0.7, 0.7, 0.7, 0.7),
                disagree,
                n,
                reps,
                resamples,
                seed.wrapping_add(5000),
            );
            let con = simulate_selfcross_power(
                spec(0.5, 0.5, 0.9, 0.9),
                disagree,
                n,
                reps,
                resamples,
                seed.wrapping_add(10000),
            );
            let arm = |a: &introscope::SelfCrossArm| {
                format!(
                    "{{\"self_disagree\": {:.2}, \"cross_disagree\": {:.2}, \"self_consensus\": {:.2}, \"cross_consensus\": {:.2}, \"full_advantage\": {:.4}, \"full_excludes_zero_rate\": {:.4}, \"disagreement_subset_advantage\": {:.4}, \"subset_excludes_zero_rate\": {:.4}}}",
                    a.spec.self_disagree, a.spec.cross_disagree, a.spec.self_consensus, a.spec.cross_consensus,
                    a.full_advantage, a.full_excludes_zero_rate, a.subset_advantage, a.subset_excludes_zero_rate
                )
            };
            format!(
                "  \"kind\": \"selfcross-power\",\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"disagree_share\": {:.2},\n  \"selfcross\": {{\n    \"disagreement_subset_advantage\": {:.4},\n    \"full_set_advantage\": {:.4},\n    \"privileged_self\": {},\n    \"general_capability\": {},\n    \"consensus\": {}\n  }},\n{}",
                n,
                reps,
                resamples,
                disagree,
                own.subset_advantage,
                own.full_advantage,
                arm(&own),
                arm(&gen),
                arm(&con),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "selfcross-share-sweep" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let mut rows = Vec::new();
            let mut min_share: Option<f64> = None;
            for k in 1..=10u64 {
                let share = k as f64 * 0.05;
                let spec = |a, b, c, d| SelfCrossSpec {
                    self_disagree: a,
                    cross_disagree: b,
                    self_consensus: c,
                    cross_consensus: d,
                };
                let own = simulate_selfcross_power(
                    spec(0.8, 0.5, 0.9, 0.9),
                    share,
                    n,
                    reps,
                    resamples,
                    seed.wrapping_add(k * 100),
                );
                let gen = simulate_selfcross_power(
                    spec(0.7, 0.7, 0.7, 0.7),
                    share,
                    n,
                    reps,
                    resamples,
                    seed.wrapping_add(5000 + k * 100),
                );
                if min_share.is_none() && own.subset_excludes_zero_rate >= 0.80 {
                    min_share = Some(share);
                }
                rows.push(format!(
                    "    {{\"disagree_share\": {:.2}, \"privileged_subset_advantage\": {:.4}, \"privileged_excludes_zero_rate\": {:.4}, \"general_subset_advantage\": {:.4}, \"general_excludes_zero_rate\": {:.4}}}",
                    share, own.subset_advantage, own.subset_excludes_zero_rate, gen.subset_advantage, gen.subset_excludes_zero_rate
                ));
            }
            format!(
                "  \"kind\": \"selfcross-share-sweep\",\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"sweep\": {{\n    \"min_share_for_power_0_80\": {},\n    \"cells\": [\n{}\n    ]\n  }},\n{}",
                n,
                reps,
                resamples,
                min_share.map_or("null".to_string(), |x| format!("{x:.2}")),
                rows.join(",\n"),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "sham-power" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let inj = simulate_sham_power(0.7734, 0.2266, n, reps, resamples, seed);
            let rea = simulate_sham_power(0.5, 0.5, n, reps, resamples, seed.wrapping_add(5000));
            let yes = simulate_sham_power(1.0, 1.0, n, reps, resamples, seed.wrapping_add(10000));
            let arm = |a: &introscope::ShamArm| {
                format!(
                    "{{\"hit_p\": {:.4}, \"sham_fa_p\": {:.4}, \"mean_gap\": {:.4}, \"excludes_zero_rate\": {:.4}}}",
                    a.hit_p, a.sham_fa_p, a.mean_gap, a.excludes_zero_rate
                )
            };
            format!(
                "  \"kind\": \"sham-power\",\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"sham\": {{\n    \"injected\": {},\n    \"sham_reactive\": {},\n    \"always_yes\": {},\n    \"sensitivity_minus_false_alarm\": {:.4}\n  }},\n{}",
                n,
                reps,
                resamples,
                arm(&inj),
                arm(&rea),
                arm(&yes),
                inj.mean_gap,
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "confidence-channel" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let g =
                simulate_confidence_channel(ConfidenceKind::Graded(1.0), n, reps, resamples, seed);
            let c = simulate_confidence_channel(
                ConfidenceKind::Constant,
                n,
                reps,
                resamples,
                seed.wrapping_add(5000),
            );
            let z = simulate_confidence_channel(
                ConfidenceKind::Noise,
                n,
                reps,
                resamples,
                seed.wrapping_add(10000),
            );
            let arm = |a: &introscope::ConfidenceArm| {
                format!(
                    "{{\"mean_auroc\": {:.4}, \"auroc_excludes_half_rate\": {:.4}, \"mean_yes_no_gap\": {:.4}, \"gap_excludes_zero_rate\": {:.4}}}",
                    a.mean_auroc, a.auroc_excludes_half_rate, a.mean_yes_no_gap, a.gap_excludes_zero_rate
                )
            };
            format!(
                "  \"kind\": \"confidence-channel\",\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"confidence_channel\": {{\n    \"yes_biased_graded\": {},\n    \"yes_biased_constant\": {},\n    \"confidence_noise\": {},\n    \"yes_biased_graded_auroc\": {:.4}\n  }},\n{}",
                n,
                reps,
                resamples,
                arm(&g),
                arm(&c),
                arm(&z),
                g.mean_auroc,
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "input-only-baseline" => {
            use BaselinePredictor as P;
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let specs = [
                ("input_driven", P::InputDriven, false),
                ("privileged_state", P::PrivilegedState, false),
                ("always_yes", P::AlwaysYes, false),
                ("coin", P::Coin, false),
                ("input_only_after_relabel", P::InputDriven, true),
            ];
            let mut arms = Vec::new();
            let mut adv = Vec::new();
            for (i, (name, p, relabel)) in specs.iter().enumerate() {
                let a = simulate_input_only_baseline(
                    *p,
                    0.75,
                    *relabel,
                    n,
                    reps,
                    resamples,
                    seed.wrapping_add(5000 * i as u64),
                );
                arms.push(format!(
                    "      \"{}\": {{\"predictor_accuracy\": {:.4}, \"input_only_accuracy\": {:.4}, \"mean_advantage\": {:.4}, \"includes_zero_rate\": {:.4}, \"positive_excludes_zero_rate\": {:.4}}}",
                    name, a.predictor_accuracy, a.baseline_accuracy, a.mean_advantage, a.includes_zero_rate, a.positive_excludes_zero_rate
                ));
                adv.push(format!("      \"{}\": {:.4}", name, a.mean_advantage));
            }
            let priv_adv = simulate_input_only_baseline(
                P::PrivilegedState,
                0.75,
                false,
                n,
                reps,
                resamples,
                seed.wrapping_add(5000),
            )
            .mean_advantage;
            format!(
                "  \"kind\": \"input-only-baseline\",\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"input_signal\": 0.75,\n  \"input_only_baseline\": {{\n    \"advantage_over_input_only\": {:.4},\n    \"arms\": {{\n{}\n    }},\n    \"advantage_by_arm\": {{\n{}\n    }}\n  }},\n{}",
                n,
                reps,
                resamples,
                priv_adv,
                arms.join(",\n"),
                adv.join(",\n"),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "impact-matched" => {
            use ImpactControl as C;
            use ImpactResponder as R;
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let specs = [
                ("concept_detector", R::ConceptDetector),
                ("impact_reactive", R::ImpactReactive),
                ("always_yes", R::AlwaysYes),
                ("coin", R::Coin),
            ];
            let mut arms = Vec::new();
            let mut gaps = Vec::new();
            for (i, (name, r)) in specs.iter().enumerate() {
                let nm = simulate_impact_matched(
                    *r,
                    C::NormMatched,
                    n,
                    reps,
                    resamples,
                    seed.wrapping_add(10000 * i as u64),
                );
                let im = simulate_impact_matched(
                    *r,
                    C::ImpactMatched,
                    n,
                    reps,
                    resamples,
                    seed.wrapping_add(10000 * i as u64 + 5000),
                );
                arms.push(format!(
                    "      \"{}\": {{\"norm_matched\": {{\"mean_gap\": {:.4}, \"excludes_zero_rate\": {:.4}}}, \"impact_matched\": {{\"mean_gap\": {:.4}, \"excludes_zero_rate\": {:.4}}}}}",
                    name, nm.mean_gap, nm.excludes_zero_rate, im.mean_gap, im.excludes_zero_rate
                ));
                gaps.push((name, im.mean_gap));
            }
            let g = |k: &str| {
                gaps.iter()
                    .find(|(n, _)| **n == k)
                    .map(|x| x.1)
                    .unwrap_or(0.0)
            };
            format!(
                "  \"kind\": \"impact-matched\",\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"concept_disruption_mean\": {:.1},\n  \"detection_gap\": {{\n    \"concept_minus_impact_matched\": {:.4},\n    \"reactive_minus_impact_matched\": {:.4},\n    \"arms\": {{\n{}\n    }}\n  }},\n{}",
                n,
                reps,
                resamples,
                CONCEPT_DISRUPTION,
                g("concept_detector"),
                g("impact_reactive"),
                arms.join(",\n"),
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "wording-arm" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            use WordingResponder as W;
            let specs = [
                ("always_yes", W::AlwaysYes),
                ("always_no", W::AlwaysNo),
                ("state_tracking", W::StateTracking),
                ("wording_follower", W::WordingFollower),
                ("partly_following", W::PartlyFollowing),
            ];
            let mut arms = Vec::new();
            let mut shifts = Vec::new();
            let mut clear = 0;
            for (i, (name, r)) in specs.iter().enumerate() {
                let a = simulate_wording_arm(
                    *r,
                    n,
                    reps,
                    resamples,
                    seed.wrapping_add(5000 * i as u64),
                );
                if a.excludes_zero_rate > 0.05
                    && !matches!(r, W::WordingFollower | W::PartlyFollowing)
                {
                    clear += 1;
                }
                arms.push(format!(
                    "      \"{}\": {{\"mean_shift_could\": {:.4}, \"mean_shift_did\": {:.4}, \"mean_shift_presupposing\": {:.4}, \"excludes_zero_rate\": {:.4}}}",
                    name, a.mean_shift[0], a.mean_shift[1], a.mean_shift[2], a.excludes_zero_rate
                ));
                shifts.push(format!("      \"{}\": {:.4}", name, a.mean_shift[2]));
            }
            format!(
                "  \"kind\": \"wording-arm\",\n  \"n_per_wording\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"wording\": {{\n    \"responders\": {{\n{}\n    }},\n    \"answer_shift_by_responder\": {{\n{}\n    }},\n    \"degenerate_or_state_clear_count\": {}\n  }},\n{}",
                n,
                reps,
                resamples,
                arms.join(",\n"),
                shifts.join(",\n"),
                clear,
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "yes-bias-audit" => {
            let n = args.number("--n", 400)? as usize;
            let reps = args.number("--reps", 200)? as usize;
            let graded = simulate_sham_power(0.7734, 0.05, n, reps, resamples, seed);
            let yes = simulate_sham_power(1.0, 1.0, n, reps, resamples, seed.wrapping_add(5000));
            let no = simulate_sham_power(0.0, 0.0, n, reps, resamples, seed.wrapping_add(10000));
            let sens = simulate_sham_power(0.5, 0.5, n, reps, resamples, seed.wrapping_add(15000));
            let arm = |a: &introscope::ShamArm| {
                format!(
                    "{{\"hit_rate\": {:.4}, \"factual_no_yes_p\": {:.4}, \"mean_adjusted\": {:.4}, \"excludes_zero_rate\": {:.4}}}",
                    a.hit_p, a.sham_fa_p, a.mean_gap, a.excludes_zero_rate
                )
            };
            let clear = [&yes, &no, &sens]
                .iter()
                .filter(|a| a.excludes_zero_rate > 0.10)
                .count();
            format!(
                "  \"kind\": \"yes-bias-audit\",\n  \"n_per_arm\": {},\n  \"reps\": {},\n  \"resamples\": {},\n  \"audit\": {{\n    \"graded\": {},\n    \"always_yes\": {},\n    \"always_no\": {},\n    \"prompt_sensitive\": {},\n    \"degenerate_clear_count\": {}\n  }},\n{}",
                n,
                reps,
                resamples,
                arm(&graded),
                arm(&yes),
                arm(&no),
                arm(&sens),
                clear,
                provenance(
                    started,
                    seed,
                    clock.elapsed().as_secs_f64(),
                    "[{\"url\": \"synthetic: seeded simulation, no external data\", \"license\": \"MIT\"}]"
                )
            )
        }
        "score" => {
            let path = args
                .value("--trials")
                .ok_or_else(|| format!("--trials is required\n\n{USAGE}"))?;
            let text = std::fs::read_to_string(path).map_err(|e| format!("read {path}: {e}"))?;
            let trials = parse_trials(&text)?;
            if trials.is_empty() {
                return Err(format!("{path} holds no trials"));
            }
            let rep = score(&trials, resamples, seed);
            let sources = format!(
                "[{{\"url\": \"{}\", \"license\": \"{}\"}}]",
                json_escape(args.value("--source").unwrap_or(path)),
                json_escape(args.value("--license").unwrap_or("unspecified"))
            );
            format!(
                "  \"kind\": \"score\",\n  \"model\": \"{}\",\n  \"trials_file\": \"{}\",\n  \"resamples\": {},\n  \"report\": {},\n{}",
                json_escape(args.value("--model").unwrap_or("unspecified")),
                json_escape(path),
                resamples,
                report_json(&rep, 2),
                provenance(started, seed, clock.elapsed().as_secs_f64(), &sources)
            )
        }
        "-h" | "--help" | "help" => {
            print!("{USAGE}");
            return Ok(());
        }
        other => return Err(format!("unknown command `{other}`\n\n{USAGE}")),
    };

    if std::path::Path::new(&out).exists() {
        return Err(format!(
            "{out} already exists; results files are never overwritten"
        ));
    }
    if let Some(dir) = std::path::Path::new(&out).parent() {
        if !dir.as_os_str().is_empty() {
            std::fs::create_dir_all(dir).map_err(|e| format!("create {}: {e}", dir.display()))?;
        }
    }
    std::fs::write(&out, format!("{{\n{body}\n}}\n")).map_err(|e| format!("write {out}: {e}"))?;
    println!("wrote {out}");
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(2)
        }
    }
}

use introscope::{
    iso_utc, json_escape, parse_trials, report_json, score, simulate_confidence_channel,
    simulate_detect_power, simulate_impact_matched, simulate_input_only_baseline,
    simulate_selfcross_power, simulate_selfpred_power, simulate_sham_power, simulate_wording_arm,
    synth_trials, BaselinePredictor, ConfidenceKind, ImpactControl, ImpactResponder, Responder,
    SelfCrossSpec, WordingResponder, CONCEPT_DISRUPTION,
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

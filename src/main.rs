use introscope::{
    iso_utc, json_escape, parse_trials, report_json, score, simulate_detect_power,
    simulate_selfcross_power, simulate_selfpred_power, simulate_sham_power, synth_trials,
    Responder, SelfCrossSpec,
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

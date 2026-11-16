//! introscope: does a model's report about its own internal state carry
//! information about that state?
//!
//! Three measures, each scored from a flat trial table and each audited
//! against degenerate responders that have no access to any internal state:
//!
//! - `detect`: the model is asked whether a concept was injected into its
//!   activations. Truth is known by construction. Scored as sensitivity d'.
//! - `selfpred`: the model predicts a property of its own answer to a
//!   hypothetical prompt; a second model makes the same prediction about it.
//!   Scored as self-prediction accuracy minus cross-prediction accuracy.
//! - `frame`: the same first-person question is asked under a lead that
//!   pushes towards "yes", one that pushes towards "no", and a neutral one.
//!   Scored as the fraction of items whose answer follows the lead.
//!
//! No dependencies: parsing, statistics, the PRNG and the JSON writer are all
//! in this file so the component builds offline.

use std::collections::BTreeMap;
use std::fmt::Write as _;

/// One row of the trial table.
///
/// Tab-separated columns: `task item variant truth report`.
///
/// | task       | variant                      | truth                      | report            |
/// |------------|------------------------------|----------------------------|-------------------|
/// | `detect`   | free label                   | `1` injected, `0` control  | `1` yes, `0` no   |
/// | `selfpred` | `self` or `cross`            | the actual behaviour       | the prediction    |
/// | `frame`    | `yes`, `no` or `neutral` lead| unused, write `-`          | `1` yes, `0` no   |
#[derive(Debug, Clone, PartialEq)]
pub struct Trial {
    pub task: String,
    pub item: String,
    pub variant: String,
    pub truth: String,
    pub report: String,
}

pub fn parse_trials(text: &str) -> Result<Vec<Trial>, String> {
    let mut out = Vec::new();
    for (i, raw) in text.lines().enumerate() {
        let line = raw.trim_end_matches('\r');
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() != 5 {
            return Err(format!(
                "line {}: expected 5 tab-separated columns, found {}",
                i + 1,
                cols.len()
            ));
        }
        let task = cols[0].trim();
        if !matches!(task, "detect" | "selfpred" | "frame") {
            return Err(format!("line {}: unknown task `{}`", i + 1, task));
        }
        out.push(Trial {
            task: task.to_string(),
            item: cols[1].trim().to_string(),
            variant: cols[2].trim().to_string(),
            truth: cols[3].trim().to_string(),
            report: cols[4].trim().to_string(),
        });
    }
    Ok(out)
}

pub fn trials_to_tsv(trials: &[Trial]) -> String {
    let mut s = String::from("# task\titem\tvariant\ttruth\treport\n");
    for t in trials {
        let _ = writeln!(
            s,
            "{}\t{}\t{}\t{}\t{}",
            t.task, t.item, t.variant, t.truth, t.report
        );
    }
    s
}

// ---------------------------------------------------------------- PRNG

/// SplitMix64: small, seedable, identical on every platform.
#[derive(Debug, Clone)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed)
    }
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// Uniform in [0, 1).
    pub fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    pub fn below(&mut self, n: usize) -> usize {
        (self.unit() * n as f64) as usize % n.max(1)
    }
    pub fn coin(&mut self, p: f64) -> bool {
        self.unit() < p
    }
}

// ---------------------------------------------------------------- statistics

/// Inverse of the standard normal CDF (Acklam's rational approximation,
/// absolute error below 1.2e-9 on (0, 1)).
pub fn probit(p: f64) -> f64 {
    assert!(p > 0.0 && p < 1.0, "probit needs 0 < p < 1, got {p}");
    const A: [f64; 6] = [
        -3.969683028665376e+01,
        2.209460984245205e+02,
        -2.759285104469687e+02,
        1.383_577_518_672_69e2,
        -3.066479806614716e+01,
        2.506628277459239e+00,
    ];
    const B: [f64; 5] = [
        -5.447609879822406e+01,
        1.615858368580409e+02,
        -1.556989798598866e+02,
        6.680131188771972e+01,
        -1.328068155288572e+01,
    ];
    const C: [f64; 6] = [
        -7.784894002430293e-03,
        -3.223964580411365e-01,
        -2.400758277161838e+00,
        -2.549732539343734e+00,
        4.374664141464968e+00,
        2.938163982698783e+00,
    ];
    const D: [f64; 4] = [
        7.784695709041462e-03,
        3.224671290700398e-01,
        2.445134137142996e+00,
        3.754408661907416e+00,
    ];
    let lo = 0.02425;
    if p < lo {
        let q = (-2.0 * p.ln()).sqrt();
        (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    } else if p <= 1.0 - lo {
        let q = p - 0.5;
        let r = q * q;
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
    } else {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    }
}

/// d' with the log-linear correction (add 0.5 to each cell), so a responder
/// with a perfect or empty cell still gets a finite score.
pub fn d_prime(hits: usize, signal: usize, false_alarms: usize, noise: usize) -> f64 {
    let h = (hits as f64 + 0.5) / (signal as f64 + 1.0);
    let f = (false_alarms as f64 + 0.5) / (noise as f64 + 1.0);
    probit(h) - probit(f)
}

/// Percentile bootstrap interval for a statistic of `n` resampled units.
pub fn bootstrap_ci<F: Fn(&[usize]) -> f64>(
    n: usize,
    stat: F,
    resamples: usize,
    seed: u64,
) -> (f64, f64) {
    if n == 0 || resamples == 0 {
        return (f64::NAN, f64::NAN);
    }
    let mut rng = Rng::new(seed);
    let mut vals = Vec::with_capacity(resamples);
    let mut idx = vec![0usize; n];
    for _ in 0..resamples {
        for slot in idx.iter_mut() {
            *slot = rng.below(n);
        }
        let v = stat(&idx);
        if v.is_finite() {
            vals.push(v);
        }
    }
    if vals.is_empty() {
        return (f64::NAN, f64::NAN);
    }
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let at = |q: f64| vals[((vals.len() - 1) as f64 * q).round() as usize];
    (at(0.025), at(0.975))
}

// ---------------------------------------------------------------- metrics

#[derive(Debug, Clone, PartialEq)]
pub struct Detect {
    pub n_signal: usize,
    pub n_noise: usize,
    pub hit_rate: f64,
    pub false_alarm_rate: f64,
    pub d_prime: f64,
    pub d_prime_ci: (f64, f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelfPred {
    pub n_self: usize,
    pub n_cross: usize,
    pub self_accuracy: f64,
    pub cross_accuracy: f64,
    pub advantage: f64,
    pub advantage_ci: (f64, f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    /// Items that have both a `yes`-lead and a `no`-lead answer.
    pub n_items: usize,
    /// Fraction of those items answering yes under the yes lead and no under
    /// the no lead: the report follows the question, not a state.
    pub lead_follow_rate: f64,
    pub lead_follow_ci: (f64, f64),
    /// Fraction answering the same under both leads.
    pub stable_rate: f64,
    /// Fraction of all `frame` answers that are yes.
    pub yes_rate: f64,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Report {
    pub n_trials: usize,
    pub detect: Option<Detect>,
    pub selfpred: Option<SelfPred>,
    pub frame: Option<Frame>,
}

fn is_yes(s: &str) -> bool {
    matches!(s.to_ascii_lowercase().as_str(), "1" | "yes" | "y" | "true")
}

fn mean(flags: &[bool], idx: &[usize]) -> f64 {
    if idx.is_empty() {
        return f64::NAN;
    }
    idx.iter().filter(|&&i| flags[i]).count() as f64 / idx.len() as f64
}

pub fn score_detect(trials: &[Trial], resamples: usize, seed: u64) -> Option<Detect> {
    let rows: Vec<(bool, bool)> = trials
        .iter()
        .filter(|t| t.task == "detect")
        .map(|t| (is_yes(&t.truth), is_yes(&t.report)))
        .collect();
    let signal: Vec<bool> = rows.iter().filter(|r| r.0).map(|r| r.1).collect();
    let noise: Vec<bool> = rows.iter().filter(|r| !r.0).map(|r| r.1).collect();
    if signal.is_empty() || noise.is_empty() {
        return None;
    }
    let count = |v: &[bool]| v.iter().filter(|&&b| b).count();
    let (hits, fas) = (count(&signal), count(&noise));
    // Resample signal and noise trials independently (stratified bootstrap).
    let (ns, nn) = (signal.len(), noise.len());
    let ci = bootstrap_ci(
        ns + nn,
        |idx| {
            let h = idx[..ns].iter().filter(|&&i| signal[i % ns]).count();
            let f = idx[ns..].iter().filter(|&&i| noise[i % nn]).count();
            d_prime(h, ns, f, nn)
        },
        resamples,
        seed,
    );
    Some(Detect {
        n_signal: ns,
        n_noise: nn,
        hit_rate: hits as f64 / ns as f64,
        false_alarm_rate: fas as f64 / nn as f64,
        d_prime: d_prime(hits, ns, fas, nn),
        d_prime_ci: ci,
    })
}

/// Result of simulating the detect measure at a known true d'.
#[derive(Debug, Clone, PartialEq)]
pub struct PowerArm {
    pub true_d_prime: f64,
    pub mean_d_prime: f64,
    /// Fraction of replicates whose bootstrap interval excludes zero.
    pub excludes_zero_rate: f64,
}

/// Standard normal CDF (Abramowitz-Stegun 7.1.26 via erf).
fn norm_cdf(x: f64) -> f64 {
    let z = x.abs() / std::f64::consts::SQRT_2;
    let t = 1.0 / (1.0 + 0.327_591_1 * z);
    let poly = t
        * (0.254_829_592
            + t * (-0.284_496_736
                + t * (1.421_413_741 + t * (-1.453_152_027 + t * 1.061_405_429))));
    let erf = 1.0 - poly * (-z * z).exp();
    if x >= 0.0 {
        0.5 * (1.0 + erf)
    } else {
        0.5 * (1.0 - erf)
    }
}

/// Simulate `reps` detect tables of `n` signal and `n` noise trials for an
/// equal-variance responder with the given true d' and an unbiased criterion,
/// score each with the production scorer, and summarise.
pub fn simulate_detect_power(
    true_d_prime: f64,
    n: usize,
    reps: usize,
    resamples: usize,
    seed: u64,
) -> PowerArm {
    let hit_p = norm_cdf(true_d_prime / 2.0);
    let fa_p = norm_cdf(-true_d_prime / 2.0);
    let mut rng = Rng::new(seed);
    let (mut sum, mut excl) = (0.0, 0usize);
    for rep in 0..reps {
        let mut trials = Vec::with_capacity(2 * n);
        for i in 0..2 * n {
            let injected = i < n;
            let said = rng.coin(if injected { hit_p } else { fa_p });
            trials.push(Trial {
                task: "detect".into(),
                item: format!("s{rep}-{i}"),
                variant: "free".into(),
                truth: if injected { "1" } else { "0" }.into(),
                report: if said { "1" } else { "0" }.into(),
            });
        }
        let d = score_detect(&trials, resamples, seed.wrapping_add(rep as u64 + 1))
            .expect("both arms present");
        sum += d.d_prime;
        if d.d_prime_ci.0 > 0.0 || d.d_prime_ci.1 < 0.0 {
            excl += 1;
        }
    }
    PowerArm {
        true_d_prime,
        mean_d_prime: sum / reps as f64,
        excludes_zero_rate: excl as f64 / reps as f64,
    }
}

/// Result of simulating the selfpred measure at known accuracies.
#[derive(Debug, Clone, PartialEq)]
pub struct SelfPredArm {
    pub self_accuracy: f64,
    pub cross_accuracy: f64,
    pub mean_advantage: f64,
    /// Fraction of replicates whose bootstrap interval excludes zero.
    pub excludes_zero_rate: f64,
}

/// Simulate `reps` selfpred tables of `n` self and `n` cross trials where the
/// prediction is right with the given probabilities, score each with the
/// production scorer, and summarise.
pub fn simulate_selfpred_power(
    self_accuracy: f64,
    cross_accuracy: f64,
    n: usize,
    reps: usize,
    resamples: usize,
    seed: u64,
) -> SelfPredArm {
    let mut rng = Rng::new(seed);
    let (mut sum, mut excl) = (0.0, 0usize);
    for rep in 0..reps {
        let mut trials = Vec::with_capacity(2 * n);
        for i in 0..2 * n {
            let own = i < n;
            let right = rng.coin(if own { self_accuracy } else { cross_accuracy });
            trials.push(Trial {
                task: "selfpred".into(),
                item: format!("p{rep}-{i}"),
                variant: if own { "self" } else { "cross" }.into(),
                truth: "a".into(),
                report: if right { "a" } else { "b" }.into(),
            });
        }
        let d = score_selfpred(&trials, resamples, seed.wrapping_add(rep as u64 + 1))
            .expect("both arms present");
        sum += d.advantage;
        if d.advantage_ci.0 > 0.0 || d.advantage_ci.1 < 0.0 {
            excl += 1;
        }
    }
    SelfPredArm {
        self_accuracy,
        cross_accuracy,
        mean_advantage: sum / reps as f64,
        excludes_zero_rate: excl as f64 / reps as f64,
    }
}

pub fn score_selfpred(trials: &[Trial], resamples: usize, seed: u64) -> Option<SelfPred> {
    let pick = |variant: &str| -> Vec<bool> {
        trials
            .iter()
            .filter(|t| t.task == "selfpred" && t.variant == variant)
            .map(|t| t.truth.eq_ignore_ascii_case(&t.report))
            .collect()
    };
    let (own, cross) = (pick("self"), pick("cross"));
    if own.is_empty() || cross.is_empty() {
        return None;
    }
    let all = |v: &[bool]| mean(v, &(0..v.len()).collect::<Vec<_>>());
    let (no, nc) = (own.len(), cross.len());
    let ci = bootstrap_ci(
        no + nc,
        |idx| {
            let a = idx[..no].iter().filter(|&&i| own[i % no]).count() as f64 / no as f64;
            let b = idx[no..].iter().filter(|&&i| cross[i % nc]).count() as f64 / nc as f64;
            a - b
        },
        resamples,
        seed,
    );
    Some(SelfPred {
        n_self: no,
        n_cross: nc,
        self_accuracy: all(&own),
        cross_accuracy: all(&cross),
        advantage: all(&own) - all(&cross),
        advantage_ci: ci,
    })
}

pub fn score_frame(trials: &[Trial], resamples: usize, seed: u64) -> Option<Frame> {
    let rows: Vec<&Trial> = trials.iter().filter(|t| t.task == "frame").collect();
    if rows.is_empty() {
        return None;
    }
    // item -> (answer under yes lead, answer under no lead); the last row wins.
    let mut by_item: BTreeMap<&str, (Option<bool>, Option<bool>)> = BTreeMap::new();
    for t in &rows {
        let e = by_item.entry(t.item.as_str()).or_default();
        match t.variant.as_str() {
            "yes" => e.0 = Some(is_yes(&t.report)),
            "no" => e.1 = Some(is_yes(&t.report)),
            _ => {}
        }
    }
    let pairs: Vec<(bool, bool)> = by_item
        .values()
        .filter_map(|(a, b)| Some(((*a)?, (*b)?)))
        .collect();
    if pairs.is_empty() {
        return None;
    }
    let follows: Vec<bool> = pairs.iter().map(|(y, n)| *y && !*n).collect();
    let stable: Vec<bool> = pairs.iter().map(|(y, n)| y == n).collect();
    let all: Vec<usize> = (0..pairs.len()).collect();
    let ci = bootstrap_ci(pairs.len(), |idx| mean(&follows, idx), resamples, seed);
    let yes = rows.iter().filter(|t| is_yes(&t.report)).count() as f64 / rows.len() as f64;
    Some(Frame {
        n_items: pairs.len(),
        lead_follow_rate: mean(&follows, &all),
        lead_follow_ci: ci,
        stable_rate: mean(&stable, &all),
        yes_rate: yes,
    })
}

pub fn score(trials: &[Trial], resamples: usize, seed: u64) -> Report {
    Report {
        n_trials: trials.len(),
        detect: score_detect(trials, resamples, seed),
        selfpred: score_selfpred(trials, resamples, seed.wrapping_add(1)),
        frame: score_frame(trials, resamples, seed.wrapping_add(2)),
    }
}

// ---------------------------------------------------------------- baselines

/// Responders with no access to any internal state. A measure that one of
/// these can pass is not measuring introspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Responder {
    /// Says yes to everything.
    AlwaysYes,
    /// Says no to everything.
    AlwaysNo,
    /// Answers by fair coin.
    Coin,
    /// Gives the answer the question's wording suggests.
    LeadFollower,
}

impl Responder {
    pub const ALL: [Responder; 4] = [
        Responder::AlwaysYes,
        Responder::AlwaysNo,
        Responder::Coin,
        Responder::LeadFollower,
    ];
    pub fn name(self) -> &'static str {
        match self {
            Responder::AlwaysYes => "always_yes",
            Responder::AlwaysNo => "always_no",
            Responder::Coin => "coin",
            Responder::LeadFollower => "lead_follower",
        }
    }
    fn yes_no(self, lead: Option<bool>, rng: &mut Rng) -> bool {
        match self {
            Responder::AlwaysYes => true,
            Responder::AlwaysNo => false,
            Responder::Coin => rng.coin(0.5),
            Responder::LeadFollower => lead.unwrap_or_else(|| rng.coin(0.5)),
        }
    }
}

/// Build `n` items per task answered by a degenerate responder. The world
/// (which trials carry an injection, what the model actually does) is drawn
/// from the seed; the responder never sees it.
pub fn synth_trials(r: Responder, n: usize, seed: u64) -> Vec<Trial> {
    let mut rng = Rng::new(seed);
    let mut out = Vec::with_capacity(n * 6);
    let bit = |b: bool| if b { "1" } else { "0" }.to_string();
    for i in 0..n {
        let injected = i % 2 == 0;
        // The detection question is worded neutrally, so there is no lead.
        let said = r.yes_no(None, &mut rng);
        out.push(Trial {
            task: "detect".into(),
            item: format!("d{i}"),
            variant: "concept".into(),
            truth: bit(injected),
            report: bit(said),
        });
    }
    for i in 0..n {
        let actual = rng.coin(0.5);
        for variant in ["self", "cross"] {
            let pred = r.yes_no(None, &mut rng);
            out.push(Trial {
                task: "selfpred".into(),
                item: format!("s{i}"),
                variant: variant.into(),
                truth: bit(actual),
                report: bit(pred),
            });
        }
    }
    for i in 0..n {
        for (variant, lead) in [("yes", Some(true)), ("no", Some(false)), ("neutral", None)] {
            let said = r.yes_no(lead, &mut rng);
            out.push(Trial {
                task: "frame".into(),
                item: format!("f{i}"),
                variant: variant.into(),
                truth: "-".into(),
                report: bit(said),
            });
        }
    }
    out
}

// ---------------------------------------------------------------- JSON

fn num(v: f64) -> String {
    if v.is_finite() {
        // Six decimals: stable across platforms and enough for a rate.
        let s = format!("{v:.6}");
        if s == "-0.000000" {
            "0.000000".into()
        } else {
            s
        }
    } else {
        "null".into()
    }
}

fn pair(v: (f64, f64)) -> String {
    format!("[{}, {}]", num(v.0), num(v.1))
}

pub fn json_escape(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(o, "\\u{:04x}", c as u32);
            }
            c => o.push(c),
        }
    }
    o
}

/// The report as a JSON object, indented by `pad` spaces.
pub fn report_json(r: &Report, pad: usize) -> String {
    let p = " ".repeat(pad);
    let mut parts = vec![format!("{p}  \"n_trials\": {}", r.n_trials)];
    if let Some(d) = &r.detect {
        parts.push(format!(
            "{p}  \"detect\": {{\"n_signal\": {}, \"n_noise\": {}, \"hit_rate\": {}, \"false_alarm_rate\": {}, \"d_prime\": {}, \"d_prime_ci95\": {}}}",
            d.n_signal, d.n_noise, num(d.hit_rate), num(d.false_alarm_rate), num(d.d_prime), pair(d.d_prime_ci)
        ));
    }
    if let Some(s) = &r.selfpred {
        parts.push(format!(
            "{p}  \"selfpred\": {{\"n_self\": {}, \"n_cross\": {}, \"self_accuracy\": {}, \"cross_accuracy\": {}, \"advantage\": {}, \"advantage_ci95\": {}}}",
            s.n_self, s.n_cross, num(s.self_accuracy), num(s.cross_accuracy), num(s.advantage), pair(s.advantage_ci)
        ));
    }
    if let Some(f) = &r.frame {
        parts.push(format!(
            "{p}  \"frame\": {{\"n_items\": {}, \"lead_follow_rate\": {}, \"lead_follow_ci95\": {}, \"stable_rate\": {}, \"yes_rate\": {}}}",
            f.n_items, num(f.lead_follow_rate), pair(f.lead_follow_ci), num(f.stable_rate), num(f.yes_rate)
        ));
    }
    format!("{{\n{}\n{p}}}", parts.join(",\n"))
}

/// UTC timestamp `YYYY-MM-DDTHH:MM:SSZ` from seconds since the Unix epoch.
pub fn iso_utc(secs: u64) -> String {
    let days = (secs / 86_400) as i64;
    let rem = secs % 86_400;
    // Civil-from-days (Howard Hinnant's algorithm).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z",
        rem / 3_600,
        rem % 3_600 / 60,
        rem % 60
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probit_matches_known_quantiles() {
        assert!(probit(0.5).abs() < 1e-9);
        assert!((probit(0.975) - 1.959964).abs() < 1e-5);
        assert!((probit(0.025) + 1.959964).abs() < 1e-5);
        assert!((probit(0.001) + 3.090232).abs() < 1e-5);
    }

    #[test]
    fn d_prime_is_zero_without_sensitivity_and_finite_at_the_edges() {
        assert!(d_prime(50, 100, 50, 100).abs() < 1e-12);
        assert!(d_prime(100, 100, 100, 100).abs() < 1e-12);
        let perfect = d_prime(100, 100, 0, 100);
        assert!(perfect.is_finite() && perfect > 4.0);
    }

    #[test]
    fn rng_is_reproducible_and_roughly_uniform() {
        let a: Vec<u64> = (0..4)
            .map({
                let mut r = Rng::new(7);
                move |_| r.next_u64()
            })
            .collect();
        let b: Vec<u64> = (0..4)
            .map({
                let mut r = Rng::new(7);
                move |_| r.next_u64()
            })
            .collect();
        assert_eq!(a, b);
        let mut r = Rng::new(1);
        let m = (0..20_000).map(|_| r.unit()).sum::<f64>() / 20_000.0;
        assert!((m - 0.5).abs() < 0.01);
    }

    #[test]
    fn iso_utc_formats_known_instants() {
        assert_eq!(iso_utc(0), "1970-01-01T00:00:00Z");
        assert_eq!(iso_utc(951_782_400), "2000-02-29T00:00:00Z");
        assert_eq!(iso_utc(1_794_787_200), "2026-11-16T00:00:00Z");
    }

    #[test]
    fn json_escape_handles_quotes_and_control_characters() {
        assert_eq!(json_escape("a\"b\\c\n\u{1}"), "a\\\"b\\\\c\\n\\u0001");
    }
}

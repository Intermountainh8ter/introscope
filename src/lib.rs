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

/// Result of simulating a sham-controlled detect score.
#[derive(Debug, Clone, PartialEq)]
pub struct ShamArm {
    pub hit_p: f64,
    pub sham_fa_p: f64,
    /// Mean of hit rate minus sham false-alarm rate over replicates.
    pub mean_gap: f64,
    /// Fraction of replicates whose bootstrap interval excludes zero.
    pub excludes_zero_rate: f64,
}

/// Simulate `reps` tables of `n` injected and `n` sham trials where the
/// responder says yes with probability `hit_p` on injected trials and
/// `sham_fa_p` on sham trials; score hit rate minus sham false-alarm rate
/// with a bootstrap interval and summarise.
pub fn simulate_sham_power(
    hit_p: f64,
    sham_fa_p: f64,
    n: usize,
    reps: usize,
    resamples: usize,
    seed: u64,
) -> ShamArm {
    let mut rng = Rng::new(seed);
    let (mut sum, mut excl) = (0.0, 0usize);
    for rep in 0..reps {
        let inj: Vec<bool> = (0..n).map(|_| rng.coin(hit_p)).collect();
        let sham: Vec<bool> = (0..n).map(|_| rng.coin(sham_fa_p)).collect();
        let gap_of = |idx: &[usize]| {
            let h = idx[..n].iter().filter(|&&i| inj[i % n]).count();
            let f = idx[n..].iter().filter(|&&i| sham[i % n]).count();
            (h as f64 - f as f64) / n as f64
        };
        let all: Vec<usize> = (0..2 * n).collect();
        let gap = gap_of(&all);
        let ci = bootstrap_ci(2 * n, gap_of, resamples, seed.wrapping_add(rep as u64 + 1));
        sum += gap;
        if ci.0 > 0.0 || ci.1 < 0.0 {
            excl += 1;
        }
    }
    ShamArm {
        hit_p,
        sham_fa_p,
        mean_gap: sum / reps as f64,
        excludes_zero_rate: excl as f64 / reps as f64,
    }
}

/// Confidence model of an always-yes responder.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConfidenceKind {
    /// Confidence is normal with mean `d_prime` on injected trials, 0 on sham.
    Graded(f64),
    /// Confidence is the same value on every trial.
    Constant,
    /// Confidence is standard normal, independent of trial status.
    Noise,
}

/// Result of simulating the confidence channel of an always-yes responder.
#[derive(Debug, Clone, PartialEq)]
pub struct ConfidenceArm {
    /// Mean AUROC of confidence for injected versus sham trials.
    pub mean_auroc: f64,
    /// Fraction of replicates whose bootstrap interval excludes 0.5.
    pub auroc_excludes_half_rate: f64,
    /// Mean yes/no sham-adjusted gap (always-yes: 0).
    pub mean_yes_no_gap: f64,
    /// Fraction of replicates whose yes/no gap interval excludes zero.
    pub gap_excludes_zero_rate: f64,
}

/// AUROC of `score` for positives versus negatives, ties counting one half.
/// `None` when either class is absent.
pub fn auroc(items: &[(f64, bool)]) -> Option<f64> {
    let mut v: Vec<(f64, bool)> = items.to_vec();
    v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let pos = v.iter().filter(|x| x.1).count();
    let neg = v.len() - pos;
    if pos == 0 || neg == 0 {
        return None;
    }
    let (mut i, mut rank_sum) = (0, 0.0);
    while i < v.len() {
        let mut j = i;
        while j < v.len() && v[j].0 == v[i].0 {
            j += 1;
        }
        let mid = (i + 1 + j) as f64 / 2.0;
        rank_sum += mid * v[i..j].iter().filter(|x| x.1).count() as f64;
        i = j;
    }
    Some((rank_sum - pos as f64 * (pos as f64 + 1.0) / 2.0) / (pos as f64 * neg as f64))
}

impl Rng {
    /// Standard normal by Box-Muller.
    pub fn normal(&mut self) -> f64 {
        let u = 1.0 - self.unit();
        let w = self.unit();
        (-2.0 * u.ln()).sqrt() * (2.0 * std::f64::consts::PI * w).cos()
    }
}

/// Simulate `reps` tables of `n` injected and `n` sham trials where the
/// responder always says yes and attaches a confidence value drawn per
/// `kind`; score AUROC of confidence and the yes/no sham-adjusted gap.
pub fn simulate_confidence_channel(
    kind: ConfidenceKind,
    n: usize,
    reps: usize,
    resamples: usize,
    seed: u64,
) -> ConfidenceArm {
    let mut rng = Rng::new(seed);
    let (mut sum, mut excl) = (0.0, 0usize);
    for rep in 0..reps {
        let items: Vec<(f64, bool)> = (0..2 * n)
            .map(|i| {
                let injected = i < n;
                let c = match kind {
                    ConfidenceKind::Graded(d) => rng.normal() + if injected { d } else { 0.0 },
                    ConfidenceKind::Constant => 0.7,
                    ConfidenceKind::Noise => rng.normal(),
                };
                (c, injected)
            })
            .collect();
        let a = auroc(&items).expect("both arms present");
        let ci = bootstrap_ci(
            2 * n,
            |idx| {
                let sample: Vec<(f64, bool)> = idx.iter().map(|&i| items[i]).collect();
                auroc(&sample).unwrap_or(f64::NAN)
            },
            resamples,
            seed.wrapping_add(rep as u64 + 1),
        );
        sum += a;
        if ci.0 > 0.5 || ci.1 < 0.5 {
            excl += 1;
        }
    }
    let gap = simulate_sham_power(1.0, 1.0, n, reps, resamples, seed.wrapping_add(7000));
    ConfidenceArm {
        mean_auroc: sum / reps as f64,
        auroc_excludes_half_rate: excl as f64 / reps as f64,
        mean_yes_no_gap: gap.mean_gap,
        gap_excludes_zero_rate: gap.excludes_zero_rate,
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

/// Self-versus-cross prediction accuracies split by item type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelfCrossSpec {
    pub self_disagree: f64,
    pub cross_disagree: f64,
    pub self_consensus: f64,
    pub cross_consensus: f64,
}

/// Advantage on all items and on the disagreement subset for one responder.
#[derive(Debug, Clone, PartialEq)]
pub struct SelfCrossArm {
    pub spec: SelfCrossSpec,
    pub full_advantage: f64,
    pub full_excludes_zero_rate: f64,
    pub subset_advantage: f64,
    pub subset_excludes_zero_rate: f64,
}

/// Simulate `reps` selfpred tables of `n` self and `n` cross items where a
/// share `disagree` of the items are disagreement items (self and cross
/// predictors would differ), and score both the full set and the subset.
pub fn simulate_selfcross_power(
    spec: SelfCrossSpec,
    disagree: f64,
    n: usize,
    reps: usize,
    resamples: usize,
    seed: u64,
) -> SelfCrossArm {
    let mut rng = Rng::new(seed);
    let n_dis = (disagree * n as f64).round() as usize;
    let excl = |d: &SelfPred| d.advantage_ci.0 > 0.0 || d.advantage_ci.1 < 0.0;
    let (mut fsum, mut ssum, mut fx, mut sx) = (0.0, 0.0, 0usize, 0usize);
    for rep in 0..reps {
        let mut all = Vec::with_capacity(2 * n);
        let mut sub = Vec::with_capacity(2 * n_dis);
        for i in 0..2 * n {
            let own = i < n;
            let dis = i % n < n_dis;
            let p = match (own, dis) {
                (true, true) => spec.self_disagree,
                (false, true) => spec.cross_disagree,
                (true, false) => spec.self_consensus,
                (false, false) => spec.cross_consensus,
            };
            let right = rng.coin(p);
            let t = Trial {
                task: "selfpred".into(),
                item: format!("s{rep}-{i}"),
                variant: if own { "self" } else { "cross" }.into(),
                truth: "a".into(),
                report: if right { "a" } else { "b" }.into(),
            };
            if dis {
                sub.push(t.clone());
            }
            all.push(t);
        }
        let s0 = seed.wrapping_add(rep as u64 + 1);
        let f = score_selfpred(&all, resamples, s0).expect("both arms present");
        let g = score_selfpred(&sub, resamples, s0).expect("both arms present");
        fsum += f.advantage;
        ssum += g.advantage;
        fx += excl(&f) as usize;
        sx += excl(&g) as usize;
    }
    let r = reps as f64;
    SelfCrossArm {
        spec,
        full_advantage: fsum / r,
        full_excludes_zero_rate: fx as f64 / r,
        subset_advantage: ssum / r,
        subset_excludes_zero_rate: sx as f64 / r,
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

// ---------------------------------------------------------------- wording arm

/// Leading wordings of one first-person question.
pub const WORDINGS: [&str; 4] = ["neutral", "could", "did", "presupposing"];

/// Responders for the wording arm.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WordingResponder {
    AlwaysYes,
    AlwaysNo,
    /// Answer fixed per item by a hidden state, independent of wording.
    StateTracking,
    /// Says yes exactly when the wording leads towards yes (did, presupposing).
    WordingFollower,
    /// Hidden-state answer, flipped to yes with probability 0.3 under the
    /// two leading wordings (did, presupposing).
    PartlyFollowing,
}

/// Result of simulating one responder over the four wordings.
#[derive(Debug, Clone, PartialEq)]
pub struct WordingArm {
    /// Mean answer shift (yes rate under the wording minus neutral) for
    /// could, did and presupposing, in that order.
    pub mean_shift: [f64; 3],
    /// Fraction of replicates whose paired bootstrap interval of the
    /// presupposing-minus-neutral shift excludes zero.
    pub excludes_zero_rate: f64,
}

/// Simulate `reps` tables of `n` items each asked under all four wordings and
/// score the paired answer shift against the neutral wording.
pub fn simulate_wording_arm(
    r: WordingResponder,
    n: usize,
    reps: usize,
    resamples: usize,
    seed: u64,
) -> WordingArm {
    let mut rng = Rng::new(seed);
    let (mut sums, mut excl) = ([0.0f64; 3], 0usize);
    for rep in 0..reps {
        let mut ans = vec![[false; 4]; n];
        for row in ans.iter_mut() {
            let state = rng.coin(0.5);
            for (w, slot) in row.iter_mut().enumerate() {
                let leading = w >= 2;
                *slot = match r {
                    WordingResponder::AlwaysYes => true,
                    WordingResponder::AlwaysNo => false,
                    WordingResponder::StateTracking => state,
                    WordingResponder::WordingFollower => leading,
                    WordingResponder::PartlyFollowing => {
                        let flip = leading && rng.coin(0.3);
                        state || flip
                    }
                };
            }
        }
        let shift_of = |idx: &[usize], w: usize| {
            let s: f64 = idx
                .iter()
                .map(|&i| ans[i][w] as i32 as f64 - ans[i][0] as i32 as f64)
                .sum();
            s / idx.len() as f64
        };
        let all: Vec<usize> = (0..n).collect();
        for (k, sum) in sums.iter_mut().enumerate() {
            *sum += shift_of(&all, k + 1);
        }
        let ci = bootstrap_ci(
            n,
            |idx| shift_of(idx, 3),
            resamples,
            seed.wrapping_add(rep as u64 + 1),
        );
        if ci.0 > 0.0 || ci.1 < 0.0 {
            excl += 1;
        }
    }
    WordingArm {
        mean_shift: sums.map(|s| s / reps as f64),
        excludes_zero_rate: excl as f64 / reps as f64,
    }
}

// ------------------------------------------------------- input-only baseline

/// Predictors compared against an input-only baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaselinePredictor {
    /// Reads the same input feature as the baseline.
    InputDriven,
    /// Reads the hidden state on half the items, the input feature otherwise.
    PrivilegedState,
    AlwaysYes,
    Coin,
}

/// Mean paired accuracy advantage over the input-only baseline.
#[derive(Debug, Clone, PartialEq)]
pub struct InputOnlyArm {
    pub predictor_accuracy: f64,
    pub baseline_accuracy: f64,
    pub mean_advantage: f64,
    /// Share of replicates whose interval includes zero.
    pub includes_zero_rate: f64,
    /// Share of replicates whose interval lies wholly above zero.
    pub positive_excludes_zero_rate: f64,
}

/// Simulate `reps` tables of `n` items. The label equals the input feature
/// with probability `input_signal` (the rest is hidden state). The input-only
/// baseline predicts the feature. With `relabel`, labels are drawn
/// independently of the feature.
pub fn simulate_input_only_baseline(
    predictor: BaselinePredictor,
    input_signal: f64,
    relabel: bool,
    n: usize,
    reps: usize,
    resamples: usize,
    seed: u64,
) -> InputOnlyArm {
    let mut rng = Rng::new(seed);
    let (mut pa, mut ba, mut adv, mut inc, mut pos) = (0.0, 0.0, 0.0, 0usize, 0usize);
    for rep in 0..reps {
        let mut pc = Vec::with_capacity(n);
        let mut bc = Vec::with_capacity(n);
        for _ in 0..n {
            let feature = rng.coin(0.5);
            let label = if relabel {
                rng.coin(0.5)
            } else if rng.coin(input_signal) {
                feature
            } else {
                !feature
            };
            let guess = match predictor {
                BaselinePredictor::InputDriven => feature,
                BaselinePredictor::PrivilegedState => {
                    if rng.coin(0.5) {
                        label
                    } else {
                        feature
                    }
                }
                BaselinePredictor::AlwaysYes => true,
                BaselinePredictor::Coin => rng.coin(0.5),
            };
            pc.push(guess == label);
            bc.push(feature == label);
        }
        let all: Vec<usize> = (0..n).collect();
        let (a, b) = (mean(&pc, &all), mean(&bc, &all));
        let ci = bootstrap_ci(
            n,
            |idx| mean(&pc, idx) - mean(&bc, idx),
            resamples,
            seed.wrapping_add(rep as u64 + 1),
        );
        pa += a;
        ba += b;
        adv += a - b;
        inc += (ci.0 <= 0.0 && ci.1 >= 0.0) as usize;
        pos += (ci.0 > 0.0) as usize;
    }
    let r = reps as f64;
    InputOnlyArm {
        predictor_accuracy: pa / r,
        baseline_accuracy: ba / r,
        mean_advantage: adv / r,
        includes_zero_rate: inc as f64 / r,
        positive_excludes_zero_rate: pos as f64 / r,
    }
}

/// Responder in the impact-matched control simulation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImpactResponder {
    /// Says yes with probability 0.7734 on concept trials, 0.2266 on control
    /// trials, whatever the disruption.
    ConceptDetector,
    /// Yes-probability rises with perturbation disruption; no concept
    /// information.
    ImpactReactive,
    AlwaysYes,
    Coin,
}

/// Which random direction serves as the control arm.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImpactControl {
    /// Same norm as the concept, lower disruption (mean 0).
    NormMatched,
    /// Same disruption distribution as the concept (mean 1).
    ImpactMatched,
}

/// Summary of one responder against one control.
#[derive(Debug, Clone, PartialEq)]
pub struct ImpactArm {
    /// Mean of concept hit rate minus control yes rate.
    pub mean_gap: f64,
    /// Fraction of replicates whose bootstrap interval excludes zero.
    pub excludes_zero_rate: f64,
}

/// Mean disruption of a concept-injection trial.
pub const CONCEPT_DISRUPTION: f64 = 1.0;

fn impact_yes(r: ImpactResponder, concept: bool, disruption: f64, rng: &mut Rng) -> bool {
    match r {
        ImpactResponder::ConceptDetector => rng.coin(if concept { 0.7734 } else { 0.2266 }),
        ImpactResponder::ImpactReactive => {
            rng.coin(1.0 / (1.0 + (-(1.2 * disruption - 0.6)).exp()))
        }
        ImpactResponder::AlwaysYes => true,
        ImpactResponder::Coin => rng.coin(0.5),
    }
}

/// Simulate `reps` tables of `n` concept and `n` control trials. Each trial
/// carries a disruption drawn from a unit normal, shifted to the concept mean
/// for concept and impact-matched trials; score yes rate on concept minus yes
/// rate on control with a bootstrap interval.
pub fn simulate_impact_matched(
    responder: ImpactResponder,
    control: ImpactControl,
    n: usize,
    reps: usize,
    resamples: usize,
    seed: u64,
) -> ImpactArm {
    let mut rng = Rng::new(seed);
    let ctrl_mean = match control {
        ImpactControl::NormMatched => 0.0,
        ImpactControl::ImpactMatched => CONCEPT_DISRUPTION,
    };
    let (mut sum, mut excl) = (0.0, 0usize);
    for rep in 0..reps {
        let con: Vec<bool> = (0..n)
            .map(|_| {
                let d = CONCEPT_DISRUPTION + rng.normal();
                impact_yes(responder, true, d, &mut rng)
            })
            .collect();
        let ctl: Vec<bool> = (0..n)
            .map(|_| {
                let d = ctrl_mean + rng.normal();
                impact_yes(responder, false, d, &mut rng)
            })
            .collect();
        let gap_of = |idx: &[usize]| {
            let h = idx[..n].iter().filter(|&&i| con[i % n]).count();
            let f = idx[n..].iter().filter(|&&i| ctl[i % n]).count();
            (h as f64 - f as f64) / n as f64
        };
        let all: Vec<usize> = (0..2 * n).collect();
        let gap = gap_of(&all);
        let ci = bootstrap_ci(2 * n, gap_of, resamples, seed.wrapping_add(rep as u64 + 1));
        sum += gap;
        if ci.0 > 0.0 || ci.1 < 0.0 {
            excl += 1;
        }
    }
    ImpactArm {
        mean_gap: sum / reps as f64,
        excludes_zero_rate: excl as f64 / reps as f64,
    }
}

/// Yes-lead added to the yes probability of every arm, one per wording.
pub const WORDING_LEADS: [f64; 5] = [0.0, 0.1, 0.2, 0.3, 0.4];

/// Summary of one responder under one wording.
#[derive(Debug, Clone, PartialEq)]
pub struct WordingCell {
    /// Yes-lead of the wording.
    pub lead: f64,
    /// Mean yes rate pooled over the injected and factual-control arms.
    pub raw_yes_rate: f64,
    /// Mean injected yes rate minus factual-control yes rate.
    pub adjusted_gap: f64,
    /// Fraction of replicates whose bootstrap interval for the gap excludes zero.
    pub excludes_zero_rate: f64,
}

/// Simulate `reps` tables of `n` injected and `n` factual-control trials for a
/// responder whose yes probability is `base_inj` / `base_ctl` plus `lead`
/// (clamped to 1) on each arm. `fixed` forces a constant answer instead.
#[allow(clippy::too_many_arguments)]
pub fn simulate_wording_cell(
    base_inj: f64,
    base_ctl: f64,
    fixed: Option<bool>,
    lead: f64,
    n: usize,
    reps: usize,
    resamples: usize,
    seed: u64,
) -> WordingCell {
    let mut rng = Rng::new(seed);
    let (p_inj, p_ctl) = match fixed {
        Some(true) => (1.0, 1.0),
        Some(false) => (0.0, 0.0),
        None => ((base_inj + lead).min(1.0), (base_ctl + lead).min(1.0)),
    };
    let (mut raw, mut gap_sum, mut excl) = (0.0, 0.0, 0usize);
    for rep in 0..reps {
        let inj: Vec<bool> = (0..n).map(|_| rng.coin(p_inj)).collect();
        let ctl: Vec<bool> = (0..n).map(|_| rng.coin(p_ctl)).collect();
        let gap_of = |idx: &[usize]| {
            let h = idx[..n].iter().filter(|&&i| inj[i % n]).count();
            let f = idx[n..].iter().filter(|&&i| ctl[i % n]).count();
            (h as f64 - f as f64) / n as f64
        };
        let all: Vec<usize> = (0..2 * n).collect();
        let gap = gap_of(&all);
        let yes = inj.iter().chain(ctl.iter()).filter(|&&b| b).count();
        raw += yes as f64 / (2 * n) as f64;
        let ci = bootstrap_ci(2 * n, gap_of, resamples, seed.wrapping_add(rep as u64 + 1));
        gap_sum += gap;
        if ci.0 > 0.0 || ci.1 < 0.0 {
            excl += 1;
        }
    }
    WordingCell {
        lead,
        raw_yes_rate: raw / reps as f64,
        adjusted_gap: gap_sum / reps as f64,
        excludes_zero_rate: excl as f64 / reps as f64,
    }
}

/// Summary of one responder over the concept, random-direction and
/// prompt-only arms.
#[derive(Debug, Clone, PartialEq)]
pub struct ThreeArm {
    /// Mean yes rate on the injected-concept arm.
    pub concept_rate: f64,
    /// Mean yes rate on the norm-matched random-direction arm.
    pub random_rate: f64,
    /// Mean yes rate on the prompt-only (leading wording, no injection) arm.
    pub prompt_rate: f64,
    /// Mean concept yes rate minus random-direction yes rate.
    pub concept_minus_random: f64,
    /// Mean concept yes rate minus prompt-only yes rate.
    pub concept_minus_prompt: f64,
    /// Fraction of replicates whose interval for concept minus random excludes zero.
    pub random_excludes_zero_rate: f64,
    /// Fraction of replicates whose interval for concept minus prompt-only excludes zero.
    pub prompt_excludes_zero_rate: f64,
}

/// Simulate `reps` tables of `n` trials on each of three arms for a responder
/// with yes probabilities `p` (concept, random, prompt-only).
pub fn simulate_three_arm(
    p: [f64; 3],
    n: usize,
    reps: usize,
    resamples: usize,
    seed: u64,
) -> ThreeArm {
    let mut rng = Rng::new(seed);
    let (mut rates, mut gaps) = ([0.0; 3], [0.0; 2]);
    let mut excl = [0usize; 2];
    for rep in 0..reps {
        let arms: Vec<Vec<bool>> = p
            .iter()
            .map(|&q| (0..n).map(|_| rng.coin(q)).collect())
            .collect();
        for (k, a) in arms.iter().enumerate() {
            rates[k] += a.iter().filter(|&&b| b).count() as f64 / n as f64;
        }
        for (g, ctl) in [1usize, 2].iter().enumerate() {
            let gap_of = |idx: &[usize]| {
                let h = idx[..n].iter().filter(|&&i| arms[0][i % n]).count();
                let f = idx[n..].iter().filter(|&&i| arms[*ctl][i % n]).count();
                (h as f64 - f as f64) / n as f64
            };
            let all: Vec<usize> = (0..2 * n).collect();
            gaps[g] += gap_of(&all);
            let ci = bootstrap_ci(
                2 * n,
                gap_of,
                resamples,
                seed.wrapping_add(rep as u64 + 1 + 1000 * g as u64),
            );
            if ci.0 > 0.0 || ci.1 < 0.0 {
                excl[g] += 1;
            }
        }
    }
    let r = reps as f64;
    ThreeArm {
        concept_rate: rates[0] / r,
        random_rate: rates[1] / r,
        prompt_rate: rates[2] / r,
        concept_minus_random: gaps[0] / r,
        concept_minus_prompt: gaps[1] / r,
        random_excludes_zero_rate: excl[0] as f64 / r,
        prompt_excludes_zero_rate: excl[1] as f64 / r,
    }
}

/// How a simulated responder names a concept after saying yes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Naming {
    /// Always names concept 0, whatever was injected.
    Fixed,
    /// Names a uniformly random concept.
    Random,
    /// Names the injected concept with this probability, otherwise a wrong one.
    Aware(f64),
}

/// Summary of one responder in the detect-versus-identify simulation.
#[derive(Debug, Clone, PartialEq)]
pub struct DetectIdentify {
    /// Mean yes rate on the injected-concept arm.
    pub concept_rate: f64,
    /// Mean yes rate on the prompt-only arm.
    pub prompt_rate: f64,
    /// Fraction of replicates whose concept-minus-prompt interval excludes zero.
    pub prompt_excludes_zero_rate: f64,
    /// Mean identification accuracy among detected injected trials (1/k if none).
    pub identification_accuracy: f64,
    /// Mean accuracy minus the reference responder's accuracy.
    pub margin: f64,
    /// Fraction of replicates whose margin interval lies above zero.
    pub margin_up_rate: f64,
}

fn naming_draw(naming: Naming, truth: usize, k: usize, rng: &mut Rng) -> usize {
    match naming {
        Naming::Fixed => 0,
        Naming::Random => rng.below(k),
        Naming::Aware(q) => {
            if rng.coin(q) {
                truth
            } else {
                (truth + 1 + rng.below(k - 1)) % k
            }
        }
    }
}

/// Simulate responders over `n` injected-concept trials (one of `k` equally
/// likely concepts) and `n` prompt-only trials. Each responder is
/// `(p_yes_concept, p_yes_prompt, naming)`; the first is the reference for the
/// identification margin. All responders see the same true concepts in a replicate.
pub fn simulate_detect_identify(
    specs: &[(f64, f64, Naming)],
    k: usize,
    n: usize,
    reps: usize,
    resamples: usize,
    seed: u64,
) -> Vec<DetectIdentify> {
    let m = specs.len();
    let mut rng = Rng::new(seed);
    let mut acc = vec![[0.0f64; 5]; m]; // concept, prompt, excl, accuracy, margin
    let mut up = vec![0usize; m];
    for rep in 0..reps {
        let truth: Vec<usize> = (0..n).map(|_| rng.below(k)).collect();
        // per responder: (yes on concept arm, named concept, yes on prompt arm)
        let sims: Vec<(Vec<bool>, Vec<usize>, Vec<bool>)> = specs
            .iter()
            .map(|&(pc, pp, nm)| {
                let yes: Vec<bool> = (0..n).map(|_| rng.coin(pc)).collect();
                let named: Vec<usize> = truth
                    .iter()
                    .map(|&t| naming_draw(nm, t, k, &mut rng))
                    .collect();
                let pr: Vec<bool> = (0..n).map(|_| rng.coin(pp)).collect();
                (yes, named, pr)
            })
            .collect();
        let ident = |s: &(Vec<bool>, Vec<usize>, Vec<bool>), idx: &[usize]| {
            let det: Vec<usize> = idx.iter().copied().filter(|&i| s.0[i % n]).collect();
            if det.is_empty() {
                return 1.0 / k as f64;
            }
            det.iter().filter(|&&i| s.1[i % n] == truth[i % n]).count() as f64 / det.len() as f64
        };
        let all: Vec<usize> = (0..n).collect();
        let ref_acc = ident(&sims[0], &all);
        for (j, s) in sims.iter().enumerate() {
            acc[j][0] += s.0.iter().filter(|&&b| b).count() as f64 / n as f64;
            acc[j][1] += s.2.iter().filter(|&&b| b).count() as f64 / n as f64;
            let gap_of = |idx: &[usize]| {
                let h = idx[..n].iter().filter(|&&i| s.0[i % n]).count();
                let f = idx[n..].iter().filter(|&&i| s.2[i % n]).count();
                (h as f64 - f as f64) / n as f64
            };
            let ci = bootstrap_ci(
                2 * n,
                gap_of,
                resamples,
                seed.wrapping_add(rep as u64 + 1 + 1000 * j as u64),
            );
            if ci.0 > 0.0 || ci.1 < 0.0 {
                acc[j][2] += 1.0;
            }
            let a = ident(s, &all);
            acc[j][3] += a;
            acc[j][4] += a - ref_acc;
            let r0 = &sims[0];
            let margin_of = |idx: &[usize]| ident(s, idx) - ident(r0, idx);
            let mci = bootstrap_ci(
                n,
                margin_of,
                resamples,
                seed.wrapping_add(rep as u64 + 1 + 77_000 + 1000 * j as u64),
            );
            if mci.0 > 0.0 {
                up[j] += 1;
            }
        }
    }
    let r = reps as f64;
    (0..m)
        .map(|j| DetectIdentify {
            concept_rate: acc[j][0] / r,
            prompt_rate: acc[j][1] / r,
            prompt_excludes_zero_rate: acc[j][2] / r,
            identification_accuracy: acc[j][3] / r,
            margin: acc[j][4] / r,
            margin_up_rate: up[j] as f64 / r,
        })
        .collect()
}

/// Yes probabilities by (state, wording) for the wording-design simulation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WordingProbs {
    pub injected_leading: f64,
    pub injected_neutral: f64,
    pub noise_leading: f64,
    pub noise_neutral: f64,
}

/// Simulate `reps` detect tables of `n` injected and `n` noise trials. In the
/// confounded design every injected trial carries leading wording and every
/// noise trial neutral wording; in the balanced design each arm is split
/// evenly between the two wordings. Scored with the production detect scorer.
pub fn simulate_wording_design(
    p: WordingProbs,
    confounded: bool,
    n: usize,
    reps: usize,
    resamples: usize,
    seed: u64,
) -> PowerArm {
    let mut rng = Rng::new(seed);
    let (mut sum, mut excl) = (0.0, 0usize);
    for rep in 0..reps {
        let mut trials = Vec::with_capacity(2 * n);
        for i in 0..2 * n {
            let injected = i < n;
            let idx = if injected { i } else { i - n };
            let leading = if confounded { injected } else { idx % 2 == 0 };
            let pr = match (injected, leading) {
                (true, true) => p.injected_leading,
                (true, false) => p.injected_neutral,
                (false, true) => p.noise_leading,
                (false, false) => p.noise_neutral,
            };
            let said = rng.coin(pr);
            trials.push(Trial {
                task: "detect".into(),
                item: format!("w{rep}-{i}"),
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
        true_d_prime: 0.0,
        mean_d_prime: sum / reps as f64,
        excludes_zero_rate: excl as f64 / reps as f64,
    }
}

/// True d' of a dose-gated responder: zero below the threshold dose, `strong`
/// at or above it.
pub fn dose_gated_d_prime(dose: f64, threshold: f64, strong: f64) -> f64 {
    if dose >= threshold {
        strong
    } else {
        0.0
    }
}

/// Pass rates of the raw and control-corrected detection gap for one responder
/// at one steering dose.
#[derive(Debug, Clone)]
pub struct ControlArm {
    pub raw_pass_rate: f64,
    pub corrected_pass_rate: f64,
    pub mean_raw_gap: f64,
    pub mean_corrected_gap: f64,
}

/// Simulate steering that lifts the generic Yes rate by `bias` on both the
/// introspection question and matched control questions, plus `detect` on the
/// introspection question only. The raw gap is the introspection yes rate
/// (injected minus noise); the corrected gap subtracts the same difference
/// measured on the control questions. A gap passes when it exceeds `threshold`.
pub fn simulate_control_question(
    base: f64,
    bias: f64,
    detect: f64,
    threshold: f64,
    n: usize,
    reps: usize,
    seed: u64,
) -> ControlArm {
    let mut rng = Rng::new(seed);
    let rate = |rng: &mut Rng, p: f64| (0..n).filter(|_| rng.coin(p)).count() as f64 / n as f64;
    let (mut raw_pass, mut cor_pass, mut raw_sum, mut cor_sum) = (0usize, 0usize, 0.0, 0.0);
    for _ in 0..reps {
        let intro_inj = rate(&mut rng, (base + bias + detect).min(1.0));
        let intro_noise = rate(&mut rng, base);
        let ctrl_inj = rate(&mut rng, (base + bias).min(1.0));
        let ctrl_noise = rate(&mut rng, base);
        let raw = intro_inj - intro_noise;
        let corrected = raw - (ctrl_inj - ctrl_noise);
        raw_sum += raw;
        cor_sum += corrected;
        if raw > threshold {
            raw_pass += 1;
        }
        if corrected > threshold {
            cor_pass += 1;
        }
    }
    let r = reps as f64;
    ControlArm {
        raw_pass_rate: raw_pass as f64 / r,
        corrected_pass_rate: cor_pass as f64 / r,
        mean_raw_gap: raw_sum / r,
        mean_corrected_gap: cor_sum / r,
    }
}

/// Mean detection advantage (yes rate injected minus not injected) of a
/// responder on the matched-question arm and on the mismatched-question arm.
#[derive(Debug, Clone)]
pub struct MismatchArm {
    pub matched_advantage: f64,
    pub mismatch_advantage: f64,
}

/// Simulate a responder injected with concept A and asked either about A
/// (matched) or about B (mismatch). `yes_matched` and `yes_mismatch` are the
/// yes probabilities under injection on each question; `yes_noise` applies
/// without injection on both.
pub fn simulate_concept_mismatch(
    yes_matched: f64,
    yes_mismatch: f64,
    yes_noise: f64,
    n: usize,
    reps: usize,
    seed: u64,
) -> MismatchArm {
    let mut rng = Rng::new(seed);
    let rate = |rng: &mut Rng, p: f64| (0..n).filter(|_| rng.coin(p)).count() as f64 / n as f64;
    let (mut matched_sum, mut mismatch_sum) = (0.0, 0.0);
    for _ in 0..reps {
        matched_sum += rate(&mut rng, yes_matched) - rate(&mut rng, yes_noise);
        mismatch_sum += rate(&mut rng, yes_mismatch) - rate(&mut rng, yes_noise);
    }
    let r = reps as f64;
    MismatchArm {
        matched_advantage: matched_sum / r,
        mismatch_advantage: mismatch_sum / r,
    }
}

/// Mean best-of-layers d' and mean d' at one pre-specified layer.
#[derive(Debug, Clone)]
pub struct LayerArm {
    pub best_layer_mean: f64,
    pub pre_specified_mean: f64,
}

/// Simulate `reps` detect tables of `n` signal and `n` noise trials at each
/// layer. `true_d` gives each layer's true d' (unbiased criterion); the score
/// is the maximum d' over layers and the d' at layer `pre_layer`.
pub fn simulate_layer_selection(
    true_d: &[f64],
    pre_layer: usize,
    n: usize,
    reps: usize,
    seed: u64,
) -> LayerArm {
    let mut rng = Rng::new(seed);
    let (mut best_sum, mut pre_sum) = (0.0, 0.0);
    for _ in 0..reps {
        let mut best = f64::NEG_INFINITY;
        for (l, d) in true_d.iter().enumerate() {
            let hit_p = norm_cdf(d / 2.0);
            let fa_p = norm_cdf(-d / 2.0);
            let hits = (0..n).filter(|_| rng.coin(hit_p)).count();
            let fas = (0..n).filter(|_| rng.coin(fa_p)).count();
            let v = d_prime(hits, n, fas, n);
            if l == pre_layer {
                pre_sum += v;
            }
            best = best.max(v);
        }
        best_sum += best;
    }
    let r = reps as f64;
    LayerArm {
        best_layer_mean: best_sum / r,
        pre_specified_mean: pre_sum / r,
    }
}

/// Mean d' and mean hit-minus-false-alarm gap of a detector under one
/// serving stack that shifts the Yes criterion.
#[derive(Debug, Clone)]
pub struct StackCell {
    pub dprime_mean: f64,
    pub gap_mean: f64,
}

/// Simulate `reps` detect tables of `n` signal and `n` noise trials for a
/// detector with true d' `d` whose Yes criterion is shifted by `shift`
/// (positive shift lowers the Yes rate on both arms).
pub fn simulate_stack_drift(d: f64, shift: f64, n: usize, reps: usize, seed: u64) -> StackCell {
    let mut rng = Rng::new(seed);
    let hit_p = norm_cdf(d / 2.0 - shift);
    let fa_p = norm_cdf(-d / 2.0 - shift);
    let (mut d_sum, mut g_sum) = (0.0, 0.0);
    for _ in 0..reps {
        let hits = (0..n).filter(|_| rng.coin(hit_p)).count();
        let fas = (0..n).filter(|_| rng.coin(fa_p)).count();
        d_sum += d_prime(hits, n, fas, n);
        g_sum += (hits as f64 - fas as f64) / n as f64;
    }
    let r = reps as f64;
    StackCell {
        dprime_mean: d_sum / r,
        gap_mean: g_sum / r,
    }
}

/// Responder whose yes probability varies by item.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemResponder {
    /// No access to the injection: per-item yes probability drawn from
    /// Beta(0.5, 0.5) (mean 0.5), identical in both arms.
    NullItemBias,
    /// True d' of 1.0 with a per-item criterion shift of sd 0.5.
    Detector,
}

/// Trial-level and item-clustered interval behaviour for one responder.
#[derive(Debug, Clone)]
pub struct ClusterArm {
    pub mean_d_prime: f64,
    pub trial_level_excludes_zero_rate: f64,
    pub clustered_excludes_zero_rate: f64,
}

/// Percentile bootstrap of d' over units in each arm. Each unit holds
/// `per_unit` trials and `counts` gives its hits.
fn two_arm_ci(
    sig: &[usize],
    noi: &[usize],
    per_unit: usize,
    resamples: usize,
    rng: &mut Rng,
) -> (f64, f64) {
    let (ns, nn) = (sig.len(), noi.len());
    let mut vals = Vec::with_capacity(resamples);
    for _ in 0..resamples {
        let h: usize = (0..ns).map(|_| sig[rng.below(ns)]).sum();
        let f: usize = (0..nn).map(|_| noi[rng.below(nn)]).sum();
        vals.push(d_prime(h, ns * per_unit, f, nn * per_unit));
    }
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let at = |q: f64| vals[((vals.len() - 1) as f64 * q).round() as usize];
    (at(0.025), at(0.975))
}

/// Simulate `reps` detect tables with `items` items per arm, each sampled `k`
/// times, and score each with a trial-level bootstrap (every sample treated
/// as independent) and an item-clustered bootstrap (items resampled).
pub fn simulate_item_cluster(
    responder: ItemResponder,
    items: usize,
    k: usize,
    reps: usize,
    resamples: usize,
    seed: u64,
) -> ClusterArm {
    let mut rng = Rng::new(seed);
    let (mut d_sum, mut trial_excl, mut clust_excl) = (0.0, 0usize, 0usize);
    for _ in 0..reps {
        let mut arms: [Vec<usize>; 2] = [Vec::new(), Vec::new()];
        for (a, arm) in arms.iter_mut().enumerate() {
            for _ in 0..items {
                let p = match responder {
                    ItemResponder::NullItemBias => {
                        (std::f64::consts::FRAC_PI_2 * rng.unit()).sin().powi(2)
                    }
                    ItemResponder::Detector => {
                        let sign = if a == 0 { 0.5 } else { -0.5 };
                        norm_cdf(sign - 0.5 * rng.normal())
                    }
                };
                arm.push((0..k).filter(|_| rng.coin(p)).count());
            }
        }
        let h: usize = arms[0].iter().sum();
        let f: usize = arms[1].iter().sum();
        d_sum += d_prime(h, items * k, f, items * k);
        let flat = |v: &[usize]| -> Vec<usize> {
            v.iter()
                .flat_map(|c| (0..k).map(move |j| usize::from(j < *c)))
                .collect()
        };
        let (sf, nf) = (flat(&arms[0]), flat(&arms[1]));
        let t = two_arm_ci(&sf, &nf, 1, resamples, &mut rng);
        let c = two_arm_ci(&arms[0], &arms[1], k, resamples, &mut rng);
        if t.0 > 0.0 || t.1 < 0.0 {
            trial_excl += 1;
        }
        if c.0 > 0.0 || c.1 < 0.0 {
            clust_excl += 1;
        }
    }
    let r = reps as f64;
    ClusterArm {
        mean_d_prime: d_sum / r,
        trial_level_excludes_zero_rate: trial_excl as f64 / r,
        clustered_excludes_zero_rate: clust_excl as f64 / r,
    }
}

/// Responder in the identify-versus-frequency-prior simulation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PriorResponder {
    /// Always says yes and names the top-ranked word.
    Guesser,
    /// Says yes and names the injected concept with this probability, else draws from the prior.
    Aware(f64),
    AlwaysYes,
    AlwaysNo,
    Coin,
}

/// Summary of one responder at one vocabulary size.
#[derive(Debug, Clone, PartialEq)]
pub struct PriorCell {
    /// Mean fraction of injected trials with a yes and the right name.
    pub accuracy: f64,
    /// Mean accuracy minus the guesser's accuracy on the same trials.
    pub advantage: f64,
    /// Fraction of replicates whose advantage interval lies above zero.
    pub advantage_up_rate: f64,
}

/// Harmonic number of order 1 for `v` words (Zipf exponent 1 normaliser).
pub fn harmonic(v: usize) -> f64 {
    (1..=v).map(|i| 1.0 / i as f64).sum()
}

fn zipf_draw(cdf: &[f64], rng: &mut Rng) -> usize {
    let u = rng.unit();
    cdf.partition_point(|&c| c <= u).min(cdf.len() - 1)
}

/// Simulate `reps` tables of `n` injected trials whose concept is drawn from a
/// Zipf(1) prior over `v` words. Each responder is scored on identification
/// accuracy (yes and the right name); the first responder is the reference
/// for the advantage. All responders see the same concepts in a replicate.
pub fn simulate_identify_prior(
    responders: &[PriorResponder],
    v: usize,
    n: usize,
    reps: usize,
    resamples: usize,
    seed: u64,
) -> Vec<PriorCell> {
    let h = harmonic(v);
    let mut cdf = Vec::with_capacity(v);
    let mut run = 0.0;
    for i in 1..=v {
        run += 1.0 / (i as f64 * h);
        cdf.push(run);
    }
    let m = responders.len();
    let mut rng = Rng::new(seed);
    let mut sums = vec![[0.0f64; 2]; m];
    let mut up = vec![0usize; m];
    for rep in 0..reps {
        let truth: Vec<usize> = (0..n).map(|_| zipf_draw(&cdf, &mut rng)).collect();
        let hits: Vec<Vec<f64>> = responders
            .iter()
            .map(|&r| {
                truth
                    .iter()
                    .map(|&t| {
                        let ok = match r {
                            PriorResponder::Guesser => t == 0,
                            PriorResponder::Aware(q) => {
                                rng.coin(q) || zipf_draw(&cdf, &mut rng) == t
                            }
                            PriorResponder::AlwaysYes => zipf_draw(&cdf, &mut rng) == t,
                            PriorResponder::AlwaysNo => false,
                            PriorResponder::Coin => rng.coin(0.5) && zipf_draw(&cdf, &mut rng) == t,
                        };
                        ok as u8 as f64
                    })
                    .collect()
            })
            .collect();
        for j in 0..m {
            let acc = hits[j].iter().sum::<f64>() / n as f64;
            let base = hits[0].iter().sum::<f64>() / n as f64;
            sums[j][0] += acc;
            sums[j][1] += acc - base;
            let diff = |idx: &[usize]| {
                idx.iter().map(|&i| hits[j][i] - hits[0][i]).sum::<f64>() / idx.len() as f64
            };
            let ci = bootstrap_ci(
                n,
                diff,
                resamples,
                seed.wrapping_add(rep as u64 + 1 + 1000 * j as u64),
            );
            if ci.0 > 0.0 {
                up[j] += 1;
            }
        }
    }
    let r = reps as f64;
    (0..m)
        .map(|j| PriorCell {
            accuracy: sums[j][0] / r,
            advantage: sums[j][1] / r,
            advantage_up_rate: up[j] as f64 / r,
        })
        .collect()
}

/// One share of the input-only sweep: advantage of each responder over the
/// input-only classifier and the self-predictor's advantage over a cross-predictor.
pub struct ShareCell {
    pub share: f64,
    pub self_advantage: f64,
    pub self_up_rate: f64,
    pub cross_advantage: f64,
    pub cross_up_rate: f64,
    pub always_yes_up_rate: f64,
    pub always_no_up_rate: f64,
    pub coin_up_rate: f64,
}

/// Simulate `reps` tables of `n` binary items at one privileged-signal share.
/// The input-only classifier is right with probability `input_acc`; the
/// self-predictor repeats the input signal except on a share of items where a
/// privileged signal makes it right; the cross-predictor is right with
/// probability `cross_acc`. Advantages are paired differences in accuracy.
pub fn simulate_share_cell(
    share: f64,
    input_acc: f64,
    cross_acc: f64,
    n: usize,
    reps: usize,
    resamples: usize,
    seed: u64,
) -> ShareCell {
    let mut rng = Rng::new(seed);
    let n_priv = (share * n as f64).round() as usize;
    let mut sum_self = 0.0;
    let mut sum_cross = 0.0;
    let mut up = [0usize; 5];
    for rep in 0..reps {
        let mut cols = vec![vec![0.0f64; n]; 6];
        for i in 0..n {
            let truth = rng.coin(0.5);
            let input = rng.coin(input_acc);
            let own = input || i < n_priv;
            let cross = rng.coin(cross_acc);
            let coin = rng.coin(0.5) == truth;
            let vals = [input, own, cross, truth, !truth, coin];
            for (c, v) in cols.iter_mut().zip(vals) {
                c[i] = v as u8 as f64;
            }
        }
        let mean = |c: &[f64]| c.iter().sum::<f64>() / n as f64;
        sum_self += mean(&cols[1]) - mean(&cols[0]);
        sum_cross += mean(&cols[1]) - mean(&cols[2]);
        // (minuend, subtrahend) column pairs: self-input, self-cross, then
        // always-yes, always-no, coin against the input classifier.
        let pairs = [(1, 0), (1, 2), (3, 0), (4, 0), (5, 0)];
        for (k, &(a, b)) in pairs.iter().enumerate() {
            let diff = |idx: &[usize]| {
                idx.iter().map(|&i| cols[a][i] - cols[b][i]).sum::<f64>() / idx.len() as f64
            };
            let ci = bootstrap_ci(
                n,
                diff,
                resamples,
                seed.wrapping_add(rep as u64 + 1 + 1000 * k as u64),
            );
            if ci.0 > 0.0 {
                up[k] += 1;
            }
        }
    }
    let r = reps as f64;
    ShareCell {
        share,
        self_advantage: sum_self / r,
        self_up_rate: up[0] as f64 / r,
        cross_advantage: sum_cross / r,
        cross_up_rate: up[1] as f64 / r,
        always_yes_up_rate: up[2] as f64 / r,
        always_no_up_rate: up[3] as f64 / r,
        coin_up_rate: up[4] as f64 / r,
    }
}

/// One responder under one answer-order design in the answer-position artifact.
pub struct PositionArm {
    pub spurious_effect: f64,
    pub excludes_zero_rate: f64,
}

/// Simulate a responder with no state signal that picks option A with
/// probability `p_a`, scored on a neutral and a threat condition of `n` items.
/// Under fixed order the correct answer is A in the neutral condition and B in
/// the threat condition; under counterbalanced order half of the items in each
/// condition have each answer as correct. The effect is neutral accuracy minus
/// threat accuracy, which is spurious because the responder never sees the state.
pub fn simulate_position_artifact(
    p_a: f64,
    counterbalanced: bool,
    n: usize,
    reps: usize,
    resamples: usize,
    seed: u64,
) -> PositionArm {
    let mut rng = Rng::new(seed);
    let mut sum = 0.0;
    let mut excl = 0usize;
    for rep in 0..reps {
        let mut neutral = vec![0.0f64; n];
        let mut threat = vec![0.0f64; n];
        for i in 0..n {
            let (a_neutral, a_threat) = if counterbalanced {
                (i % 2 == 0, i % 2 == 0)
            } else {
                (true, false)
            };
            neutral[i] = (rng.coin(p_a) == a_neutral) as u8 as f64;
            threat[i] = (rng.coin(p_a) == a_threat) as u8 as f64;
        }
        let drop = |idx: &[usize]| {
            idx.iter().map(|&i| neutral[i] - threat[i]).sum::<f64>() / idx.len() as f64
        };
        sum += neutral.iter().sum::<f64>() / n as f64 - threat.iter().sum::<f64>() / n as f64;
        let ci = bootstrap_ci(n, drop, resamples, seed.wrapping_add(rep as u64 + 1));
        if ci.0 > 0.0 || ci.1 < 0.0 {
            excl += 1;
        }
    }
    PositionArm {
        spurious_effect: sum / reps as f64,
        excludes_zero_rate: excl as f64 / reps as f64,
    }
}

/// Responder kinds for the strength-identity gap simulation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GapResponder {
    /// Graded readout, always names the same concept.
    StrengthOnly,
    /// Same readout, names the injected concept with this probability.
    Aware(f64),
    /// Constant readout, always names the same concept.
    Constant,
    /// Uniform random readout, random naming.
    Coin,
}

/// Mean results for one responder in the strength-identity gap simulation.
pub struct GapArm {
    pub mean_auroc: f64,
    pub mean_identification: f64,
    /// Mean of (AUROC - 0.5) - (identification accuracy - 1/k).
    pub mean_gap: f64,
    /// Mean of the absolute per-replicate gap.
    pub mean_abs_gap: f64,
}

/// Simulate `n` injected and `n` matched-random trials with `k` equally likely
/// concepts. Strength is scored as AUROC of the readout (injected vs matched
/// random); identification is the accuracy of the named concept on injected
/// trials. Both are chance-corrected before subtracting.
pub fn simulate_strength_identity_gap(
    responder: GapResponder,
    k: usize,
    n: usize,
    reps: usize,
    seed: u64,
) -> GapArm {
    let mut rng = Rng::new(seed);
    let (mut sa, mut si, mut sg, mut sabs) = (0.0, 0.0, 0.0, 0.0);
    for _ in 0..reps {
        let mut items = Vec::with_capacity(2 * n);
        let mut hits = 0usize;
        for _ in 0..n {
            let truth = rng.below(k);
            let (score, named) = match responder {
                GapResponder::StrengthOnly => (1.0 + rng.normal(), 0),
                GapResponder::Aware(q) => (
                    1.0 + rng.normal(),
                    naming_draw(Naming::Aware(q), truth, k, &mut rng),
                ),
                GapResponder::Constant => (0.0, 0),
                GapResponder::Coin => (rng.unit(), rng.below(k)),
            };
            if named == truth {
                hits += 1;
            }
            items.push((score, true));
        }
        for _ in 0..n {
            let score = match responder {
                GapResponder::StrengthOnly | GapResponder::Aware(_) => rng.normal(),
                GapResponder::Constant => 0.0,
                GapResponder::Coin => rng.unit(),
            };
            items.push((score, false));
        }
        let a = auroc(&items).unwrap_or(0.5);
        let id = hits as f64 / n as f64;
        let gap = (a - 0.5) - (id - 1.0 / k as f64);
        sa += a;
        si += id;
        sg += gap;
        sabs += gap.abs();
    }
    let r = reps as f64;
    GapArm {
        mean_auroc: sa / r,
        mean_identification: si / r,
        mean_gap: sg / r,
        mean_abs_gap: sabs / r,
    }
}

//! The degenerate-responder audit and the scorer's behaviour on tables whose
//! right answer is known by construction.

use introscope::{parse_trials, score, synth_trials, trials_to_tsv, Responder, Rng, Trial};

const N: usize = 2000;
const RESAMPLES: usize = 400;

fn t(task: &str, item: &str, variant: &str, truth: &str, report: &str) -> Trial {
    Trial {
        task: task.into(),
        item: item.into(),
        variant: variant.into(),
        truth: truth.into(),
        report: report.into(),
    }
}

#[test]
fn no_degenerate_responder_shows_detection_sensitivity() {
    for r in Responder::ALL {
        let d = score(&synth_trials(r, N, 7), RESAMPLES, 7).detect.unwrap();
        assert!(
            d.d_prime.abs() < 0.15,
            "{} reached d' {}",
            r.name(),
            d.d_prime
        );
        assert!(
            d.d_prime_ci.0 <= 0.0 && d.d_prime_ci.1 >= 0.0,
            "{}",
            r.name()
        );
    }
}

#[test]
fn no_degenerate_responder_shows_a_self_prediction_advantage() {
    for r in Responder::ALL {
        let s = score(&synth_trials(r, N, 7), RESAMPLES, 7)
            .selfpred
            .unwrap();
        assert!(
            s.advantage.abs() < 0.05,
            "{} reached advantage {}",
            r.name(),
            s.advantage
        );
        assert!((s.self_accuracy - 0.5).abs() < 0.05, "{}", r.name());
    }
}

#[test]
fn the_frame_measure_separates_the_lead_follower_from_the_constant_responders() {
    let frame = |r| score(&synth_trials(r, N, 7), RESAMPLES, 7).frame.unwrap();
    let follower = frame(Responder::LeadFollower);
    assert_eq!(follower.lead_follow_rate, 1.0);
    assert_eq!(follower.stable_rate, 0.0);
    for r in [Responder::AlwaysYes, Responder::AlwaysNo] {
        let f = frame(r);
        assert_eq!(f.lead_follow_rate, 0.0, "{}", r.name());
        assert_eq!(f.stable_rate, 1.0, "{}", r.name());
    }
    // A coin follows the lead on a quarter of items by chance.
    let coin = frame(Responder::Coin);
    assert!((coin.lead_follow_rate - 0.25).abs() < 0.04);
    assert!((coin.stable_rate - 0.5).abs() < 0.04);
}

#[test]
fn a_responder_that_sees_the_injection_is_detected() {
    // 80 % hits, 10 % false alarms: d' = z(0.8) - z(0.1) = 2.1232.
    let mut rng = Rng::new(3);
    let trials: Vec<Trial> = (0..4000)
        .map(|i| {
            let injected = i % 2 == 0;
            let said = rng.coin(if injected { 0.8 } else { 0.1 });
            t(
                "detect",
                &format!("d{i}"),
                "concept",
                if injected { "1" } else { "0" },
                if said { "1" } else { "0" },
            )
        })
        .collect();
    let d = score(&trials, RESAMPLES, 7).detect.unwrap();
    assert!((d.d_prime - 2.1232).abs() < 0.15, "d' {}", d.d_prime);
    assert!(d.d_prime_ci.0 > 1.5 && d.d_prime_ci.1 < 2.7);
}

#[test]
fn privileged_self_access_shows_as_a_positive_advantage() {
    let mut rng = Rng::new(5);
    let mut trials = Vec::new();
    for i in 0..3000 {
        let actual = rng.coin(0.5);
        for (variant, p_right) in [("self", 0.8), ("cross", 0.6)] {
            let right = rng.coin(p_right);
            let pred = if right { actual } else { !actual };
            trials.push(t(
                "selfpred",
                &format!("s{i}"),
                variant,
                if actual { "1" } else { "0" },
                if pred { "1" } else { "0" },
            ));
        }
    }
    let s = score(&trials, RESAMPLES, 7).selfpred.unwrap();
    assert!(
        (s.advantage - 0.2).abs() < 0.04,
        "advantage {}",
        s.advantage
    );
    assert!(s.advantage_ci.0 > 0.1);
}

#[test]
fn self_prediction_compares_free_text_labels_case_insensitively() {
    let trials = vec![
        t("selfpred", "a", "self", "Paris", "paris"),
        t("selfpred", "b", "self", "even", "odd"),
        t("selfpred", "a", "cross", "Paris", "Rome"),
        t("selfpred", "b", "cross", "even", "odd"),
    ];
    let s = score(&trials, 50, 7).selfpred.unwrap();
    assert_eq!(s.self_accuracy, 0.5);
    assert_eq!(s.cross_accuracy, 0.0);
}

#[test]
fn frame_items_missing_one_lead_are_left_out() {
    let trials = vec![
        t("frame", "q1", "yes", "-", "yes"),
        t("frame", "q1", "no", "-", "no"),
        t("frame", "q2", "yes", "-", "yes"),
        t("frame", "q3", "neutral", "-", "no"),
    ];
    let f = score(&trials, 50, 7).frame.unwrap();
    assert_eq!(f.n_items, 1);
    assert_eq!(f.lead_follow_rate, 1.0);
}

#[test]
fn a_task_with_no_usable_trials_is_absent_from_the_report() {
    let only_signal = vec![t("detect", "d0", "concept", "1", "1")];
    let rep = score(&only_signal, 50, 7);
    assert!(rep.detect.is_none() && rep.selfpred.is_none() && rep.frame.is_none());
    assert_eq!(rep.n_trials, 1);
}

#[test]
fn scoring_is_identical_for_the_same_seed() {
    let trials = synth_trials(Responder::Coin, 300, 11);
    assert_eq!(score(&trials, 200, 7), score(&trials, 200, 7));
}

#[test]
fn the_trial_table_round_trips_through_tsv() {
    let trials = synth_trials(Responder::LeadFollower, 20, 2);
    assert_eq!(parse_trials(&trials_to_tsv(&trials)).unwrap(), trials);
}

#[test]
fn malformed_rows_are_rejected_with_the_line_number() {
    let err = parse_trials("detect\td0\tconcept\t1\n").unwrap_err();
    assert!(err.contains("line 1") && err.contains("5"), "{err}");
    let err = parse_trials("# header\nvibes\tx\ty\t1\t1\n").unwrap_err();
    assert!(err.contains("line 2") && err.contains("vibes"), "{err}");
}

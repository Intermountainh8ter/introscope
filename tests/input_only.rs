use introscope::{simulate_input_only_baseline, BaselinePredictor as P};

#[test]
fn input_driven_has_no_advantage_over_input_only() {
    let a = simulate_input_only_baseline(P::InputDriven, 0.75, false, 400, 40, 300, 7);
    assert!(a.mean_advantage.abs() < 0.05);
    assert!(a.includes_zero_rate >= 0.5);
}

#[test]
fn privileged_state_beats_input_only() {
    let a = simulate_input_only_baseline(P::PrivilegedState, 0.75, false, 400, 40, 300, 7);
    assert!(a.mean_advantage >= 0.10);
    assert!(a.positive_excludes_zero_rate >= 0.8);
}

#[test]
fn degenerate_responders_show_no_positive_advantage() {
    for p in [P::AlwaysYes, P::Coin] {
        let a = simulate_input_only_baseline(p, 0.75, false, 400, 40, 300, 7);
        assert!(a.positive_excludes_zero_rate <= 0.05);
    }
}

#[test]
fn relabeling_drops_input_only_to_chance() {
    let a = simulate_input_only_baseline(P::InputDriven, 0.75, true, 400, 40, 300, 7);
    assert!((a.baseline_accuracy - 0.5).abs() < 0.05);
}

#[test]
fn deterministic_for_a_seed() {
    let a = simulate_input_only_baseline(P::Coin, 0.75, false, 100, 5, 100, 3);
    let b = simulate_input_only_baseline(P::Coin, 0.75, false, 100, 5, 100, 3);
    assert_eq!(a, b);
}

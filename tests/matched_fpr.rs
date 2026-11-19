use introscope::simulate_matched_fpr;

#[test]
fn injected_beats_random_at_matched_fpr() {
    let a = simulate_matched_fpr(1.5, false, 0.10, 400, 40, 7);
    assert!((a.mean_false_alarm_rate - 0.10).abs() < 0.02);
    assert!(a.mean_advantage > 0.40);
    assert!(a.excludes_zero_rate > 0.95);
}

#[test]
fn yes_shift_has_no_advantage() {
    let a = simulate_matched_fpr(0.0, true, 0.10, 400, 40, 7);
    assert!(a.mean_advantage.abs() < 0.02);
    assert!(a.excludes_zero_rate <= 0.20);
}

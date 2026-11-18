use introscope::simulate_stack_drift;

#[test]
fn dprime_is_stable_while_gap_moves_with_criterion() {
    let a = simulate_stack_drift(1.0, 0.0, 400, 40, 7);
    let b = simulate_stack_drift(1.0, 0.6, 400, 40, 7);
    assert!((a.dprime_mean - b.dprime_mean).abs() < 0.1);
    assert!(a.gap_mean > b.gap_mean);
}

#[test]
fn null_detector_stays_near_zero_under_shift() {
    let c = simulate_stack_drift(0.0, -0.6, 400, 40, 7);
    assert!(c.dprime_mean.abs() < 0.1);
}

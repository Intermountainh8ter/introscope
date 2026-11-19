use introscope::simulate_factual_no_control;

#[test]
fn global_shift_cancels() {
    let g = simulate_factual_no_control(0.20, 0.30, 0.30, 400, 40, 100, 7);
    assert!(g.mean_difference_of_shifts.abs() < 0.03);
    assert!(g.excludes_zero_rate <= 0.10);
}

#[test]
fn specific_shift_survives() {
    let s = simulate_factual_no_control(0.20, 0.30, 0.0, 400, 40, 100, 7);
    assert!(s.mean_difference_of_shifts >= 0.25);
    assert!(s.excludes_zero_rate >= 0.95);
}

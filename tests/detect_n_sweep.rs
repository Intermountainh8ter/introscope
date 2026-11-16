use introscope::simulate_detect_power;

#[test]
fn random_direction_stays_near_nominal_at_small_n() {
    let r = simulate_detect_power(0.0, 50, 60, 200, 7);
    assert!(r.excludes_zero_rate <= 0.20);
}

#[test]
fn power_grows_with_n_for_weak_detector() {
    let small = simulate_detect_power(0.5, 25, 60, 200, 7);
    let large = simulate_detect_power(0.5, 400, 60, 200, 7);
    assert!(large.excludes_zero_rate > small.excludes_zero_rate);
    assert!(large.excludes_zero_rate >= 0.9);
}

use introscope::simulate_stack_drift;

#[test]
fn template_effect_exceeds_seed_noise() {
    let hi = simulate_stack_drift(1.3, 0.0, 400, 40, 7).dprime_mean;
    let lo = simulate_stack_drift(0.7, 0.0, 400, 40, 7).dprime_mean;
    let a = simulate_stack_drift(1.0, 0.0, 400, 40, 8).dprime_mean;
    let b = simulate_stack_drift(1.0, 0.0, 400, 40, 9).dprime_mean;
    assert!(hi - lo > 5.0 * (a - b).abs());
}

#[test]
fn null_stays_near_zero() {
    assert!(simulate_stack_drift(0.0, 0.0, 400, 40, 7).dprime_mean.abs() < 0.1);
}

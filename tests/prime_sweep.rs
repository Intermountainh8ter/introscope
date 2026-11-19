use introscope::simulate_prime_sweep;

#[test]
fn sweep_is_monotone_for_prime_only() {
    let s = simulate_prime_sweep(0.30, 0.0, 20, 400, 20, 7);
    assert!(s.points.last().unwrap().prime_only_hit > 0.99);
    assert!(s.points[0].prime_only_hit < 0.01);
}

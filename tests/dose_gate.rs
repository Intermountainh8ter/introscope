use introscope::{dose_gated_d_prime, simulate_detect_power};

#[test]
fn gate_is_zero_below_threshold() {
    assert_eq!(dose_gated_d_prime(0.5, 1.0, 1.5), 0.0);
    assert_eq!(dose_gated_d_prime(1.0, 1.0, 1.5), 1.5);
    assert_eq!(dose_gated_d_prime(2.0, 1.0, 1.5), 1.5);
}

#[test]
fn gated_responder_detects_only_at_threshold() {
    let lo = simulate_detect_power(dose_gated_d_prime(0.5, 1.0, 1.5), 200, 20, 200, 7);
    let hi = simulate_detect_power(dose_gated_d_prime(1.0, 1.0, 1.5), 200, 20, 200, 7);
    assert!(lo.mean_d_prime.abs() < 0.3);
    assert!(hi.mean_d_prime > 1.0);
}

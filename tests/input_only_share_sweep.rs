use introscope::simulate_share_cell;

#[test]
fn zero_share_has_no_advantage_over_input_only() {
    let c = simulate_share_cell(0.0, 0.75, 0.60, 400, 30, 100, 7);
    assert!(c.self_advantage.abs() < 0.03);
    assert_eq!(c.self_up_rate, 0.0);
}

#[test]
fn large_share_is_detected() {
    let c = simulate_share_cell(0.20, 0.75, 0.60, 400, 30, 100, 7);
    assert!(c.self_up_rate >= 0.80);
    assert!(c.cross_advantage > 0.1);
}

#[test]
fn degenerate_responders_do_not_beat_input_only() {
    let c = simulate_share_cell(0.10, 0.75, 0.60, 400, 30, 100, 7);
    assert!(c.always_yes_up_rate <= 0.10);
    assert!(c.always_no_up_rate <= 0.10);
    assert!(c.coin_up_rate <= 0.10);
}

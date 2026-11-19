use introscope::simulate_relabel_control;

#[test]
fn advantage_vanishes_after_relabel() {
    let a = simulate_relabel_control(0.90, 0.80, 0.80, 400, 40, 7);
    assert!(a.advantage_before > 0.08);
    assert!(a.advantage_after.abs() < 0.02);
}

#[test]
fn majority_tracks_base_rate() {
    let a = simulate_relabel_control(0.90, 0.80, 0.80, 400, 40, 7);
    assert!((a.majority_before - 0.80).abs() < 0.03);
    assert!((a.majority_after - 0.80).abs() < 0.03);
}

use introscope::simulate_position_artifact;

#[test]
fn fixed_order_makes_a_spurious_drop() {
    let a = simulate_position_artifact(0.65, false, 400, 30, 100, 7);
    assert!(a.spurious_effect > 0.25 && a.spurious_effect < 0.35);
    assert!(a.excludes_zero_rate >= 0.90);
}

#[test]
fn counterbalancing_removes_the_drop() {
    let a = simulate_position_artifact(0.65, true, 400, 30, 100, 7);
    assert!(a.spurious_effect.abs() < 0.03);
    assert!(a.excludes_zero_rate <= 0.10);
}

#[test]
fn fair_coin_shows_no_drop_under_either_order() {
    for cb in [false, true] {
        let a = simulate_position_artifact(0.5, cb, 400, 30, 100, 7);
        assert!(a.spurious_effect.abs() < 0.03);
        assert!(a.excludes_zero_rate <= 0.10);
    }
}

use introscope::{simulate_strength_identity_gap, GapResponder};

#[test]
fn strength_only_has_positive_gap() {
    let a = simulate_strength_identity_gap(GapResponder::StrengthOnly, 10, 200, 20, 7);
    assert!(a.mean_gap >= 0.2);
    assert!((a.mean_identification - 0.1).abs() < 0.03);
}

#[test]
fn content_aware_gap_is_negative_and_degenerates_near_zero() {
    let a = simulate_strength_identity_gap(GapResponder::Aware(0.8), 10, 200, 20, 7);
    assert!(a.mean_gap < 0.0);
    for r in [GapResponder::Constant, GapResponder::Coin] {
        let d = simulate_strength_identity_gap(r, 10, 200, 20, 7);
        assert!(d.mean_abs_gap <= 0.08);
    }
}

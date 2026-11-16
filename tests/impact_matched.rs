use introscope::{simulate_impact_matched, ImpactControl as C, ImpactResponder as R};

#[test]
fn reactive_responder_beats_norm_matched_but_not_impact_matched() {
    let n = simulate_impact_matched(R::ImpactReactive, C::NormMatched, 400, 40, 300, 7);
    assert!(n.excludes_zero_rate >= 0.8);
    let m = simulate_impact_matched(R::ImpactReactive, C::ImpactMatched, 400, 40, 300, 7);
    assert!(m.excludes_zero_rate <= 0.2);
    assert!(m.mean_gap.abs() < 0.05);
}

#[test]
fn concept_detector_keeps_gap_against_impact_matched() {
    let a = simulate_impact_matched(R::ConceptDetector, C::ImpactMatched, 400, 40, 300, 7);
    assert!(a.mean_gap >= 0.20);
    assert!(a.excludes_zero_rate >= 0.95);
}

#[test]
fn degenerate_responders_stay_at_zero() {
    for r in [R::AlwaysYes, R::Coin] {
        let a = simulate_impact_matched(r, C::ImpactMatched, 400, 40, 300, 7);
        assert!(a.excludes_zero_rate <= 0.05);
    }
}

#[test]
fn deterministic_for_a_seed() {
    let a = simulate_impact_matched(R::Coin, C::NormMatched, 50, 5, 50, 3);
    let b = simulate_impact_matched(R::Coin, C::NormMatched, 50, 5, 50, 3);
    assert_eq!(a, b);
}

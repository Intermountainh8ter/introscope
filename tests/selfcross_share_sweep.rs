use introscope::{simulate_selfcross_power, SelfCrossSpec};

const PRIV: SelfCrossSpec = SelfCrossSpec {
    self_disagree: 0.8,
    cross_disagree: 0.5,
    self_consensus: 0.9,
    cross_consensus: 0.9,
};
const GEN: SelfCrossSpec = SelfCrossSpec {
    self_disagree: 0.7,
    cross_disagree: 0.7,
    self_consensus: 0.7,
    cross_consensus: 0.7,
};

#[test]
fn subset_power_grows_with_share() {
    let small = simulate_selfcross_power(PRIV, 0.05, 400, 60, 200, 7);
    let large = simulate_selfcross_power(PRIV, 0.30, 400, 60, 200, 7);
    assert!(large.subset_excludes_zero_rate > small.subset_excludes_zero_rate);
    assert!(large.subset_excludes_zero_rate >= 0.9);
}

#[test]
fn general_capability_stays_near_nominal() {
    let r = simulate_selfcross_power(GEN, 0.10, 400, 60, 200, 7);
    assert!(r.subset_excludes_zero_rate <= 0.20);
}

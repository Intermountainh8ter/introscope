use introscope::{simulate_item_cluster, ItemResponder};

#[test]
fn pseudoreplication_inflates_trial_level_rate_only() {
    let c = simulate_item_cluster(ItemResponder::NullItemBias, 100, 4, 40, 200, 7);
    assert!(c.trial_level_excludes_zero_rate > c.clustered_excludes_zero_rate);
}

#[test]
fn detector_keeps_clustered_power() {
    let c = simulate_item_cluster(ItemResponder::Detector, 100, 4, 20, 200, 7);
    assert!(c.clustered_excludes_zero_rate >= 0.8);
    assert!(c.mean_d_prime > 0.5);
}

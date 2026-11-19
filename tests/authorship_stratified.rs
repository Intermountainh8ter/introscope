use introscope::simulate_authorship_stratified;

#[test]
fn content_guess_gap_vanishes_within_strata() {
    let a = simulate_authorship_stratified(0.8, 0.2, [[0.2, 0.2], [0.8, 0.8]], 400, 40, 7);
    assert!(a.pooled_authorship_gap >= 0.30);
    assert!(a.within_stratum_authorship_gap.abs() < 0.05);
}

#[test]
fn signal_keeps_within_stratum_gap() {
    let a = simulate_authorship_stratified(0.8, 0.2, [[0.25, 0.75], [0.25, 0.75]], 400, 40, 9);
    assert!(a.within_stratum_authorship_gap >= 0.30);
}

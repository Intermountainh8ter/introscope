use introscope::simulate_concept_mismatch;

#[test]
fn introspector_has_no_mismatch_advantage() {
    let a = simulate_concept_mismatch(0.70, 0.20, 0.20, 400, 40, 7);
    assert!(a.mismatch_advantage.abs() < 0.05);
    assert!(a.matched_advantage > 0.45);
}

#[test]
fn global_shifter_keeps_mismatch_advantage() {
    let a = simulate_concept_mismatch(0.60, 0.60, 0.20, 400, 40, 7);
    assert!(a.mismatch_advantage > 0.35);
}

#[test]
fn reproducible() {
    let a = simulate_concept_mismatch(0.6, 0.6, 0.2, 100, 5, 3);
    let b = simulate_concept_mismatch(0.6, 0.6, 0.2, 100, 5, 3);
    assert_eq!(a.mismatch_advantage, b.mismatch_advantage);
}

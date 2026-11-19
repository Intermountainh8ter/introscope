use introscope::simulate_parse_censoring;

#[test]
fn deterministic_for_a_seed() {
    let a = simulate_parse_censoring(0.5, 10, 100, 10, 7);
    let b = simulate_parse_censoring(0.5, 10, 100, 10, 7);
    assert_eq!(a, b);
}

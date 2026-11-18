use introscope::simulate_three_arm;

#[test]
fn state_reader_separates_concept_from_both_controls() {
    let a = simulate_three_arm([0.7, 0.1, 0.1], 200, 30, 200, 7);
    assert!(a.concept_minus_random > 0.45);
    assert!(a.concept_minus_prompt > 0.45);
    assert!(a.random_excludes_zero_rate > 0.9);
}

#[test]
fn wording_follower_and_constants_show_no_gap() {
    for p in [[0.6; 3], [1.0; 3], [0.0; 3]] {
        let a = simulate_three_arm(p, 100, 30, 200, 7);
        assert!(a.concept_minus_random.abs() < 0.1);
        assert!(a.random_excludes_zero_rate < 0.2);
    }
}

#[test]
fn constants_have_exactly_zero_gap() {
    let a = simulate_three_arm([1.0; 3], 50, 10, 100, 7);
    assert_eq!(a.concept_minus_random, 0.0);
    assert_eq!(a.prompt_excludes_zero_rate, 0.0);
}

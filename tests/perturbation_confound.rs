use introscope::simulate_three_arm;

#[test]
fn shifter_fools_prompt_only_but_not_random() {
    let a = simulate_three_arm([0.45, 0.45, 0.10], 200, 30, 200, 7);
    assert!(a.prompt_excludes_zero_rate >= 0.9);
    assert!(a.random_excludes_zero_rate <= 0.2);
}

#[test]
fn state_reader_beats_both_controls() {
    let a = simulate_three_arm([0.7, 0.1, 0.1], 200, 30, 200, 7);
    assert!(a.prompt_excludes_zero_rate >= 0.9);
    assert!(a.random_excludes_zero_rate >= 0.9);
}

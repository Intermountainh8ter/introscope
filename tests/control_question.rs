use introscope::simulate_control_question;

#[test]
fn pure_bias_passes_raw_but_not_corrected() {
    let a = simulate_control_question(0.3, 0.2, 0.0, 0.10, 400, 40, 7);
    assert!(a.raw_pass_rate > 0.9);
    assert!(a.corrected_pass_rate < 0.2);
}

#[test]
fn true_detection_passes_corrected() {
    let a = simulate_control_question(0.3, 0.2, 0.2, 0.10, 400, 40, 7);
    assert!(a.corrected_pass_rate > 0.8);
}

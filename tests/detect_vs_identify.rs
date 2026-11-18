use introscope::{simulate_detect_identify, Naming};

fn specs() -> Vec<(f64, f64, Naming)> {
    vec![
        (0.6, 0.1, Naming::Fixed),
        (0.7, 0.1, Naming::Aware(0.8)),
        (1.0, 1.0, Naming::Fixed),
        (0.0, 0.0, Naming::Random),
        (0.5, 0.5, Naming::Random),
    ]
}

#[test]
fn agnostic_detects_but_does_not_identify() {
    let r = simulate_detect_identify(&specs(), 10, 200, 20, 200, 7);
    assert!(r[0].prompt_excludes_zero_rate >= 0.9);
    assert!((r[0].identification_accuracy - 0.1).abs() < 0.06);
}

#[test]
fn aware_beats_agnostic_and_degenerates_do_not() {
    let r = simulate_detect_identify(&specs(), 10, 200, 20, 200, 7);
    assert!(r[1].margin >= 0.5);
    for d in &r[2..] {
        assert!(d.margin_up_rate <= 0.2);
    }
}

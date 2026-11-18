use introscope::{harmonic, simulate_identify_prior, PriorResponder};

const RS: [PriorResponder; 4] = [
    PriorResponder::Guesser,
    PriorResponder::Aware(0.8),
    PriorResponder::AlwaysYes,
    PriorResponder::AlwaysNo,
];

#[test]
fn guesser_matches_inverse_harmonic() {
    for v in [10, 1000] {
        let c = simulate_identify_prior(&RS, v, 400, 30, 100, 7);
        assert!((c[0].accuracy - 1.0 / harmonic(v)).abs() < 0.03);
    }
}

#[test]
fn aware_advantage_grows_with_vocabulary() {
    let a = simulate_identify_prior(&RS, 10, 400, 20, 100, 7)[1].advantage;
    let b = simulate_identify_prior(&RS, 1000, 400, 20, 100, 7)[1].advantage;
    assert!(b - a > 0.10);
}

#[test]
fn degenerate_responders_have_no_upward_advantage() {
    let c = simulate_identify_prior(&RS, 100, 400, 20, 100, 7);
    assert!(c[2].advantage_up_rate <= 0.10 && c[3].advantage_up_rate <= 0.10);
}

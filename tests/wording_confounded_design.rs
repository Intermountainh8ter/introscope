use introscope::{simulate_wording_design, WordingProbs};

fn follower() -> WordingProbs {
    WordingProbs {
        injected_leading: 0.7,
        injected_neutral: 0.3,
        noise_leading: 0.7,
        noise_neutral: 0.3,
    }
}

#[test]
fn follower_scores_only_when_confounded() {
    let c = simulate_wording_design(follower(), true, 200, 20, 200, 7);
    let b = simulate_wording_design(follower(), false, 200, 20, 200, 7);
    assert!(c.mean_d_prime > 0.7);
    assert!(b.mean_d_prime.abs() < 0.2);
}

#[test]
fn deterministic() {
    let a = simulate_wording_design(follower(), true, 100, 5, 100, 3);
    let b = simulate_wording_design(follower(), true, 100, 5, 100, 3);
    assert_eq!(a, b);
}

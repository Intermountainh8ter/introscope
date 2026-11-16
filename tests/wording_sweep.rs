use introscope::simulate_wording_cell;

#[test]
fn lead_moves_raw_rate_but_not_adjusted_gap_for_graded() {
    let a = simulate_wording_cell(0.55, 0.05, None, 0.0, 200, 40, 200, 7);
    let b = simulate_wording_cell(0.55, 0.05, None, 0.4, 200, 40, 200, 7);
    assert!(b.raw_yes_rate - a.raw_yes_rate > 0.3);
    assert!((b.adjusted_gap - a.adjusted_gap).abs() < 0.08);
}

#[test]
fn fixed_responders_have_zero_gap() {
    for fx in [true, false] {
        let c = simulate_wording_cell(0.0, 0.0, Some(fx), 0.3, 100, 20, 100, 7);
        assert_eq!(c.adjusted_gap, 0.0);
        assert_eq!(c.excludes_zero_rate, 0.0);
    }
}

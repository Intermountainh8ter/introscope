use introscope::simulate_layer_selection;

#[test]
fn best_of_layers_inflates_null() {
    let a = simulate_layer_selection(&[0.0; 24], 12, 100, 40, 7);
    assert!(a.best_layer_mean > a.pre_specified_mean + 0.15);
    assert!(a.pre_specified_mean.abs() < 0.15);
}

#[test]
fn introspector_survives_selection() {
    let mut d = vec![0.0; 24];
    d[12] = 1.0;
    let a = simulate_layer_selection(&d, 12, 100, 40, 7);
    assert!(a.best_layer_mean > 0.8);
}

#[test]
fn deterministic() {
    let a = simulate_layer_selection(&[0.0; 4], 1, 50, 5, 3);
    let b = simulate_layer_selection(&[0.0; 4], 1, 50, 5, 3);
    assert_eq!(a.best_layer_mean, b.best_layer_mean);
}

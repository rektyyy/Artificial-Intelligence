fn square_area_to_circle(size:f64) -> f64 {
    let r: f64 = size.sqrt() / 2.0;
    return std::f64::consts::PI * r.powi(2);
}
fn assert_close(a:f64, b:f64, epsilon:f64) {
    assert!( (a-b).abs() < epsilon, "Expected: {}, got: {}",b,a);
}

#[test]
fn test1() {
    assert_close(square_area_to_circle(9.0), 7.0685834705770345, 1e-8);
}

#[test]
fn test2() {
    assert_close(square_area_to_circle(20.0), 15.70796326794897, 1e-8);
}

#[test]
fn test3() {
    assert_close(square_area_to_circle(0.0), 0.0, 1e-8);
}

#[test]
fn test4() {
    assert_close(square_area_to_circle(1.0), 0.7853981633974483, 1e-8);
}

#[test]
fn test5() {
    assert_close(square_area_to_circle(100.0), 78.53981633974483, 1e-8);
}
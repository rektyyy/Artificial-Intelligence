fn summy(strng: &str) -> i32 {
    strng.split_whitespace().fold(0, |acc, x| {
        acc + x.parse::<i32>().unwrap_or(0)
    })
}

#[test]
fn test_1() {
    assert_eq!(summy("1 2 3"), 6);
}

#[test]
fn test_2() {
    assert_eq!(summy("1 2 3 4"), 10);
}

#[test]
fn test_3() {
    assert_eq!(summy("1 2 3 4 5"), 15);
}

#[test]
fn test_4() {
    assert_eq!(summy("10 10"), 20);
}

#[test]
fn test_5() {
    assert_eq!(summy("0 0"), 0);
}
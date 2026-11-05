fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
  data.into_iter().map(|(age, handicap)| {
    if age >= 55 && handicap > 7{
      "Senior".to_string()
    }
    else{
      "Open".to_string()
    }
  }).collect()
}

#[test]
fn returns_expected1() {
    assert_eq!(open_or_senior(vec![(45, 12), (55,21), (19, -2), (104, 20)]), vec!["Open", "Senior", "Open", "Senior"]);
}

#[test]
fn returns_expected2() {
  assert_eq!(open_or_senior(vec![(55, 8)]), vec!["Senior"]);
}

#[test]
fn returns_expected3() {
  assert_eq!(open_or_senior(vec![(54, 20)]), vec!["Open"]);
}

#[test]
fn returns_expected4() {
  assert_eq!(open_or_senior(vec![(55, 7)]), vec!["Open"]);
}

#[test]
fn returns_expected5() {
  assert_eq!(open_or_senior(vec![]), Vec::<String>::new());
}
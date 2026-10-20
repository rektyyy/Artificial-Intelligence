fn number(bus_stops:&[(i32,i32)]) -> i32 {
    let mut result: i32 = 0;
    for (start, end) in bus_stops{
        result += start - end;
    }
    result
}

#[test]
fn test1() {
  assert_eq!(number(&[(10,0),(3,5),(5,8)]), 5);
}

#[test]
fn test2() {
  assert_eq!(number(&[(3,0),(9,1),(4,8),(12,2),(6,1),(7,8)]), 21);
}

#[test]
fn test3() {
  assert_eq!(number(&[(3,0),(3,1),(5,8)]), 2);
}

#[test]
fn test4() {
  assert_eq!(number(&[(3,0),(9,1),(4,8),(12,2),(6,1),(7,8)]), 21);
}

#[test]
fn test5() {
  assert_eq!(number(&[(3,0),(2,1),(3,0)]), 7);
}
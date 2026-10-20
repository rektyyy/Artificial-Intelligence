fn get_count(string: &str) -> usize {
  let mut vowels_count: usize = 0;
  let vowels = ['a', 'e', 'i', 'o', 'u'];
  for c in string.chars(){
    if vowels.contains(&c){
        vowels_count += 1;
    }
  }
  vowels_count
}

#[test]
fn test1() {
  assert_eq!(get_count("abracadabra"), 5);
}
#[test]
fn test2() {
  assert_eq!(get_count("aaaa"), 4);
}
#[test]
fn test3() {
  assert_eq!(get_count("abcd"), 1);
}
#[test]
fn test4() {
  assert_eq!(get_count("elo elo"), 4);
}
#[test]
fn test5() {
  assert_eq!(get_count("claudio"), 4);
}
fn solution(num: i32) -> i32 {
  (1..num).filter(|&x| x % 3 == 0|| x % 5 == 0).sum()
}

mod tests {
    use super::solution;
    
    #[test]
    fn sample_tests() {
      assertion(25719750, 10500);
    }
    #[test]
    fn sample_tests2() {
        assertion(  23,   10);
    }
      #[test]
    fn sample_tests3() {
        assertion(  33,   11);
    }
      #[test]
    fn sample_tests4() {
        assertion( 225,   33);
    }
      #[test]
    fn sample_tests5() {
        
      assertion(   8,    6);
    }

    fn assertion(expected : i32, input : i32) {
        let actual = solution(input);
        
        assert!(
            expected == actual,
            "\nTest failed!\n expected: {}\n actual: {}\n input: {}\n", expected, actual, input
        );
    }
}
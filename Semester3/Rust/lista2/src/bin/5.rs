fn count_bits(n: i64) -> u32 {
    let mut count: u32 = 0;
    let mut number: i64 = n;
    while number > 0{
        if number % 2 == 1{
            count += 1;
        }
        number >>= 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(n: i64, expected: u32) {
        let actual = count_bits(n);
        assert_eq!(actual, expected,
            "expected {} but got {} for n = {}",
            expected, actual, n
        );
    }

    #[test] 
    fn sample_tests() {
        do_test(77231418, 14);
    }
    #[test]
    fn sample_tests2() {
        do_test(0, 0);
    }

    #[test]
    fn sample_tests3() {
        do_test(674259, 10);
    }

    #[test]
    fn sample_tests4() {
        do_test(7, 3);
    }

    #[test]
    fn sample_tests5() {
        do_test(4, 1);
    }
}

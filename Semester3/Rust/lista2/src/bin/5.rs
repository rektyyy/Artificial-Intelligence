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
        do_test(0, 0);
        do_test(4, 1);
        do_test(7, 3);
        do_test(77231418, 14);
        do_test(12525589, 11);
        do_test(31, 5);
        do_test(417862, 7);
        do_test(89, 4);
        do_test(674259, 10);
        do_test(i64::MAX, 63)
    }
}

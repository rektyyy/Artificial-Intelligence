fn string_to_number(s: &str) -> i32 {
    return s.parse::<i32>().unwrap();
}

#[cfg(test)]
mod tests {
    use super::string_to_number;

    #[test]
    fn returns_expected() {
      assert_eq!(string_to_number("1234"), 1234);
    }

    #[test]
    fn returns_expected2(){
      assert_eq!(string_to_number("605"), 605);
    }

    #[test]
    fn returns_expected3(){
      assert_eq!(string_to_number("1405"), 1405);
    }

    #[test]
    fn returns_expected4(){
      assert_eq!(string_to_number("1405"), 1405);
    }

    #[test]
    fn works_on_random() {
        for _ in 0..5 {
            let num : i32 = rand::random();
            let input = num.to_string();
            assert_eq!(string_to_number(&input), num);
        }        
    }
}
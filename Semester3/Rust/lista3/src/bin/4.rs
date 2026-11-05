fn longest(a1: &str, a2: &str) -> String {
    ('a'..='z').filter(|&x| a1.contains(x) || a2.contains(x)).collect()
}

#[cfg(test)]
    mod tests {
    use super::*;
   
    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest(s1, s2), exp);
        println!("{}", longest(s1, s2) == exp);
        assert_eq!(&longest(s1, s2), exp)
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
    }

    #[test]
    fn test2(){
        testing("a", "b", "ab");
    }

    #[test]
    fn test3(){
        testing("abc", "efg", "abcefg");
    }

    #[test]
    fn test4(){
        testing("bbbcas", "las", "abcls");
    }

    #[test]
    fn test5(){
        testing("loopingisfunbutdangerous", "lessdangerousthancoding", "abcdefghilnoprstu");
    }
}

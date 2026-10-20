fn printer_error(s: &str) -> String {
    let mut bad: i32 = 0;
    for c in s.chars(){
        if c < 'a' || c > 'm'{
            bad += 1;
        }
    }
    format!("{}/{}", bad, s.len())
}


#[test]
fn test1() {
    assert_eq!(&printer_error("aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"), "3/56");
}

#[test]
fn test2() {
    assert_eq!(&printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"), "6/60");
}

#[test]
fn test3() {
    assert_eq!(&printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyzuuuuu"), "11/65");
}

#[test]
fn test4() {
    assert_eq!(&printer_error("zzzzzz"), "6/6");
}

#[test]
fn test5() {
    assert_eq!(&printer_error("aaaaaa"), "0/6");
}
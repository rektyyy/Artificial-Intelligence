fn dna_strand(dna: &str) -> String {
    let mut res: String = Default::default();
    for c in dna.chars(){
        if c == 'A'{
            res.push('T');
        }
        else if c == 'T'{
            res.push('A');
        }
        else if c == 'C'{
            res.push('G');
        }
        else if c == 'G'{
            res.push('C');
        }
    
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::dna_strand;

    fn dotest(s: &str, expected: &str) {
        let actual = dna_strand(s);
        assert!(actual == expected, 
            "With dna = \"{s}\"\nExpected \"{expected}\" but got \"{actual}\"")
    }
    
    #[test]
    fn test1() {
        dotest("AAAA","TTTT");
    }
    #[test]
    fn test2() {
        dotest("ATTGC","TAACG");
    }
    #[test]
    fn test3() {
        dotest("GTAT","CATA");
    }
    #[test]
    fn test4() {
        dotest("A","T");
    }
    #[test]
    fn test5() {
        dotest("G","C");
    }
}
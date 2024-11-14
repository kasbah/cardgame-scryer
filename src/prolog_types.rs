use scryer_prolog::Term;
use std::collections::BTreeMap;

pub fn from_prolog(term: &Term) -> BTreeMap<String, String> {
    match &term {
        Term::Compound(str, args) if str == "t" => {
            let mut map = BTreeMap::new();
            println!("{:?}", args);
            map
        }
        Term::Atom(str) if str == "t" => BTreeMap::new(),
        _ => panic!("Unexpected term: {:?}", &term),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_from_prolog_empty_assoc() {
        let term = Term::Atom(String::from("t"));
        let map = from_prolog(&term);
        assert_eq!(map, BTreeMap::new());
    }
}

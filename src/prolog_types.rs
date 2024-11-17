use scryer_prolog::Term;
use std::collections::BTreeMap;

pub fn from_prolog(term: &Term) -> BTreeMap<String, String> {
    match &term {
        Term::Compound(str, args) if str == "t" => match (args.get(0), args.get(1)) {
            (Some(Term::Atom(key)), Some(Term::Atom(value))) => {
                let mut map = BTreeMap::new();
                map.insert(key.to_owned(), value.to_owned());
                map
            }
            _ => panic!("Unexpected args: {:?}", args),
        },
        Term::Atom(str) if str == "t" => BTreeMap::new(),
        _ => panic!("Unexpected term: {:?}", &term),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use scryer_prolog::{LeafAnswer, Machine, MachineBuilder, Term};

    fn run_once(machine: &mut Machine, query: &str) -> Result<LeafAnswer, String> {
        let answers = machine.run_query(query);
        for answer in answers {
            return answer;
        }
        panic!("No answer");
    }
    #[test]
    fn test_from_prolog_empty_assoc() {
        let term = Term::Atom(String::from("t"));
        let map = from_prolog(&term);
        assert_eq!(map, BTreeMap::new());
    }

    #[test]
    fn test_from_prolog_assoc() {
        let mut machine = MachineBuilder::default().build();

        machine.load_module_string(
            "test",
            r#"
            :- use_module(library(assoc)).
        "#,
        );

        let query = r#"
            list_to_assoc([a-b], X).
        "#;
        let answer = run_once(&mut machine, query);
        match answer {
            Ok(LeafAnswer::LeafAnswer { bindings, .. }) => match bindings.get("X") {
                Some(x) => {
                    let map = from_prolog(x);
                    assert_eq!(
                        map,
                        BTreeMap::from([(String::from("a"), String::from("b"))])
                    );
                }
                _ => panic!("Unexpected bindings: {:?}", bindings),
            },
            _ => panic!("Unexpected answer: {:?}", answer),
        }
    }
}

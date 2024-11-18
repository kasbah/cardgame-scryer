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
    use scryer_prolog::{
        LeafAnswer, Machine, MachineBuilder,
        Term::{Atom, Compound},
    };

    fn query_once_binding(machine: &mut Machine, query: &str, var: &str) -> Term {
        let mut answers = machine.run_query(query);
        let answer = answers.next();
        match answer {
            Some(Ok(LeafAnswer::LeafAnswer { bindings, .. })) => match bindings.get(var) {
                Some(x) => return x.to_owned(),
                _ => panic!("Unexpected bindings: {:?}", bindings),
            },
            _ => panic!("Unexpected answer: {:?}", answer),
        }
    }

    #[test]
    fn test_from_prolog_empty_assoc() {
        let term = Atom("t".to_string());
        let map = from_prolog(&term);
        assert_eq!(map, BTreeMap::new());
    }

    #[test]
    fn test_from_prolog_assoc1() {
        let term = Compound(
            "t".to_string(),
            Vec::from([
                Atom("a".to_string()),
                Atom("b".to_string()),
                Atom("-".to_string()),
                Atom("t".to_string()),
                Atom("t".to_string()),
            ]),
        );
        let map = from_prolog(&term);
        assert_eq!(map, BTreeMap::from([("a".to_string(), "b".to_string())]));
    }

    #[test]
    fn test_from_prolog_assoc() {
        let mut machine = MachineBuilder::default().build();

        machine.load_module_string("test", r#":- use_module(library(assoc))."#);

        let query = r#"
            list_to_assoc([a-b], X).
        "#;
        let term = query_once_binding(&mut machine, query, "X");
        let map = from_prolog(&term);
        assert_eq!(
            map,
            BTreeMap::from([(String::from("a"), String::from("b"))])
        );
    }
}

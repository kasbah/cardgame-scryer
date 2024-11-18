use scryer_prolog::{
    Term,
    Term::{Atom, Compound},
};
use std::collections::BTreeMap;

pub fn from_prolog_assoc(term: &Term) -> BTreeMap<String, String> {
    match &term {
        Compound(str, args) if str == "t" => match &args[..] {
            [Atom(key), Atom(value), rest @ ..] => {
                let mut map = BTreeMap::new();
                map.insert(key.to_owned(), value.to_owned());
                match rest {
                    [Atom(s), terms @ ..] if s == "<" || s == "-" => {
                        for term in terms {
                            map.append(&mut from_prolog_assoc(term));
                        }
                        map
                    }
                    _ => panic!("Unexpected rest args: {:?}", rest),
                }
            }
            _ => panic!("Unexpected args: {:?}", args),
        },
        Atom(str) if str == "t" => BTreeMap::new(),
        _ => panic!("Unexpected term: {:?}", &term),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use scryer_prolog::{LeafAnswer, Machine, MachineBuilder};

    fn query_once_binding(machine: &mut Machine, query: &str, var: &str) -> Term {
        let mut answers = machine.run_query(query);
        let answer = answers.next();
        match answer {
            Some(Ok(LeafAnswer::LeafAnswer { bindings, .. })) => match bindings.get(var) {
                Some(x) => {
                    println!("{:?}", x);
                    return x.to_owned();
                }
                _ => panic!("Unexpected bindings: {:?}", bindings),
            },
            _ => panic!("Unexpected answer: {:?}", answer),
        }
    }

    #[test]
    fn test_from_prolog_empty_assoc() {
        let term = Atom("t".to_string());
        let map = from_prolog_assoc(&term);
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
        let map = from_prolog_assoc(&term);
        assert_eq!(map, BTreeMap::from([("a".to_string(), "b".to_string())]));
    }
    #[test]
    fn test_from_prolog_assoc2() {
        let term = Compound(
            "t".to_string(),
            Vec::from([
                Atom("c".to_string()),
                Atom("d".to_string()),
                Atom("<".to_string()),
                Compound(
                    "t".to_string(),
                    Vec::from([
                        Atom("a".to_string()),
                        Atom("b".to_string()),
                        Atom("-".to_string()),
                        Atom("t".to_string()),
                        Atom("t".to_string()),
                    ]),
                ),
                Atom("t".to_string()),
            ]),
        );
        let map = from_prolog_assoc(&term);
        assert_eq!(
            map,
            BTreeMap::from([
                ("a".to_string(), "b".to_string()),
                ("c".to_string(), "d".to_string())
            ])
        );
    }

    #[test]
    fn test_from_prolog_assoc3() {
        let term = Compound(
            "t".to_string(),
            Vec::from([
                Atom("c".to_string()),
                Atom("d".to_string()),
                Atom("-".to_string()),
                Compound(
                    "t".to_string(),
                    Vec::from([
                        Atom("a".to_string()),
                        Atom("b".to_string()),
                        Atom("-".to_string()),
                        Atom("t".to_string()),
                        Atom("t".to_string()),
                    ]),
                ),
                Compound(
                    "t".to_string(),
                    Vec::from([
                        Atom("e".to_string()),
                        Atom("f".to_string()),
                        Atom("-".to_string()),
                        Atom("t".to_string()),
                        Atom("t".to_string()),
                    ]),
                ),
            ]),
        );
        let map = from_prolog_assoc(&term);
        assert_eq!(
            map,
            BTreeMap::from([
                ("a".to_string(), "b".to_string()),
                ("c".to_string(), "d".to_string()),
                ("e".to_string(), "f".to_string())
            ])
        );
    }

    #[test]
    fn test_from_prolog_assoc() {
        let mut machine = MachineBuilder::default().build();

        machine.load_module_string("test", r#":- use_module(library(assoc))."#);

        let query = r#"
            list_to_assoc([a-b, c-d, e-f, h-i, j-k], X).
        "#;
        let term = query_once_binding(&mut machine, query, "X");
        let map = from_prolog_assoc(&term);
        assert_eq!(
            map,
            BTreeMap::from([
                ("a".to_string(), "b".to_string()),
                ("c".to_string(), "d".to_string()),
                ("e".to_string(), "f".to_string()),
                ("h".to_string(), "i".to_string()),
                ("j".to_string(), "k".to_string())
            ])
        );
    }
}

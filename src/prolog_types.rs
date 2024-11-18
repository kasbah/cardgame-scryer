use scryer_prolog::{
    Term,
    Term::{Atom, Compound},
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TermOrAssoc {
    Term(Term),
    Assoc(BTreeMap<String, TermOrAssoc>),
}

pub fn from_prolog_assoc(term: &Term) -> TermOrAssoc {
    match &term {
        Compound(str, args) if str == "t" => match &args[..] {
            [Atom(key), value, rest @ ..] => {
                let mut map = BTreeMap::new();
                let v = from_prolog_assoc(value);
                map.insert(key.clone(), v);
                match rest {
                    [Atom(s), terms @ ..] if s == "<" || s == "-" => {
                        for term in terms {
                            let mut result = from_prolog_assoc(term);
                            match &mut result {
                                TermOrAssoc::Assoc(m) => map.append(m),
                                _ => panic!("Unexpected from_prolog_assoc result: {:?}", result),
                            }
                        }
                        TermOrAssoc::Assoc(map)
                    }
                    _ => panic!("Unexpected rest args: {:?}", rest),
                }
            }
            _ => panic!("Unexpected args: {:?}", args),
        },
        Atom(str) if str == "t" => TermOrAssoc::Assoc(BTreeMap::new()),
        _ => TermOrAssoc::Term(term.clone()),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use dashu::Integer;
    use scryer_prolog::{LeafAnswer, Machine, MachineBuilder};

    fn query_once_binding(machine: &mut Machine, query: &str, var: &str) -> Term {
        let mut answers = machine.run_query(query);
        let answer = answers.next();
        match answer {
            Some(Ok(LeafAnswer::LeafAnswer { bindings, .. })) => match bindings.get(var) {
                Some(x) => {
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
        let result = from_prolog_assoc(&term);
        assert_eq!(result, TermOrAssoc::Assoc(BTreeMap::new()));
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
        let result = from_prolog_assoc(&term);
        assert_eq!(
            result,
            TermOrAssoc::Assoc(BTreeMap::from([(
                "a".to_string(),
                TermOrAssoc::Term(Atom("b".to_string()))
            )]))
        );
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
        let result = from_prolog_assoc(&term);
        assert_eq!(
            result,
            TermOrAssoc::Assoc(BTreeMap::from([
                ("a".to_string(), TermOrAssoc::Term(Atom("b".to_string()))),
                ("c".to_string(), TermOrAssoc::Term(Atom("d".to_string())))
            ]))
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
        let result = from_prolog_assoc(&term);
        assert_eq!(
            result,
            TermOrAssoc::Assoc(BTreeMap::from([
                ("a".to_string(), TermOrAssoc::Term(Atom("b".to_string()))),
                ("c".to_string(), TermOrAssoc::Term(Atom("d".to_string()))),
                ("e".to_string(), TermOrAssoc::Term(Atom("f".to_string())))
            ]))
        );
    }

    #[test]
    fn test_from_prolog_assoc_int() {
        let term = Compound(
            "t".to_string(),
            Vec::from([
                Atom("a".to_string()),
                Term::Integer(Integer::from(1)),
                Atom("-".to_string()),
                Atom("t".to_string()),
                Atom("t".to_string()),
            ]),
        );
        let result = from_prolog_assoc(&term);
        assert_eq!(
            result,
            TermOrAssoc::Assoc(BTreeMap::from([(
                "a".to_string(),
                TermOrAssoc::Term(Term::Integer(Integer::from(1)))
            )]))
        );
    }

    #[test]
    fn test_from_prolog_assoc_machine() {
        let mut machine = MachineBuilder::default().build();

        machine.load_module_string("test", r#":- use_module(library(assoc))."#);

        let query = r#"
            list_to_assoc([a-b, c-[d(x), e(x)], f-1, g-(h-i), j-k], X).
        "#;
        let term = query_once_binding(&mut machine, query, "X");
        let result = from_prolog_assoc(&term);
        assert_eq!(
            result,
            TermOrAssoc::Assoc(BTreeMap::from([
                ("a".to_string(), TermOrAssoc::Term(Atom("b".to_string()))),
                (
                    "c".to_string(),
                    TermOrAssoc::Term(Term::List(vec![
                        Compound("d".to_string(), vec![Atom("x".to_string())]),
                        Compound("e".to_string(), vec![Atom("x".to_string())])
                    ]))
                ),
                (
                    "f".to_string(),
                    TermOrAssoc::Term(Term::Integer(Integer::from(1)))
                ),
                (
                    "g".to_string(),
                    TermOrAssoc::Term(Compound(
                        "-".to_string(),
                        vec![Atom("h".to_string()), Atom("i".to_string())]
                    ))
                ),
                ("j".to_string(), TermOrAssoc::Term(Atom("k".to_string())))
            ]))
        );
    }
}

use scryer_prolog::{
    Term,
    Term::{Atom, Compound},
};
use std::collections::BTreeMap;

pub fn from_prolog_assoc(term: &Term) -> BTreeMap<String, Term> {
    match &term {
        Compound(str, args) if str == "t" => match &args[..] {
            [Atom(key), value, rest @ ..] => {
                let mut map = BTreeMap::new();
                map.insert(key.clone(), value.clone());
                match rest {
                    [Atom(s), terms @ ..] if s == "<" || s == "-" || s == ">" => {
                        for term in terms {
                            let mut m = from_prolog_assoc(term);
                            map.append(&mut m);
                        }
                        map
                    }
                    _ => panic!("Unexpected rest args: {:?}", rest),
                }
            }
            _ => panic!("Unexpected args: {:?}", args),
        },
        Atom(str) if str == "t" => BTreeMap::new(),
        _ => panic!("Unexpected term: {:?}", term),
    }
}

pub fn to_prolog(term: &Term) -> String {
    match term {
        Term::Integer(int) => int.to_string(),
        Term::Rational(rat) => rat.to_string(),
        Term::Float(float) => float.to_string(),
        Term::Atom(str) => str.clone(),
        Term::String(str) => format!(r#""{str}""#),
        Term::List(terms) => format!(
            "[{}]",
            &terms
                .iter()
                .map(to_prolog)
                .collect::<Vec<String>>()
                .join(", ")
        ),
        Term::Compound(name, args) => format!(
            "{}({})",
            name,
            &args
                .iter()
                .map(to_prolog)
                .collect::<Vec<String>>()
                .join(", ")
        ),
        Term::Var(var) => var.to_string(),
        _ => panic!("Unexpected term: {:?}", term),
    }
}

pub fn to_prolog_assoc(map: &BTreeMap<String, Term>, var: &str) -> String {
    let pairs = map
        .iter()
        .map(|(key, value)| format!("{key}-{}", to_prolog(value)))
        .collect::<Vec<String>>()
        .join(", ");
    let result = format!("list_to_assoc([{pairs}], {var})");
    result
}

#[derive(Debug, Clone, PartialEq)]
pub enum TermOrAssoc {
    Term(Term),
    Assoc(BTreeMap<String, TermOrAssoc>),
}

pub fn from_prolog_assoc_recursive(term: &Term) -> TermOrAssoc {
    match &term {
        Compound(str, args) if str == "t" => match &args[..] {
            [Atom(key), value, rest @ ..] => {
                let mut map = BTreeMap::new();
                let v = from_prolog_assoc_recursive(value);
                map.insert(key.clone(), v);
                match rest {
                    [Atom(s), terms @ ..] if s == "<" || s == "-" => {
                        for term in terms {
                            let mut result = from_prolog_assoc_recursive(term);
                            match &mut result {
                                TermOrAssoc::Assoc(m) => map.append(m),
                                _ => panic!(
                                    "Unexpected from_prolog_assoc_recursive result: {:?}",
                                    result
                                ),
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
    use scryer_prolog::{LeafAnswer, MachineBuilder};

    #[test]
    fn test_to_prolog1() {
        let term = Atom("hello".to_string());
        let result = to_prolog(&term);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_to_prolog2() {
        let term = Compound(
            "a".to_string(),
            vec![Atom("b".to_string()), Atom("c".to_string())],
        );
        let result = to_prolog(&term);
        assert_eq!(result, "a(b, c)");
    }

    #[test]
    fn test_to_prolog_int() {
        let term = Term::Integer(Integer::from(42));
        let result = to_prolog(&term);
        assert_eq!(result, "42");
    }

    #[test]
    fn test_to_prolog_var() {
        let term = Term::Var("X".to_string());
        let result = to_prolog(&term);
        assert_eq!(result, "X");
    }

    #[test]
    fn test_to_prolog_list() {
        let term = Term::List(vec![
            Term::Atom("a".to_string()),
            Term::Atom("b".to_string()),
        ]);
        let result = to_prolog(&term);
        assert_eq!(result, "[a, b]");
    }

    #[test]
    fn test_to_prolog_assoc1() {
        let map = BTreeMap::from([("a".to_string(), Atom("b".to_string()))]);
        let result = to_prolog_assoc(&map, "X");
        assert_eq!(result, "list_to_assoc([a-b], X)");
    }

    #[test]
    fn test_to_prolog_assoc2() {
        let map = BTreeMap::from([
            ("foo".to_string(), Atom("bar".to_string())),
            ("hello".to_string(), Term::Integer(42.into())),
            (
                "list".to_string(),
                Term::List(vec![Atom("a".to_string()), Atom("b".to_string())]),
            ),
        ]);
        let result = to_prolog_assoc(&map, "MyVar");
        assert_eq!(
            result,
            "list_to_assoc([foo-bar, hello-42, list-[a, b]], MyVar)"
        );
    }

    #[test]
    fn test_from_prolog_empty_assoc() {
        let term = Atom("t".to_string());
        let result = from_prolog_assoc(&term);
        assert_eq!(result, BTreeMap::new());
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
            BTreeMap::from([("a".to_string(), Atom("b".to_string()))])
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
            BTreeMap::from([
                ("a".to_string(), Atom("b".to_string())),
                ("c".to_string(), Atom("d".to_string()))
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
        let result = from_prolog_assoc(&term);
        assert_eq!(
            result,
            BTreeMap::from([
                ("a".to_string(), Atom("b".to_string())),
                ("c".to_string(), Atom("d".to_string())),
                ("e".to_string(), Atom("f".to_string()))
            ])
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
            BTreeMap::from([("a".to_string(), Term::Integer(Integer::from(1)))])
        );
    }

    #[test]
    fn test_from_prolog_assoc_machine() {
        let mut scryer = MachineBuilder::default().build();
        scryer.load_module_string("test", r#":- use_module(library(assoc))."#);

        let query = r#"
            list_to_assoc([a-b, c-[d(x), e(x)], f-1, g-(h-i), j-k], X).
        "#;
        let mut answers = scryer.run_query(query);
        let answer = answers.next().unwrap();
        let term = match answer {
            Ok(LeafAnswer::LeafAnswer { bindings, .. }) => bindings.get("X").cloned().unwrap(),
            _ => panic!("Unexpected answer: {:?}", answer),
        };
        let result = from_prolog_assoc(&term);
        assert_eq!(
            result,
            BTreeMap::from([
                ("a".to_string(), Atom("b".to_string())),
                (
                    "c".to_string(),
                    Term::List(vec![
                        Compound("d".to_string(), vec![Atom("x".to_string())]),
                        Compound("e".to_string(), vec![Atom("x".to_string())])
                    ])
                ),
                ("f".to_string(), Term::Integer(Integer::from(1))),
                (
                    "g".to_string(),
                    Compound(
                        "-".to_string(),
                        vec![Atom("h".to_string()), Atom("i".to_string())]
                    )
                ),
                ("j".to_string(), Atom("k".to_string()))
            ])
        );
    }

    #[test]
    fn test_from_prolog_empty_assoc_recursive() {
        let term = Atom("t".to_string());
        let result = from_prolog_assoc_recursive(&term);
        assert_eq!(result, TermOrAssoc::Assoc(BTreeMap::new()));
    }

    #[test]
    fn test_from_prolog_assoc1_recursive() {
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
        let result = from_prolog_assoc_recursive(&term);
        assert_eq!(
            result,
            TermOrAssoc::Assoc(BTreeMap::from([(
                "a".to_string(),
                TermOrAssoc::Term(Atom("b".to_string()))
            )]))
        );
    }

    #[test]
    fn test_from_prolog_assoc2_recursive() {
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
        let result = from_prolog_assoc_recursive(&term);
        assert_eq!(
            result,
            TermOrAssoc::Assoc(BTreeMap::from([
                ("a".to_string(), TermOrAssoc::Term(Atom("b".to_string()))),
                ("c".to_string(), TermOrAssoc::Term(Atom("d".to_string())))
            ]))
        );
    }

    #[test]
    fn test_from_prolog_assoc3_recursive() {
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
        let result = from_prolog_assoc_recursive(&term);
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
    fn test_from_prolog_assoc_int_recursive() {
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
        let result = from_prolog_assoc_recursive(&term);
        assert_eq!(
            result,
            TermOrAssoc::Assoc(BTreeMap::from([(
                "a".to_string(),
                TermOrAssoc::Term(Term::Integer(Integer::from(1)))
            )]))
        );
    }

    #[test]
    fn test_from_prolog_assoc_machine_recursive() {
        let mut scryer = MachineBuilder::default().build();

        scryer.load_module_string("test", r#":- use_module(library(assoc))."#);

        let query = r#"
            list_to_assoc([a-b, c-[d(x), e(x)], f-1, g-(h-i), j-k], X).
        "#;
        let mut answers = scryer.run_query(query);
        let answer = answers.next().unwrap();
        let term = match answer {
            Ok(LeafAnswer::LeafAnswer { bindings, .. }) => bindings.get("X").cloned().unwrap(),
            _ => panic!("Unexpected answer: {:?}", answer),
        };
        let result = from_prolog_assoc_recursive(&term);
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

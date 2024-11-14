use scryer_prolog::MachineBuilder;
use scryer_prolog::{LeafAnswer, Term};

fn main() {
    let mut machine = MachineBuilder::default().build();

    machine.load_module_string(
        "facts",
        String::from(
            r#"
            triple("a", "p1", "b").
            triple("a", "p2", "b").
            "#,
        ),
    );

    let query = r#"triple("a",P,"b")."#;
    let complete_answer: Vec<_> = machine.run_query(query).collect::<Result<_, _>>().unwrap();

    assert_eq!(
        complete_answer,
        [
            LeafAnswer::from_bindings([("P", Term::string("p1")),]),
            LeafAnswer::from_bindings([("P", Term::string("p2")),]),
        ],
    );

    let query = r#"triple("a","p1","b")."#;
    let complete_answer: Vec<_> = machine.run_query(query).collect::<Result<_, _>>().unwrap();

    assert_eq!(complete_answer, [LeafAnswer::True],);

    let query = r#"triple("x","y","z")."#;
    let complete_answer: Vec<_> = machine.run_query(query).collect::<Result<_, _>>().unwrap();

    assert_eq!(complete_answer, [LeafAnswer::False],);
}

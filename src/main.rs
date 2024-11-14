use scryer_prolog::{LeafAnswer, MachineBuilder, Term};
mod prolog_types;

fn main() {
    let mut machine = MachineBuilder::default().build();

    let file_content = include_str!("logic.pl");

    machine.load_module_string("logic", file_content);

    let query = r#"
        init(State), random_options(State, StateOut).
    "#;

    let answers = machine.run_query(query);

    for answer in answers {
        match answer {
            Ok(LeafAnswer::LeafAnswer { bindings, .. }) => match bindings.get("StateOut") {
                Some(term) => {
                    prolog_types::from_prolog(term);
                }
                _ => panic!("Unexpected bindings: {:?}", bindings),
            },
            Ok(LeafAnswer::False) => {}
            _ => panic!("Unexpected answer: {:?}", answer),
        }
    }
}

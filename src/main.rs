use scryer_prolog::{LeafAnswer, MachineBuilder, Term};

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
                Some(Term::Compound(str, _args)) if str == "t" => {
                    println!("its a t!")
                }
                _ => panic!("Unexpected bindings: {:?}", bindings),
            },
            Ok(LeafAnswer::False) => {}
            _ => panic!("Unexpected answer: {:?}", answer),
        }
    }
}

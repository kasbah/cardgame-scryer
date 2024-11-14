use scryer_prolog::MachineBuilder;
use scryer_prolog::LeafAnswer;

fn main() {
    let mut machine = MachineBuilder::default().build();

    let file_content = include_str!("logic.pl");

    machine.load_module_string("logic", file_content);

    //let query = r#"
    //    once(init(State)).
    //"#;
    //let answers = machine.run_query(query);

    //for answer in answers {
    //    println!("{:?}", answer);
    //}

    let query2 = r#"
        init(State), random_options(State, StateOut).
    "#;

    let answers = machine.run_query(query2);

    for answer in answers {
        match answer {
            Ok(LeafAnswer::LeafAnswer { bindings, .. }) => {
                println!("{:?}", bindings.get("StateOut"));
            }
            Ok(LeafAnswer::False) => {}
            _ => panic!("Unexpected answer: {:?}", answer),
        }
    }
}

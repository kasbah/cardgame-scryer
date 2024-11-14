use scryer_prolog::MachineBuilder;
//use scryer_prolog::{LeafAnswer, Term};

fn main() {
    let mut machine = MachineBuilder::default().build();

    let file_content = include_str!("logic.pl");

    machine.load_module_string("logic", file_content);

    let query = r#"
        card(Id, Distance, Temp, OrbitTime, Radius, Mass, EarthSimilarity),
        X = card(Id, Distance, Temp, OrbitTime, Radius, Mass, EarthSimilarity).
    "#;
    let answers = machine.run_query(query);

    for answer in answers {
        println!("{:?}", answer);
    }
}


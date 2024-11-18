use rand::Rng;

pub fn random_choice<T>(options: &Vec<T>) -> usize {
    rand::thread_rng().gen_range(0..options.len())
}

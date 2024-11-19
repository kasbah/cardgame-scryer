use rand::Rng;

pub fn random_choice<T>(options: &[T]) -> usize {
    rand::thread_rng().gen_range(0..options.len())
}

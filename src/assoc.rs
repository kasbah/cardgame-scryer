#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn assoc_test() {
        let mut map = std::collections::HashMap::new();
        map.insert(1, 2);
        map.insert(3, 4);
        map.insert(5, 6);
        assert_eq!(map.get(&1), Some(&2));
        assert_eq!(map.get(&3), Some(&4));
        assert_eq!(map.get(&5), Some(&6));
    }
}

use std::{collections::HashMap, hash::Hash};

// Extends a hashmap with another hashmap without overwriting the first hashmap's existing values.
pub fn extend_no_overwrite<K, V>(
    mut hash_map_one: HashMap<K, V>,
    hash_map_two: HashMap<K, V>,
) -> HashMap<K, V>
where
    K: Eq + Hash,
{
    for (key, value) in hash_map_two {
        hash_map_one.entry(key).or_insert(value);
    }
    hash_map_one
}

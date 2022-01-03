use std::collections::HashMap;
use std::hash::Hash;

pub(crate) trait GetOrInsert<K, V> {
    fn remove_or_default(&mut self, key: &K) -> V;
}

impl<K, V> GetOrInsert<K, V> for HashMap<K, V>
where
    K: Hash + Eq,
    V: Default,
{
    fn remove_or_default(&mut self, key: &K) -> V {
        self.remove(key).unwrap_or_default()
    }
}

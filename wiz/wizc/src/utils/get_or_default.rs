use std::collections::HashMap;
use std::hash::Hash;

pub(crate) trait GetOrDefault<K, V> {
    fn get_or_default(&mut self, key: &K) -> &V;
    fn get_or_default_mut(&mut self, key: &K) -> &mut V;
    fn remove_or_default(&mut self, key: &K) -> V;
}

impl<K, V> GetOrDefault<K, V> for HashMap<K, V>
where
    K: Hash + Eq + Clone,
    V: Default,
{
    fn get_or_default(&mut self, key: &K) -> &V {
        if self.contains_key(key) {
            self.get(key).unwrap()
        } else {
            self.insert(key.clone(), V::default());
            self.get(key).unwrap()
        }
    }

    fn get_or_default_mut(&mut self, key: &K) -> &mut V {
        if self.contains_key(key) {
            self.get_mut(key).unwrap()
        } else {
            self.insert(key.clone(), V::default());
            self.get_mut(key).unwrap()
        }
    }

    fn remove_or_default(&mut self, key: &K) -> V {
        self.remove(key).unwrap_or_default()
    }
}

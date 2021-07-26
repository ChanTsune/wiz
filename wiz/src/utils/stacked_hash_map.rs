use std::collections::HashMap;
use std::hash::Hash;
use std::collections::hash_map::RandomState;
use std::fmt;

#[derive(fmt::Debug, Eq, PartialEq, Clone)]
pub(crate) struct StackedHashMap<K, V> {
    map_stack: Vec<HashMap<K, V>>
}

impl <K, V> StackedHashMap<K, V>
    where
        K: Hash + Eq
{
    pub(crate) fn new() -> Self {
        StackedHashMap {
            map_stack: vec![]
        }
    }

    pub(crate) fn from(m: HashMap<K,V>) -> Self {
        StackedHashMap {
            map_stack: vec![m]
        }
    }

    pub(crate) fn push(&mut self, m: HashMap<K, V>) {
        self.map_stack.push(m);
    }

    pub(crate) fn pop(&mut self) -> Option<HashMap<K, V>> {
        self.map_stack.pop()
    }

    pub(crate) fn insert(&mut self, k:K, v:V) -> Option<V> {
        let last_index = self.map_stack.len() -1;
        self.map_stack[last_index].insert(k,v)
    }

    pub(crate) fn get(&self, k:&K) -> Option<&V> {
        for env in self.map_stack.iter().rev() {
            if let Some(t) = env.get(k) {
                return Some(t);
            }
        }
        None
    }
}

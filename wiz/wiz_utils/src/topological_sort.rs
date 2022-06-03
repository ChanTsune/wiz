use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CircularDependencyError;

impl CircularDependencyError {
    pub fn new() -> Self {
        Self
    }
}

impl Display for CircularDependencyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl Error for CircularDependencyError {}

pub fn topological_sort<T>(
    data: HashMap<T, HashSet<T>>,
) -> Result<Vec<HashSet<T>>, CircularDependencyError>
where
    T: Eq + Hash + Clone,
{
    if data.is_empty() {
        return Ok(vec![]);
    }
    // Copy the input so as to leave it unmodified.
    // Discard self-dependencies and copy two levels deep.
    let mut data = data
        .into_iter()
        .map(|(item, deps)| {
            let deps = deps
                .into_iter()
                .filter(|e| !item.eq(e))
                .collect::<HashSet<_>>();
            (item, deps)
        })
        .collect::<HashMap<_, _>>();

    // Find all items that don't depend on anything.

    let t = data
        .values()
        .cloned()
        .reduce(|l, r| l.union(&r).cloned().collect::<HashSet<_>>())
        .unwrap_or_default();

    let v = data.keys().cloned().collect::<HashSet<_>>();

    let extra_items_in_deps = t.difference(&v).collect::<HashSet<_>>();

    // Add empty dependencies where needed.
    data.extend(
        extra_items_in_deps
            .into_iter()
            .map(|item| (item.clone(), HashSet::new())),
    );

    let mut result = vec![];

    loop {
        let ordered = data
            .iter()
            .filter(|(_, deps)| deps.is_empty())
            .map(|(item, _)| item)
            .cloned()
            .collect::<HashSet<_>>();
        if ordered.is_empty() {
            break;
        }

        data = data
            .into_iter()
            .filter(|(item, _)| !ordered.contains(item))
            .map(|(item, dep)| {
                (
                    item,
                    dep.difference(&ordered).cloned().collect::<HashSet<_>>(),
                )
            })
            .collect();

        result.push(ordered);
    }

    if data.is_empty() {
        Ok(result)
    } else {
        Err(CircularDependencyError::new())
    }
}

#[cfg(test)]
mod tests {
    use super::{topological_sort, CircularDependencyError};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn empty() {
        assert_eq!(topological_sort(HashMap::<&str, _>::new()), Ok(vec![]))
    }

    #[test]
    fn no_dependencies() {
        let data = HashMap::from([("a", HashSet::new())]);
        assert_eq!(topological_sort(data), Ok(vec![HashSet::from(["a"])]))
    }

    #[test]
    fn one_dependencies() {
        let data = HashMap::from([("a", HashSet::from(["b"])), ("b", HashSet::new())]);
        assert_eq!(
            topological_sort(data),
            Ok(vec![HashSet::from(["b"]), HashSet::from(["a"])])
        )
    }

    #[test]
    fn two_dependencies() {
        let data = HashMap::from([
            ("a", HashSet::from(["b", "c"])),
            ("b", HashSet::new()),
            ("c", HashSet::new()),
        ]);
        assert_eq!(
            topological_sort(data),
            Ok(vec![HashSet::from(["b", "c"]), HashSet::from(["a"])])
        )
    }

    #[test]
    fn many_dependencies() {
        let data = HashMap::from([
            (2, HashSet::from([11])),
            (9, HashSet::from([11, 8, 10])),
            (10, HashSet::from([11, 3])),
            (11, HashSet::from([7, 5])),
            (8, HashSet::from([7, 3])),
        ]);
        assert_eq!(
            topological_sort(data),
            Ok(vec![
                HashSet::from([3, 5, 7]),
                HashSet::from([8, 11]),
                HashSet::from([2, 10]),
                HashSet::from([9])
            ])
        );
    }

    #[test]
    fn circular_dependency_each() {
        let data = HashMap::from([("a", HashSet::from(["b"])), ("b", HashSet::from(["a"]))]);
        assert_eq!(topological_sort(data), Err(CircularDependencyError::new()))
    }

    #[test]
    fn circular_dependency_triple() {
        let data = HashMap::from([
            ("a", HashSet::from(["b"])),
            ("b", HashSet::from(["c"])),
            ("c", HashSet::from(["a"])),
        ]);
        assert_eq!(topological_sort(data), Err(CircularDependencyError::new()))
    }
}

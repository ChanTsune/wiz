mod ext;
mod stacked_hash_map;

pub use ext::string::StringExt;
pub use stacked_hash_map::StackedHashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

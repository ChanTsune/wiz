
pub(crate) trait StringExt {
    fn remove_first(&self) -> String;
}

impl StringExt for String {
    fn remove_first(&self) -> String {
        let mut chars = self.chars();
        chars.next();
        String::from(chars.as_str())
    }
}

pub(crate) trait StringExt {
    fn remove_first(&self) -> String;
    fn remove_last(&self) -> String;
}

impl StringExt for String {
    fn remove_first(&self) -> String {
        let mut chars = self.chars();
        chars.next();
        String::from(chars.as_str())
    }

    fn remove_last(&self) -> String {
        let mut chars = self.chars();
        chars.next_back();
        String::from(chars.as_str())
    }
}

pub(crate) trait StringExt {
    fn remove_first(&self) -> Self;
    fn remove_last(&self) -> Self;
}

impl StringExt for String {

    fn remove_first(&self) -> Self {
        let str: &str = self;
        Self::from(str.remove_first())
    }

    fn remove_last(&self) -> Self {
        let str: &str = self;
        Self::from(str.remove_last())
    }
}

impl StringExt for &str {
    fn remove_first(&self) -> Self {
        let mut chars = self.chars();
        chars.next();
        chars.as_str()
    }

    fn remove_last(&self) -> Self {
        let mut chars = self.chars();
        chars.next_back();
        chars.as_str()
    }
}

#[cfg(test)]
mod tests {
    use crate::ext::string::StringExt;

    #[test]
    fn test_remove_first() {
        assert_eq!("123456789", "0123456789".remove_first());
        assert_eq!(String::from("123456789"), String::from("0123456789").remove_first());
    }

    #[test]
    fn test_remove_last() {
        assert_eq!("012345678", "0123456789".remove_last());
        assert_eq!(String::from("012345678"), String::from("0123456789").remove_last());
    }
}
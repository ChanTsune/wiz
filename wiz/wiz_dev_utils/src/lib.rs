trait StringExt where Self: ToString {
    fn trim_indent(&self) -> String;
    fn trim_margin<T: ToString>(&self, margin_prefix: T) -> String;
    fn indent_count<T: ToString>(&self, indent_prefix: T) -> usize;
}

impl StringExt for &str {
    fn trim_indent(&self) -> String {
        todo!()
    }

    fn trim_margin<T: ToString>(&self, margin_prefix: T) -> String {
        self.split_terminator('\n')
            .filter(|i| !i.is_empty())
            .filter_map(|i| {
                let f = i.find(margin_prefix.to_string().as_str())? + 1;
                let (_, r) = i.split_at(f);
                Some(format!("{}\n", r))
            })
            .collect()
    }

    fn indent_count<T: ToString>(&self, indent_prefix: T) -> usize {
        let indent_prefix = indent_prefix.to_string();
        let mut self_ = self.to_string();
        let mut count = 0;
        while self_.starts_with(indent_prefix.as_str()) {
            self_ = self_.as_str()[indent_prefix.len()..].to_string();
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use crate::StringExt;

    #[test]
    fn test_trim_indent() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_trim_margin() {
        assert_eq!(
            r"
        |fun add(x: i32, y: y: i32): i32 {
        |  return x + y
        |}
        "
            .trim_margin('|'),
            "fun add(x: i32, y: y: i32): i32 {\n  return x + y\n}\n"
        );
    }

    #[test]
    fn test_indent_count() {
        assert_eq!("    ".indent_count(' '), 4);
        assert_eq!("    ".indent_count("    "), 1);
    }
}

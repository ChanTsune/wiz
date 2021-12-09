trait StringExt {
    fn trim_indent(&self) -> String;
    fn trim_margin<T: ToString>(&self, margin_prefix: T) -> String;
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
}

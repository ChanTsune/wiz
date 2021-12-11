use std::cmp::min;

pub trait StringExt
where
    Self: ToString,
{
    fn trim_indent(&self) -> String;
    fn trim_margin<T: ToString>(&self, margin_prefix: T) -> String;
    fn indent_count<T: ToString>(&self, indent_prefix: T) -> usize;
}

impl StringExt for &str {
    fn trim_indent(&self) -> String {
        let i = self.split_terminator('\n').filter(|i| !i.is_empty());
        let indent_width = i
            .clone()
            .filter_map(|i| match i.indent_count(' ') {
                a if a == i.len() => None,
                a => Some(a),
            })
            .min()
            .unwrap_or_default();
        i.map(|i| {
            let (_, r) = i.split_at(min(indent_width, i.len()));
            if r.is_empty() {
                String::new()
            } else {
                format!("{}\n", r)
            }
        })
        .collect()
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
        assert_eq!(
            r"
        fun add(x: i32, y: y: i32): i32 {
          return x + y
        }"
            .trim_indent(),
            "fun add(x: i32, y: y: i32): i32 {\n  return x + y\n}\n"
        );
        assert_eq!(r"
        Token { kind: Whitespace, len: 1 }
        Token { kind: Literal { kind: Char { terminated: true }, suffix_start: 3 }, len: 3 }".trim_indent(), "Token { kind: Whitespace, len: 1 }\nToken { kind: Literal { kind: Char { terminated: true }, suffix_start: 3 }, len: 3 }\n")
    }

    #[test]
    fn test_trim_margin() {
        assert_eq!(
            r"
        |fun add(x: i32, y: y: i32): i32 {
        |  return x + y
        |}"
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

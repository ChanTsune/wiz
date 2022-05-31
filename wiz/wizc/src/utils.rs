use std::path::Path;

pub(crate) fn path_string_to_page_name(path: &str) -> &str {
    Path::new(path)
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::path_string_to_page_name;

    #[test]
    fn test_path_string_to_page_name() {
        assert_eq!(path_string_to_page_name("../main.wiz"), "main");
        assert_eq!(path_string_to_page_name("main.wiz"), "main");
        assert_eq!(path_string_to_page_name("main"), "main");
        assert_eq!(path_string_to_page_name(""), "");
    }
}

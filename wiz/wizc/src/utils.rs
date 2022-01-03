use std::path::Path;

pub(crate) mod get_or_default;
pub(crate) mod stacked_hash_map;

pub(crate) fn path_string_to_page_name(path: String) -> String {
    let path = Path::new(&path);
    path.file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils::path_string_to_page_name;

    #[test]
    fn test_path_string_to_page_name() {
        assert_eq!(
            path_string_to_page_name(String::from("../main.wiz")),
            String::from("main")
        );
        assert_eq!(
            path_string_to_page_name(String::from("main.wiz")),
            String::from("main")
        );
        assert_eq!(
            path_string_to_page_name(String::from("main")),
            String::from("main")
        );
        assert_eq!(path_string_to_page_name(String::new()), String::new());
    }
}

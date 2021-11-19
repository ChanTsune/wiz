pub mod builder;
pub mod expr;
pub mod format;
pub mod ml_decl;
pub mod ml_file;
pub mod ml_node;
pub mod ml_type;
pub mod statement;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

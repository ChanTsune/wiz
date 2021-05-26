mod ast;
mod parser;


fn main() {
    println!("Hello, world!");
    let file_node = ast::file::File{body: Vec::new()};
    println!("{:?}", file_node);
}

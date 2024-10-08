mod parser;

fn main() {
    println!("Hello, world!");
    let body = "";
    let input = parser::Span::new_extra(body, "filename");
    parser::parse(input).unwrap();
}

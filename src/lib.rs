mod parser;
mod types;



pub fn assemble(source: &str) -> String {
    let (parse_stream, labels) = parser::parse_source(source);
    todo!()
}

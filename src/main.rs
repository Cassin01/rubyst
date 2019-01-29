mod tree;
mod is;
mod interpreter;
mod read_code;
mod parser;

fn main() {
    let code = read_code::read_code();
    let ast = parser::parser(code);
    println!("{:?}", ast);

    println!("{}", interpreter::evaluate(ast));
}
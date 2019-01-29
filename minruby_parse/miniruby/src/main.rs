mod parser;
use parser::parser;
mod tree;
mod is;
mod interpreter;
mod read_code;

fn main() {
    let code = read_code::read_code();

    let ast = parser(code);

    println!("{:?}", ast);

    println!("{}", interpreter::evaluate(ast));
}
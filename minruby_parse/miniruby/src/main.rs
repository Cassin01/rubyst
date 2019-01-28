mod parser;
use parser::parser;
mod tree;
mod is;
mod interpreter;

fn main() {
    let code = String::from("(6 + 7) * 5 * (2 + 3)");
    let ast = parser(code);

    println!("{:?}", ast);

    println!("{}", interpreter::evaluate(ast));
}
mod tree;
mod is;
mod interpreter;
mod read_code;

mod parser;
use parser::parser;

fn main() {
    let ast = parser(String::from(" 1 + 15 % (2 ** 3)"));
    println!("{:?}", ast);

    println!("{}", interpreter::evaluate(ast));
}

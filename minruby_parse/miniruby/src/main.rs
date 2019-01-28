mod parser;
use parser::parser;
mod tree;
mod is;

fn main() {
    let code = String::from("5 * (2 + 3)");
    let ast = parser(code);

    println!("{:?}", ast);
}
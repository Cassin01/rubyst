mod tree;
mod is;
mod interpreter;
mod read_code;
mod parser;
mod stmts;

fn main() {
    let code = read_code::read_code();
    println!("{}", code);
    let ast = parser::parser(code);
    println!("{:?}", ast);

    println!("{:?}", interpreter::evaluate(ast));
}
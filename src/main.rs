mod tree;
mod is;
mod interpreter;
mod read_code;
mod parser;

fn main() {
    let code = read_code::read_code();
    let ast = parser::parser(code.chars().peekable());
    //println!("{:?}", ast);

    interpreter::evaluate(ast);
}
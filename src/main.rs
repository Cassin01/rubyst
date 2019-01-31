mod tree;
mod is;
mod interpreter;
mod read_code;
mod parser;
mod stmts;

fn main() {
    let code = read_code::read_code();
    let ast = parser::parser(code);

    interpreter::evaluate(ast);
}
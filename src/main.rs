mod tree;
mod is;
mod interpreter;
mod read_code;
mod parser;

fn main() {
    let code = read_code::read_code();
    let ast = parser::parser(code.chars().peekable());
    //read_tree(&ast, 0, 2, String::from("H"));
    interpreter::interpret(ast);
}

/*
fn read_tree(tree: &tree::Tree, u: usize, indent: usize, lr: String) {
    println!("{}{} {:?}", " ".repeat(u) ,lr, tree.root);
    if let Some(ref x) = tree.left {
        read_tree(x, u + indent, indent, String::from("L"));
    }
    if let Some(ref x) = tree.right {
        read_tree(x, u + indent, indent, String::from("R"));
    }
}
*/
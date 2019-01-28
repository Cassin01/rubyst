use super::tree::Tree;
use super::tree::Op;

fn is_num(c: &char) -> bool {
    match c {
        '1' => true,
        '2' => true,
        '3' => true,
        '4' => true,
        '5' => true,
        '6' => true,
        '7' => true,
        '8' => true,
        '9' => true,
        _   => false
    }
}

fn is_space(c: &char) -> bool {
    match c {
        ' ' => true,
        _   => false,
    }
}

fn is_operator_sums(c: &char) -> bool {
    match c {
        '+' => true,
        '-' => true,
        _   => false,
    }
}

fn is_operator_products(c: &char) -> bool {
    match c {
        '*' => true,
        '/' => true,
        _   => false,
    }
}

fn enum_op(op: String) -> Op<String> {
    match op.as_str() {
        "*" => Op::Mult,
        "/" => Op::Divid,
        "+" => Op::Plus,
        "-" => Op::Minus,
        _   => panic!("not operator"),
    }
}


fn push_num(mut tree: Tree, num: String) -> Tree {
    if tree.root == Op::Nil {
        tree.left(Some(Box::new(Tree::new(Op::Lit(num)))))
    } else {
        tree.push_back(num);
        tree
    }
}

fn push_op(tree: Tree, op: String) -> Tree {
    if tree.root == Op::Nil {
        tree.root(op)
    } else {
        Tree::new(enum_op(op)).left(Some(Box::new(tree)))
    }
}

fn push_op_products(tree: Tree, op: String)
    -> Tree {
    match tree.root {
        Op::Nil => tree.root(op),
        Op::Plus | Op::Minus =>
            Tree::new(tree.root).left(tree.left).right(Some(Box::new(Tree::new(enum_op(op)).left(tree.right)))),
        Op::Mult | Op::Divid =>
            Tree::new(enum_op(op)).left(Some(Box::new(tree))),
        _ => panic!("not operator"),
    }
}


pub fn parser(code: String) -> Tree {
    let mut num = String::new();
    let mut num_flag = false;

    // Abstract syntax tree
    let mut ast = Tree::new(Op::Nil);

    for c in code.chars() {
        if is_num(&c) {
            num_flag = true;
            num.push(c);
        } else if is_space(&c) {
            if num_flag {
                ast = push_num(ast, num.clone());
                num.clear();
                num_flag = false;
            }
        } else if is_operator_sums(&c) {
            num_flag = false;
            ast = push_op(ast, c.to_string());
        } else if is_operator_products(&c) {
            num_flag = false;
            ast = push_op_products(ast, c.to_string());
        } else {
            panic!("err {} is not supported", c);
        }
    }
    if num_flag {
        ast = push_num(ast, num.clone());
        num.clear();
    }
    ast
}

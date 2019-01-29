use super::tree::Tree;
use super::tree::Op;
use super::is;

fn enum_op(op: String) -> Op<String> {
    match op.as_str() {
        "*" => Op::Mul,
        "/" => Op::Div,
        "+" => Op::Add,
        "-" => Op::Neg,
        "%" => Op::Rem,
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

fn push_tree(mut tree: Tree, insert_tree: Tree) -> Tree {
    if tree.root == Op::Nil {
        tree.left(Some(Box::new(insert_tree)))
    } else {
        tree.push_back_tree(insert_tree);
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
        Op::Add | Op::Neg =>
            Tree::new(tree.root).left(tree.left).right(Some(Box::new(Tree::new(enum_op(op)).left(tree.right)))),
        Op::Mul | Op::Div | Op::Rem =>
            Tree::new(enum_op(op)).left(Some(Box::new(tree))),
        Op::Lit(_) => panic!("not operator"),
    }
}

pub fn parser(code: String) -> Tree {
    let mut num = String::new();
    let mut code_in_brackets = String::new();
    let mut num_flag = false;
    let mut not_brackets = true;

    // Abstract syntax tree
    let mut ast = Tree::new(Op::Nil);

    for c in code.chars() {
        if is::is_num(&c) && not_brackets {
            num_flag = true;
            num.push(c);
        } else if is::is_space(&c) && not_brackets {
            if num_flag {
                ast = push_num(ast, num.clone());
                num.clear();
                num_flag = false;
            }
        } else if is::is_operator_sums(&c) && not_brackets{
            num_flag = false;
            ast = push_op(ast, c.to_string());
        } else if is::is_operator_products(&c) && not_brackets {
            num_flag = false;
            ast = push_op_products(ast, c.to_string());
        } else if is::is_first_bracket(&c) && not_brackets {
            not_brackets = false;
        } else if is::is_second_bracket(&c) {
            ast = push_tree(ast, parser(code_in_brackets.clone()));
            code_in_brackets.clear();
            not_brackets = true;
        } else if !not_brackets {
            code_in_brackets.push(c);
        } else {
        }
    }
    if num_flag {
        ast = push_num(ast, num.clone());
        num.clear();
    }
    ast
}
use super::tree::Tree;
use super::tree::Op;
use super::is;
use std::iter::Peekable;
use std::str::Chars;


fn push_op(tree: Tree, op: String) -> Tree {
    if tree.root == Op::Nil {
        tree.root(op)
    } else {
        Tree::new(enum_op(op)).left(Some(Box::new(tree)))
    }
}

fn enum_op(op: String) -> Op<String> {
    match op.as_str() {
        "*" => Op::Mul,
        "/" => Op::Div,
        "+" => Op::Add,
        "-" => Op::Neg,
        "%" => Op::Rem,
        "**" => Op::Pow,
        _   => panic!("not operator"),
    }
}

fn push_op_products(tree: Tree, op: String)
    -> Tree {
    match tree.root {
        Op::Nil => tree.root(op),
        Op::Add | Op::Neg =>
            Tree::new(tree.root).left(tree.left).right(Some(Box::new(Tree::new(enum_op(op)).left(tree.right)))),
        Op::Mul | Op::Div | Op::Rem | Op::Pow =>
            Tree::new(enum_op(op)).left(Some(Box::new(tree))),
        Op::Lit(_) => panic!("not operator"),
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

fn is_operator_sums(s: &String) -> bool {
    match s.as_str() {
        "+" => true,
        "-" => true,
        _   => false,
    }
}

fn is_operator_puroducts(s: &String) -> bool {
    match s.as_str() {
        "*" => true,
        "/" => true,
        "%" => true,
        "**" => true,
        _   => false,
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

pub fn parser(code: String) -> Tree {
    let mut num = String::new();
    let mut op = String::new();
    let mut code_in_brackets = String::new();

    // Abstract syntax tree
    let mut ast = Tree::new(Op::Nil);

    let mut cs = code.chars().peekable();


    loop {
        match cs.peek() {
            Some(_) => (),
            None => break,
        }

        // 数字
        if is_this(&mut cs, &is::is_num) {
            while is_this(&mut cs, &is::is_num) {
                if let Some(c) = cs.next() {
                    num.push(c);
                } else {
                    break;
                }
            }
            ast = push_num(ast, num.clone());
            num.clear();
        }

        // スペース
        else if is_this(&mut cs, &is::is_space) {
            while is_this(&mut cs, &is::is_space) {
                if let Some(_) = cs.next() {
                } else {
                    break;
                }
            }
        }

        // 演算子
        else if is_this(&mut cs, &is::is_operator) {
            while is_this(&mut cs, &is::is_operator) {
                if let Some(c) = cs.next() {
                    op.push(c);
                } else {
                    break;
                }
            }
            if is_operator_sums(&op) {
                ast = push_op(ast, op.clone());
            } else if is_operator_puroducts(&op) {
                ast = push_op_products(ast, op.clone());
            }
            op.clear();
        }

        // 括弧
        else if is_this(&mut cs, &is::is_first_bracket) {
            cs.next();
            while !is_this(&mut cs, &is::is_second_bracket) {
                if let Some(c) = cs.next() {
                    code_in_brackets.push(c);
                } else {
                    break;
                }
            }
            cs.next();
            ast = push_tree(ast, parser(code_in_brackets.clone()));
            code_in_brackets.clear();
        }
    }
    ast
}

fn is_this(cs: &mut Peekable<Chars>, f: &Fn(&char)->bool) -> bool {
    match cs.peek() {
        Some(c) => f(&c),
        None => false,
    }
}
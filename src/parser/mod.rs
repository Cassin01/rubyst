use super::tree::Tree;
use super::tree::Op;
use super::is;
use std::iter::Peekable;
use std::str::Chars;
use super::tree::PushBack;

fn push_stmt(tree: Tree, insert_tree: Tree) -> Tree {
    Tree::new(Op::STMT(Box::new(tree))).left(Some(Box::new(insert_tree)))
}

fn push_fun(tree: Tree, fnc: String, insert_tree: Tree) -> Tree {
    if tree.root == Op::Nil {
        tree.root(Op::Fun(fnc)).left(Some(Box::new(insert_tree)))
    } else {
        Tree::new(Op::Fun(fnc)).left(Some(Box::new(tree))).left(Some(Box::new(insert_tree)))
    }
}

fn push_op_asi(tree: Tree, op: Op<String>) -> Tree {
    if let Op::Asi = op {
        match tree.root {
            Op::Nil => tree.root(op),
            Op::Asi => Tree::new(tree.root)
                            .left(tree.left)
                            .right(Some(Box::new(
                                Tree::new(Op::Asi)
                                    .left(tree.right)))),
            Op::Val(_) | Op::Lit(_) => panic!("not operator"),
            _       => Tree::new(Op::Asi)
                            .left(Some(Box::new(tree)))
        }
    } else {
        panic!("not assignment operation");
    }
}

fn push_op_eqls(tree: Tree, op: String) -> Tree {
    match tree.root {
        Op::Nil => tree.root(Tree::enum_op(op)),
        Op::Asi => Tree::new(tree.root)
                        .left(tree.left)
                        .right(Some(Box::new(
                            Tree::new(Tree::enum_op(op))
                                .left(tree.right)))),
        Op::Val(_) | Op::Lit(_) => panic!("not operator"),
        _       => Tree::new(Tree::enum_op(op))
                        .left(Some(Box::new(tree)))
    }
}

fn push_op_sums(tree: Tree, op: String)
    -> Tree {
        match tree.root {
            Op::Nil => tree.root(Tree::enum_op(op)),
            Op::Asi | Op::STMT(_) | Op::Fun(_) | Op::ROp(_) =>
                Tree::new(tree.root).left(tree.left).right(Some(Box::new(Tree::new(Tree::enum_op(op)).left(tree.right)))),
            Op::Add | Op::Neg | Op::Mul | Op::Div | Op::Rem | Op::Pow =>
                Tree::new(Tree::enum_op(op)).left(Some(Box::new(tree))),
            Op::Val(_) | Op::Lit(_) => panic!("not operator"),
        }
}

fn push_op_products(tree: Tree, op: String)
    -> Tree {
    match tree.root {
        Op::Nil => tree.root(Tree::enum_op(op)),
        Op::Asi | Op::STMT(_) | Op::Fun(_) | Op::ROp(_) | Op::Add | Op::Neg =>
            Tree::new(tree.root).left(tree.left).right(Some(Box::new(Tree::new(Tree::enum_op(op)).left(tree.right)))),
        Op::Mul | Op::Div | Op::Rem | Op::Pow =>
            Tree::new(Tree::enum_op(op)).left(Some(Box::new(tree))),
        Op::Val(_) | Op::Lit(_) => panic!("not operator"),
    }
}

fn push_op_pows(tree: Tree, op: String)
    -> Tree {
        match tree.root {
            Op::Nil => tree.root(Tree::enum_op(op)),
            Op::Asi | Op::STMT(_) | Op::Fun(_) | Op::ROp(_) | Op::Add |
            Op::Neg | Op::Mul | Op::Div | Op::Rem =>
                Tree::new(tree.root)
                    .left(tree.left)
                    .right(Some(Box::new(Tree::new(Tree::enum_op(op)).left(tree.right)))),
            Op::Pow =>
                Tree::new(Tree::enum_op(op))
                .left(Some(Box::new(tree))),
            Op::Val(_) | Op::Lit(_) => panic!("not operator"),
        }
}

fn push_num(mut tree: Tree, num: String) -> Tree {
    if tree.root == Op::Nil {
        tree.left(Some(Box::new(Tree::new(Op::Lit(num)))))
    } else {
        //tree.push_back_op(Op::Lit(num));
        tree.push_back(Op::Lit(num));
        tree
    }
}

fn push_val(mut tree: Tree, val: String) -> Tree {
    if tree.root == Op::Nil {
        tree.left(Some(Box::new(Tree::new(Op::Val(val)))))
    } else {
        tree.push_back(Op::Val(val));
        tree
    }
}

fn push_tree(mut tree: Tree, insert_tree: Tree) -> Tree {
    if tree.root == Op::Nil {
        tree.left(Some(Box::new(insert_tree)))
    } else {
        tree.push_back(insert_tree);
        tree
    }
}

pub fn parser(mut cs: Peekable<Chars>) -> Tree {
    let mut num = String::new();
    let mut op = String::new();
    let mut ob = String::new();
    let mut code_in_brackets = String::new();

    // Abstract syntax tree
    let mut ast = Tree::new(Op::Nil);

    loop {
        match cs.peek() {
            Some(_) => (),
            None => break,
        }

        // 複文
        if is_this(&mut cs, &is::is_new_line) {
            cs.next();
            ast = push_stmt(ast, parser(cs));
            break;
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
            if is::is_operator_sums(&op) {
                ast = push_op_sums(ast, op.clone());
            } else if is::is_operator_puroducts(&op) {
                ast = push_op_products(ast, op.clone());
            } else if is::is_operator_pows(&op) {
                ast = push_op_pows(ast, op.clone());
            } else if is::is_operator_eqls(&op) {
                ast = push_op_eqls(ast, op.clone());
            } else if is::is_operator_assign(&op) {
                ast = push_op_asi(ast, Op::Asi);
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
            ast = push_tree(ast, parser(code_in_brackets.clone().chars().peekable()));
            code_in_brackets.clear();
        }

        // 関数 (変数)
        else if is_this(&mut cs, &is::is_alphabet) {
            while is_this(&mut cs, &is::is_alphabet) {
                if let Some(c) = cs.next() {
                    ob.push(c);
                } else {
                    break;
                }
            }

            // 関数
            if is_this(&mut cs, &is::is_first_bracket) {
                cs.next();
                while !is_this(&mut cs, &is::is_second_bracket) {
                    if let Some(c) = cs.next() {
                        code_in_brackets.push(c);
                    } else {
                        break;
                    }
                }
                cs.next();
                ast = push_fun(ast, ob.clone(), parser(code_in_brackets.clone().chars().peekable()));
                code_in_brackets.clear();

            // 変数
            } else {
                ast = push_val(ast, ob.clone());
            }
            ob.clear();
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
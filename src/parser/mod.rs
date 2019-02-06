use super::tree::Tree;
use super::tree::Op;
use super::is;
use std::iter::Peekable;
use std::str::Chars;
use super::tree::PushBack;
use super::tree::TreeInsert;

pub fn parser(mut cs: Peekable<Chars>) -> Tree {
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
        else if is_this(&mut cs, &is::is_num) {
            let mut num = String::new();
            while is_this(&mut cs, &is::is_num) {
                if let Some(c) = cs.next() {
                    num.push(c);
                } else {
                    break;
                }
            }
            ast = push_nv(ast, Op::Lit(num.clone()));
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
            let mut op = String::new();
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
        }

        // 括弧
        else if is_this(&mut cs, &is::is_first_bracket) {
            ast = push_tree(ast, parser(eat_codes_in_bracket(&mut cs).clone().chars().peekable()));
        }

        // 関数 (変数)
        else if is_this(&mut cs, &is::is_alphabet) {
            let mut ob = String::new();
            while is_this(&mut cs, &is::is_alphabet) {
                if let Some(c) = cs.next() {
                    ob.push(c);
                } else {
                    break;
                }
            }

            // 関数
            if is_this(&mut cs, &is::is_first_bracket) {
                ast = push_fun(ast, ob.clone(), parser(eat_codes_in_bracket(&mut cs).clone().chars().peekable()));

            // 予約語

            // 変数
            } else {
                ast = push_nv(ast, Op::Val(ob.clone()));
            }
        }
    }
    ast
}

fn push_stmt(tree: Tree, insert_tree: Tree) -> Tree {
    Tree::new(Op::STMT(Box::new(tree))).left(insert_tree)
}

fn push_fun(tree: Tree, fnc: String, insert_tree: Tree) -> Tree {
    if tree.root == Op::Nil {
        tree.root(Op::Fun(fnc)).left(insert_tree)
    } else {
        Tree::new(Op::Fun(fnc)).left(tree).left(insert_tree)
    }
}

fn push_op_asi(tree: Tree, op: Op<String>) -> Tree {
    if let Op::Asi = op {
        match tree.root {
            Op::Nil => tree.root(op),
            Op::Asi => Tree::new(tree.root)
                            .left(tree.left)
                            .right(Tree::new(Op::Asi).left(tree.right)),
            Op::Val(_) | Op::Lit(_)
                    => panic!("not operator"),
            _       => Tree::new(Op::Asi).left(tree)
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
                        .right(Tree::new(Tree::enum_op(op)).left(tree.right)),
        Op::Val(_) | Op::Lit(_)
                => panic!("not operator"),
        _       => Tree::new(Tree::enum_op(op))
                        .left(tree)
    }
}

fn push_op_sums(tree: Tree, op: String)
    -> Tree {
        match tree.root {
            Op::Nil => tree.root(Tree::enum_op(op)),
            Op::Asi | Op::STMT(_) | Op::Fun(_) | Op::ROp(_) =>
                        Tree::new(tree.root)
                        .left(tree.left)
                        .right(Tree::new(Tree::enum_op(op)).left(tree.right)),
            Op::Add | Op::Neg | Op::Mul | Op::Div | Op::Rem | Op::Pow =>
                Tree::new(Tree::enum_op(op)).left(tree),
            Op::Val(_) | Op::Lit(_) => panic!("not operator"),
        }
}

fn push_op_products(tree: Tree, op: String)
    -> Tree {
    match tree.root {
        Op::Nil
            => tree.root(Tree::enum_op(op)),
        Op::Asi | Op::STMT(_) | Op::Fun(_) | Op::ROp(_) | Op::Add | Op::Neg
            => Tree::new(tree.root)
                    .left(tree.left)
                    .right(Tree::new(Tree::enum_op(op)).left(tree.right)),
        Op::Mul | Op::Div | Op::Rem | Op::Pow
            => Tree::new(Tree::enum_op(op)).left(tree),
        Op::Val(_) | Op::Lit(_)
            => panic!("not operator"),
    }
}

fn push_op_pows(tree: Tree, op: String)
    -> Tree {
        match tree.root {
            Op::Nil
                => tree.root(Tree::enum_op(op)),
            Op::Asi | Op::STMT(_) | Op::Fun(_) | Op::ROp(_) | Op::Add |
            Op::Neg | Op::Mul | Op::Div | Op::Rem
                => Tree::new(tree.root)
                        .left(tree.left)
                        .right(Tree::new(Tree::enum_op(op)).left(tree.right)),
            Op::Pow
                => Tree::new(Tree::enum_op(op))
                        .left(tree),
            Op::Val(_) | Op::Lit(_)
                => panic!("not operator"),
        }
}

// push numbers and values
fn push_nv(mut tree: Tree, op: Op<String>) -> Tree {
    if tree.root == Op::Nil {
        tree.left(Tree::new(op))
    } else {
        tree.push_back(op);
        tree
    }
}

fn push_tree(mut tree: Tree, insert_tree: Tree) -> Tree {
    if tree.root == Op::Nil {
        tree.left(insert_tree)
    } else {
        tree.push_back(insert_tree);
        tree
    }
}


fn is_this(cs: &mut Peekable<Chars>, f: &Fn(&char)->bool) -> bool {
    match cs.peek() {
        Some(c) => f(&c),
        None => false,
    }
}

fn eat_codes_in_bracket(cs: &mut Peekable<Chars>) -> String {
    cs.next();
    let mut code_in_brackets = String::new();
    let mut bracket_num = 1;
    loop {
        if is_this(cs, &is::is_first_bracket) {
            bracket_num += 1;
        } else if is_this(cs, &is::is_second_bracket) {
            bracket_num -= 1;
            if bracket_num == 0 {
                break;
            }
        }
        if let Some(c) = cs.next() {
            code_in_brackets.push(c);
        } else {
            break;
        }
    }
    cs.next();
    code_in_brackets
}
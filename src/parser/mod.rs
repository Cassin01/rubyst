mod push;
mod eat;
mod reserved_functions;
use super::tree::Tree;
use super::tree::Op;
use super::is;
use std::iter::Peekable;
use std::str::Chars;

pub fn parser(mut cs: Peekable<Chars>) -> Tree {
    // Abstract syntax tree
    let mut ast = Tree::new(Op::Nil);

    loop {
        match cs.peek() {
            Some(_) => (),
            None => break,
        }

        // 複文
        if is::is_this(&mut cs, &is::is_new_line) {
            cs.next();
            ast = push::push_stmt(ast, parser(cs));
            break;
        }

        // 数字
        else if is::is_this(&mut cs, &is::is_num) {
            ast = push::push_nv(ast, Op::Num(make_name(&mut cs, &is::is_num).clone()));
        }

        // スペース
        else if is::is_this(&mut cs, &is::is_space) {
            while is::is_this(&mut cs, &is::is_space) {
                if let Some(_) = cs.next() {
                } else {
                    break;
                }
            }
        }

        // 演算子
        else if is::is_this(&mut cs, &is::is_operator) {
            let op = make_name(&mut cs, &is::is_operator);
            if is::is_operator_sums(&op) {
                ast = push::push_op_sums(ast, op.clone());
            } else if is::is_operator_puroducts(&op) {
                ast = push::push_op_products(ast, op.clone());
            } else if is::is_operator_pows(&op) {
                ast = push::push_op_pows(ast, op.clone());
            } else if is::is_operator_eqls(&op) {
                ast = push::push_op_eqls(ast, op.clone());
            } else if is::is_operator_assign(&op) {
                ast = push::push_op_asi(ast, Op::Asi);
            } else {
                panic!("oprator {:?} is not supported", op);
            }
        }

        // 括弧
        else if is::is_this(&mut cs, &is::is_first_bracket) {
            ast = push::push_tree(ast, parser(eat::eat_codes_in_bracket(&mut cs).clone().chars().peekable()));
        }

        // 文字列
        else if is::is_this(&mut cs, &is::is_quotation) {
            ast = push::push_tree(ast, Tree::new(Op::Str(eat::in_quotation(&mut cs))));
        }

        // 関数, 変数
        else if is::is_this(&mut cs, &is::is_alphabet) {
            // 名前取得
            let ob = make_name(&mut cs, &is::is_alphabet);

            // 関数
            if is::is_this(&mut cs, &is::is_first_bracket) {
                ast = push::push_fun(ast, ob, parser(eat::eat_codes_in_bracket(&mut cs).clone().chars().peekable()));

            // 予約語
            } else if is::reserved_function(&ob) {
                ast = reserved_functions::reserved(&mut cs, ast, ob);
                break;

            // 変数
            } else {
                ast = push::push_nv(ast, Op::Val(ob));
            }
        }
    }
    ast
}

// 同一の種類の文字で構成されていなくなるまで読み込む
fn make_name(cs: &mut Peekable<Chars>, is: &Fn(&char)->bool) -> String {
    let mut name = String::new();
    while is::is_this(cs, &is) {
        if let Some(c) = cs.next() {
            name.push(c);
        } else {
            break;
        }
    }
    name
}
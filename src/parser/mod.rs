mod push;
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
            ast = push::push_nv(ast, Op::Lit(make_name(&mut cs, &is::is_num).clone()));
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
            }
        }

        // 括弧
        else if is::is_this(&mut cs, &is::is_first_bracket) {
            ast = push::push_tree(ast, parser(eat_codes_in_bracket(&mut cs).clone().chars().peekable()));
        }

        // 関数, 変数
        else if is::is_this(&mut cs, &is::is_alphabet) {
            // 名前取得
            let ob = make_name(&mut cs, &is::is_alphabet);

            // 関数
            if is::is_this(&mut cs, &is::is_first_bracket) {
                ast = push::push_fun(ast, ob, parser(eat_codes_in_bracket(&mut cs).clone().chars().peekable()));

            // 予約語
            } else if is::is_reserved_word(&ob) {

            // 変数
            } else {
                ast = push::push_nv(ast, Op::Val(ob));
            }
        }
    }
    ast
}

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

fn eat_codes_in_bracket(cs: &mut Peekable<Chars>) -> String {
    cs.next();
    let mut code_in_brackets = String::new();
    let mut bracket_num = 1;
    loop {
        if is::is_this(cs, &is::is_first_bracket) {
            bracket_num += 1;
        } else if is::is_this(cs, &is::is_second_bracket) {
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
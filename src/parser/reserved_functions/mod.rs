// src/parser/reserved_functions
// This code makes a reserved funciton in code a parser

use std::iter::Peekable;
use std::str::Chars;
use super::push;
use super::eat;
use super::super::is;
use super::super::tree::Tree;
use super::super::tree::Op;
use super::super::tree::TreeInsert;
use super::parser;

pub fn reserved(cs: &mut Peekable<Chars>, ast: Tree, ob: String) -> Tree {
    match ob {
        ref ob if is::reserved_while(&ob) => reserved_while(cs, ast, ob.to_string()),
        ref ob if is::reserved_if(&ob)    => reserved_if(cs, ast, ob.to_string()),
        _ => panic!("reserved funciton '{}' is not suported"),
    }
}

#[allow(dead_code, unused_variables)]
fn reserved_while(cs: &mut Peekable<Chars>, ast: Tree, ob: String) -> Tree {
    ast
}

fn reserved_if(cs: &mut Peekable<Chars>, mut ast: Tree, ob: String) -> Tree {
    let condition = eat::eat_condition(cs);
    let mut if_num = 1;
    let in_if = eat::eat_in_if(cs, &mut if_num);
    // else が呼ばれた場合
    if if_num == 1 {
        let in_else = eat::eat_in_if(cs, &mut if_num);
        let else_tree = Tree::new(Op::Fun(String::from("else"))).left(parser(in_if.chars().peekable())).right(parser(in_else.chars().peekable()));

        if ast.root == Op::Nil {
            if ast.left == None {
                ast = ast.root(Op::Fun(ob))
                            .left(parser(condition.chars().peekable()))
                            .right(else_tree);
                // if 終了後改行が呼ばれないのでここで呼ぶ
                ast = push::push_stmt(ast, parser(cs.clone()));
                return ast
            } else {
                panic!("if can't return value ");
            }
        } else {
            panic!("undefined medthod tree.root for {:?} (NoMethodError)", ast.root);
        }

    // else が呼ばれなかった場合
    } else {
        if ast.root == Op::Nil {
            if ast.left == None {
                ast = ast.root(Op::Fun(ob))
                        .left(parser(condition.chars().peekable()))
                        .right(parser(in_if.chars().peekable()));
                // if 終了後改行が呼ばれないのでここで呼ぶ
                ast = push::push_stmt(ast, parser(cs.clone()));
                return ast
            } else {
                panic!("if can't return value ");
            }
        } else {
            panic!("undefined medthod tree.root for {:?} (NoMethodError)", ast.root);
        }
    }
}
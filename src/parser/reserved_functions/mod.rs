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
use super::super::tree::PushBack;
use super::parser;

pub fn reserved(cs: &mut Peekable<Chars>, ast: Tree, ob: String) -> Tree {
    match ob {
        ref ob if is::reserved_while(&ob) => reserved_while(cs, ast, ob.to_string()),
        ref ob if is::reserved_if(&ob)    => reserved_if(cs, ast, ob.to_string()),
        ref ob if is::reserved_begin(&ob) => reserved_begin(cs, ast, ob.to_string()),
        ref ob if is::reserved_case(&ob)  => reserved_case(cs, ast),
        _ => panic!("reserved funciton '{}' is not suported"),
    }
}

fn reserved_begin(cs: &mut Peekable<Chars>, mut ast: Tree, ob: String) -> Tree {
    let in_while = eat::in_while(cs);
    let mut word = String::new();
    loop {
        if is::is_this(cs, &is::is_new_line) || is::is_this(cs, &is::is_space){
            if let Some(_) = cs.next() {
                if !word.is_empty() {
                    if word == String::from("while") {
                        break;
                    } else {
                        panic!("should be 'while' but '{}' (No While Err)", word);
                    }
                }
            } else {
                panic!("begin was called but code was end without calling 'while' (No While Err)");
            }
        }
        else if is::is_this(cs, &is::is_alphabet){
            if let Some(x) = cs.next() {
                word.push(x);
            } else {
                panic!("begin was called but code was end without calling 'while' (No While Err)");
            }
        }
    }
    let condition = eat::eat_condition(cs);

    if ast.root == Op::Nil {
        if ast.left == None {
            ast = ast.root(Op::Fun(ob))
                    .left(parser(condition.chars().peekable()))
                    .right(parser(in_while.chars().peekable()));
            // if 終了後改行が呼ばれないのでここで呼ぶ
            ast = push::push_stmt(ast, parser(cs.clone()));
            ast
        } else {
            panic!("if can't return value ");
        }
    } else {
        panic!("undefined medthod tree.root for {:?} (NoMethodError)", ast.root);
    }
}

fn reserved_while(cs: &mut Peekable<Chars>, mut ast: Tree, ob: String) -> Tree {
    let condition = eat::eat_condition(cs);
    let in_while = eat::in_while(cs);

    if ast.root == Op::Nil {
        if ast.left == None {
            ast = ast.root(Op::Fun(ob))
                    .left(parser(condition.chars().peekable()))
                    .right(parser(in_while.chars().peekable()));
            // if 終了後改行が呼ばれないのでここで呼ぶ
            ast = push::push_stmt(ast, parser(cs.clone()));
            ast
        } else {
            panic!("if can't return value ");
        }
    } else {
        panic!("undefined medthod tree.root for {:?} (NoMethodError)", ast.root);
    }
}

fn reserved_if(cs: &mut Peekable<Chars>, ast: Tree, ob: String) -> Tree {
    let condition = parser(eat::eat_condition(cs).chars().peekable());
    push::push_stmt(if_raw(cs, ast, ob, condition), parser(cs.clone()))
}

fn reserved_case(cs: &mut Peekable<Chars>, mut ast: Tree) -> Tree {
    let left = eat::eat_condition(cs);
    let mut times = 1; // 0 -> end, 1 -> when
    let _ = eat_till_when(cs, &mut times);
    match times {
        0 => return ast,
        1 => (),
        other  => panic!("'{}' is not supproted in this time", other),
    }
    loop {
        let right = eat::eat_condition(cs);
        let condition = left.clone() + " " + "==" + " " + &right;
        let closure = eat_till_when(cs, &mut times);
        match times {
            0 =>    if ast.root == Op::Nil {
                        if ast.left == None {
                            ast = ast.root(Op::Fun("if".to_string()))
                                    .left(parser(condition.chars().peekable()))
                                    .right(parser(closure.chars().peekable()));
                            // 改行
                            ast = push::push_stmt_left(ast);
                            ast.push_back(Op::Nil);
                            return ast
                        } else {
                            panic!("if can't return value ");
                        }
                    } else if ast.root == Op::STMT {
                        let insert_tree = push::push_stmt_left(Tree::new(Op::Fun("if".to_string())).left(parser(condition.chars().peekable())).right(parser(closure.chars().peekable())));
                            ast = push::push_tree(ast, insert_tree);
                            ast.push_back(Op::Nil);
                            return ast
                    } else {
                        panic!("undefined medthod tree.root for {:?} (NoMethodError)", ast.root);
                    },
            1 =>    if ast.root == Op::Nil {
                        if ast.left == None {
                            ast = ast.root(Op::Fun("if".to_string()))
                                    .left(parser(condition.chars().peekable()))
                                    .right(parser(closure.chars().peekable()));
                            // 改行
                            ast = push::push_stmt_left(ast);
                        } else {
                            panic!("if can't return value ");
                        }
                    } else if ast.root == Op::STMT {
                        let insert_tree = push::push_stmt_left(Tree::new(Op::Fun("if".to_string())).left(parser(condition.chars().peekable())).right(parser(closure.chars().peekable())));
                            ast = push::push_tree(ast, insert_tree);
                    } else {
                        panic!("undefined medthod tree.root for {:?} (NoMethodError)", ast.root);
                    },
            _  => {
                panic!("'{}' is not supproted in this time");
            }
        }
    }
}

fn eat_till_when(cs: &mut Peekable<Chars>, times: &mut i64) -> String {
    let mut closure = String::new();
    let mut word = String::new();
    let mut begin_times = 0;
    loop {
        if is::is_this(cs, &is::is_new_line) || is::is_this(cs, &is::is_space) {
            if let Some(x) = cs.next() {
                if !word.is_empty() {

                    if is::in_closure(&word) {
                        *times += 1;

                        // begin ~ while
                        if word == String::from("begin") {
                            begin_times += 1;
                        }
                        if word == String::from("while") && begin_times > 0 {
                            begin_times -= 1;
                            *times -= 1;
                        }
                    } else if word == String::from("end") {
                        *times -= 1;
                        if times == &0 {
                            return closure
                        } else {
                            word.push(x);
                            closure.push_str(word.as_str());
                            word.clear();
                        }
                    } else if word == String::from("when") {
                        if times == &1 {
                            return closure
                        } else {
                            word.push(x);
                            closure.push_str(word.as_str());
                            word.clear();
                        }
                    } else {
                        word.push(x);
                        closure.push_str(word.as_str());
                        word.clear();
                    }
                } else {
                    word.push(x);
                    closure.push_str(word.as_str());
                    word.clear();
                }
            } else {
                panic!("not supprted");
            }
        }
        else {
            if let Some(x) = cs.next() {
                word.push(x);
            } else {
                if word == String::from("end") {
                    *times -= 1;
                    if times == &0 {
                        return closure
                    } else {
                        panic!("case was called but code was end without calling 'end'.\n Code require more 'end' identifire.");
                    }
                } else {
                    panic!("case was called but code was end without calling 'end'");
                }
            }
        }
    }
}

fn if_raw(cs: &mut Peekable<Chars>, ast: Tree, ob: String, condition: Tree) -> Tree {
    let mut if_num = 1;
    let in_if = eat::in_if(cs, &mut if_num);
    // else が呼ばれた場合
    if if_num == 1 {
        let in_else = eat::in_if(cs, &mut if_num);
        let else_tree = Tree::new(Op::Fun(String::from("else"))).left(parser(in_if.chars().peekable())).right(parser(in_else.chars().peekable()));

        if ast.root == Op::Nil {
            if ast.left == None {
                ast.root(Op::Fun(ob))
                            .left(condition)
                            .right(else_tree)
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
                ast.root(Op::Fun(ob))
                    .left(condition)
                    .right(parser(in_if.chars().peekable()))
            } else {
                panic!("if can't return value ");
            }
        } else {
            panic!("undefined medthod tree.root for {:?} (NoMethodError)", ast.root);
        }
    }
}
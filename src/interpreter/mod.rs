use super::tree::Tree;
use super::tree::Op;
mod functions;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int(i64),
    Bool(bool),
}
use self::Type::Int;
use self::Type::Bool;

impl Tree {
    fn extract_option(leaf: Option<Box<Self>>) ->  Tree {
        match leaf {
            Some(x) => *x,
            None    => panic!("this leaf don't have value"),
        }
    }
}

pub fn evaluate(tree: Tree) -> Type {
    match tree.root {
        Op::Lit(x) => Int(x.parse().unwrap()),
        Op::Add    => adapt_funci(tree, &functions::add),
        Op::Neg    => adapt_funci(tree, &functions::neg),
        Op::Mul    => adapt_funci(tree, &functions::mul),
        Op::Div    => adapt_funci(tree, &functions::div),
        Op::Rem    => adapt_funci(tree, &functions::rem),
        Op::Pow    => adapt_funci(tree, &functions::pow),
        Op::ROp(_) => adapt_funcb(tree, &rop),
        Op::Nil => panic!("not interpret"),
    }
}

fn adapt_funci(tree: Tree, f: &Fn(i64, i64)-> i64) -> Type {
    if let Int(left)  = evaluate(Tree::extract_option(tree.left)) {
        if let Int(right) = evaluate(Tree::extract_option(tree.right)) {
            Int(f(left, right))
        } else {
            panic!("not int");
        }
    } else {
            panic!("not int");
    }
}

fn adapt_funcb(tree: Tree, f: &Fn(String, i64, i64)-> bool) -> Type {
    if let Op::ROp(op) = tree.root {
        if let Int(left)  = evaluate(Tree::extract_option(tree.left)) {
            if let Int(right) = evaluate(Tree::extract_option(tree.right)) {
                Bool(f(op, left, right))
            } else {
                panic!("not int");
            }
        } else {
                panic!("not int");
        }
    } else {
        panic!("not rop");
    }
}

fn rop(op: String, l: i64, r: i64) -> bool {
    match op.as_str() {
        "==" =>  functions::eq(l, r),
        "!=" => !functions::eq(l, r),
        "<"  =>  functions::lt(l, r),
        "<=" => !functions::gt(l, r),
        ">"  =>  functions::gt(l, r),
        ">=" => !functions::lt(l, r),
        _    => panic!("not operation"),
    }
}
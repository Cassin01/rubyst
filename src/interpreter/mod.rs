use super::tree::Tree;
use super::tree::Op;
mod functions;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int(i64),
    Bool(bool),
    Nil,
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
        Op::Fun(_)    => adapt_funcf(tree),
        Op::Nil => panic!("not interpret"),
    }
}

fn p(t: Type) -> Type{
    match t {
        Int(x)  => println!("{}", x),
        Bool(x) => println!("{}", x),
        _       => panic!("funciton p is support type {:?}", t),
    }
    Type::Nil
}

fn adapt_funcf(tree: Tree) -> Type {
    if let Op::Fun(fun) = tree.root {
        match fun.as_str() {
            "p" => {
                p(evaluate(Tree::extract_option(tree.left)))
            }
            _ => panic!("this function is not supproted"),
        }
    } else {
        panic!("this is not function");
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
    if let Int(left)  = evaluate(Tree::extract_option(tree.left)) {
        if let Int(right) = evaluate(Tree::extract_option(tree.right)) {
            if let Op::ROp(op) = tree.root {
                Bool(f(op, left, right))
            } else {
                panic!("not rop");
            }
        } else {
            panic!("not int");
        }
    } else {
        panic!("not int");
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
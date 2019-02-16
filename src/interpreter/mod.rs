use super::tree::Tree;
use super::tree::Op;
use std::collections::HashMap;
mod functions;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int(i64),
    Bool(bool),
    Val(String),
    Nil,
}

impl Tree {
    fn extract_option(leaf: Option<Box<Self>>) ->  Tree {
        match leaf {
            Some(x) => *x,
            None    => panic!("this leaf don't have value"),
        }
    }
}

pub fn interpret(tree: Tree) -> Type {
    // variable and values
    let mut vvs = HashMap::new();
    evaluate(tree, &mut vvs)
}

fn evaluate(tree: Tree, vvs: &mut HashMap<String, Type>) -> Type {
    match tree.root {
        Op::Lit(x)  => Type::Int(x.parse().unwrap()),
        Op::Add     => adapt_funci(tree, &functions::add, vvs),
        Op::Neg     => adapt_funci(tree, &functions::neg, vvs),
        Op::Mul     => adapt_funci(tree, &functions::mul, vvs),
        Op::Div     => adapt_funci(tree, &functions::div, vvs),
        Op::Rem     => adapt_funci(tree, &functions::rem, vvs),
        Op::Pow     => adapt_funci(tree, &functions::pow, vvs),
        Op::ROp(_)  => adapt_funcb(tree, &rop, vvs),
        Op::Fun(_)  => adapt_funcf(tree, vvs),
        Op::STMT    => adapt_func_stmt(tree, vvs),
        Op::Val(x)  => Type::Val(x),
        Op::Asi     => adapt_func_assi(tree, vvs),
        Op::Nil     => adapt_func_nil(tree, vvs),
    }
}

fn p(t: Type, vvs: &HashMap<String, Type>) -> Type {
    match t {
        Type::Int(x)  => println!("{}", x),
        Type::Bool(x) => println!("{}", x),
        Type::Val(x)  => return p(vvs[&x].clone(), vvs),
        Type::Nil   => (),
    }
    Type::Nil
}

fn adapt_func_nil(tree: Tree, vvs: &mut HashMap<String, Type>) -> Type {
    if tree.left == None && tree.right == None {
        Type::Nil
    } else if tree.right == None {
        evaluate(Tree::extract_option(tree.left), vvs)
    } else if tree.left == None {
        evaluate(Tree::extract_option(tree.right), vvs)
    } else {
        panic!("dont use Nil at end of tree");
    }
}

fn adapt_func_assi(tree: Tree, vvs: &mut HashMap<String, Type>) -> Type {
    if let Type::Val(left) = evaluate(Tree::extract_option(tree.left), vvs) {
        let mut cvvs = vvs.clone();
        vvs.insert(left.clone(), evaluate(Tree::extract_option(tree.right), &mut cvvs));
        Type::Val(left)
    } else {
        panic!("not left is not valuable");
    }
}

fn adapt_func_stmt(tree: Tree, vvs: &mut HashMap<String, Type>) -> Type {
    if let Op::STMT = tree.root {
        evaluate(Tree::extract_option(tree.left), vvs);
        evaluate(Tree::extract_option(tree.right), vvs)
    } else {
        panic!("This is not statement");
    }
}

fn adapt_funcf(tree: Tree, vvs: &mut HashMap<String, Type>) -> Type {
    if let Op::Fun(fun) = tree.root.clone() {
        match fun.as_str() {
            "p"  => p(evaluate(Tree::extract_option(tree.left), vvs), vvs),
            "if" => f_if(tree, vvs),
            _ => panic!("this function is not supproted"),
        }
    } else {
        panic!("this is not function");
    }
}

fn adapt_funci(tree: Tree, f: &Fn(i64, i64)-> i64, vvs: &mut HashMap<String, Type>) -> Type {
    use self::Type::Int;
    use self::Type::Val;
    match evaluate(Tree::extract_option(tree.left), vvs) {
        Int(left) => {
            match evaluate(Tree::extract_option(tree.right), vvs) {
                Int(right) => Int(f(left, right)),
                Val(right) => if let Int(r) = vvs[&right] {
                    Int(f(left, r))
                } else {
                    panic!("not int");
                },
                _ => panic!("not int"),
            }
        }
        Val(left) => {
            if let Int(l) = vvs[&left] {
                match evaluate(Tree::extract_option(tree.right), vvs) {
                    Int(right) => Int(f(l, right)),
                    Val(right) =>
                        if let Int(r) = vvs[&right] {
                            Int(f(l, r))
                        } else {
                            panic!("not int");
                        },
                    _ => panic!("not int"),
                }
            } else {
                panic!("not int");
            }
        }
        _ => panic!("not int"),
    }
}

fn adapt_funcb(tree: Tree, f: &Fn(String, i64, i64)-> bool, vvs: &mut HashMap<String, Type>) -> Type {
    use self::Type::Int;
    use self::Type::Bool;
    use self::Type::Val;
    fn rb(root: Op<String>, f: &Fn(String, i64, i64) -> bool , l: i64, r: i64) -> Type {
        if let Op::ROp(op) = root {
            Bool(f(op, l, r))
        } else {
            panic!("not rop");
        }
    }
    match evaluate(Tree::extract_option(tree.left), vvs) {
        Int(left) => {
            match evaluate(Tree::extract_option(tree.right), vvs) {
                Int(right) => rb(tree.root, f, left, right),
                Val(right) =>
                    if let Int(r) = vvs[&right] {
                        rb(tree.root, f, left, r)
                    } else {
                        panic!("not int");
                    },
                _ => panic!("not int"),
            }
        }
        Val(left) => {
            if let Int(l) = vvs[&left] {
                match evaluate(Tree::extract_option(tree.right), vvs) {
                    Int(right) => rb(tree.root, f, l, right),
                    Val(right) =>
                        if let Int(r) = vvs[&right] {
                            rb(tree.root, f, l, r)
                        } else {
                            panic!("not int");
                        },
                    _ => panic!("not int"),
                }
            } else {
                panic!("not int");
            }
        }
        _ => panic!("not int"),
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

fn f_if(tree: Tree, vvs: &mut HashMap<String, Type>)  -> Type {
    if let Some(t) = tree.right {
        match t.root {
            Op::STMT | Op::Nil =>
                match evaluate(Tree::extract_option(tree.left), vvs) {
                    Type::Bool(true) => evaluate(*t, vvs),
                    Type::Bool(false) => return Type::Nil,
                    _ => panic!("not condition"),
                },
            Op::Fun(_) => // else
                match evaluate(Tree::extract_option(tree.left), vvs) {
                    Type::Bool(true) => evaluate(Tree::extract_option(t.left), vvs),
                    Type::Bool(false) => evaluate(Tree::extract_option(t.right), vvs),
                    _ => panic!("not condition"),
                },
            y => panic!("Op {:?} is not supported", y),
        }
    } else {
        panic!("this tree.right shoud be exsist");

    }
}
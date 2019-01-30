use super::tree::Tree;
use super::tree::Op;

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
        Op::Add => {
            if let Int(left)  = evaluate(Tree::extract_option(tree.left)) {
                if let Int(right) = evaluate(Tree::extract_option(tree.right)) {
                    Int(left + right)
                } else {
                    panic!("not int");
                }
            } else {
                    panic!("not int");
            }
        },
        Op::Neg => {
            if let Int(left)  = evaluate(Tree::extract_option(tree.left)) {
                if let Int(right) = evaluate(Tree::extract_option(tree.right)) {
                    Int(left - right)
                } else {
                    panic!("not int");
                }
            } else {
                    panic!("not int");
            }
        },
        Op::Mul => {
            if let Int(left)  = evaluate(Tree::extract_option(tree.left)) {
                if let Int(right) = evaluate(Tree::extract_option(tree.right)) {
                    Int(left * right)
                } else {
                    panic!("not int");
                }
            } else {
                    panic!("not int");
            }
        },
        Op::Div => {
            if let Int(left)  = evaluate(Tree::extract_option(tree.left)) {
                if let Int(right) = evaluate(Tree::extract_option(tree.right)) {
                    Int(left / right)
                } else {
                    panic!("not int");
                }
            } else {
                    panic!("not int");
            }
        },
        Op::Rem => {
            if let Int(left)  = evaluate(Tree::extract_option(tree.left)) {
                if let Int(right) = evaluate(Tree::extract_option(tree.right)) {
                    Int(left % right)
                } else {
                    panic!("not int");
                }
            } else {
                    panic!("not int");
            }
        }
        Op::Pow => {
            if let Int(left)  = evaluate(Tree::extract_option(tree.left)) {
                if let Int(right) = evaluate(Tree::extract_option(tree.right)) {
                    if right >= 0 {
                        Int(left.pow(right as u32))
                    } else {
                        Int(1 / left.pow((right * -1) as u32))
                    }
                } else {
                    panic!("not int");
                }
            } else {
                    panic!("not int");
            }
        }
        Op::Eql => {
            if let Int(left)  = evaluate(Tree::extract_option(tree.left)) {
                if let Int(right) = evaluate(Tree::extract_option(tree.right)) {
                    if left == right {
                        Bool(true)
                    } else {
                        Bool(false)
                    }
                } else {
                    panic!("not int");
                }
            } else {
                    panic!("not int");
            }
        }
        Op::NEq => {
            if let Bool(x) = evaluate(Tree::new(Op::Eql).left(tree.left).right(tree.right)) {
                Bool(!x)
            } else {
                panic!("not bool");
            }
        }

        Op::Nil => panic!("not interpret"),
    }
}
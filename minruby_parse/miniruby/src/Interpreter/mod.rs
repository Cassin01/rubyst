use super::tree::Tree;
use super::tree::Op;

impl Tree {
    fn extract_option(leaf: Option<Box<Self>>) ->  Tree {
        match leaf {
            Some(x) => *x,
            None    => panic!("this leaf don't have value"),
        }
    }
}


pub fn evaluate(tree: Tree) -> i64 {
    match tree.root {
        Op::Lit(x) => x.parse().unwrap(),
        Op::Plus => {
            let left  = evaluate(Tree::extract_option(tree.left));
            let right = evaluate(Tree::extract_option(tree.right));
            left + right
        },
        Op::Minus => {
            let left  = evaluate(Tree::extract_option(tree.left));
            let right = evaluate(Tree::extract_option(tree.right));
            left - right
        },
        Op::Mult => {
            let left  = evaluate(Tree::extract_option(tree.left));
            let right = evaluate(Tree::extract_option(tree.right));
            left * right
        },
        Op::Divid => {
            let left  = evaluate(Tree::extract_option(tree.left));
            let right = evaluate(Tree::extract_option(tree.right));
            left + right
        },
        Op::Nil => panic!("not interpret"),
    }
}
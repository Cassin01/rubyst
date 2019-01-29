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
        Op::Add => {
            let left  = evaluate(Tree::extract_option(tree.left));
            let right = evaluate(Tree::extract_option(tree.right));
            left + right
        },
        Op::Neg => {
            let left  = evaluate(Tree::extract_option(tree.left));
            let right = evaluate(Tree::extract_option(tree.right));
            left - right
        },
        Op::Mul => {
            let left  = evaluate(Tree::extract_option(tree.left));
            let right = evaluate(Tree::extract_option(tree.right));
            left * right
        },
        Op::Div => {
            let left  = evaluate(Tree::extract_option(tree.left));
            let right = evaluate(Tree::extract_option(tree.right));
            left + right
        },
        Op::Rem => {
            let left  = evaluate(Tree::extract_option(tree.left));
            let right = evaluate(Tree::extract_option(tree.right));
            left % right
        }
        Op::Nil => panic!("not interpret"),
    }
}
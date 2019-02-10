use super::super::tree::PushBack;
use super::super::tree::TreeInsert;
use super::super::tree::Tree;
use super::super::tree::Op;

pub fn push_stmt(tree: Tree, insert_tree: Tree) -> Tree {
    Tree::new(Op::STMT).left(tree).right(insert_tree)
}

pub fn push_fun(tree: Tree, fnc: String, insert_tree: Tree) -> Tree {
    if tree.root == Op::Nil {
        tree.root(Op::Fun(fnc)).left(insert_tree)
    } else {
        push_tree(tree, Tree::new(Op::Fun(fnc)).left(insert_tree))
    }
}

pub fn push_op_asi(tree: Tree, op: Op<String>) -> Tree {
    if let Op::Asi = op {
        match tree.root {
            Op::Nil => tree.root(op),
            Op::Asi => Tree::new(tree.root)
                            .left(tree.left)
                            .right(Tree::new(Op::Asi).left(tree.right)),
            Op::Val(_) | Op::Lit(_)
                    => panic!("not operator"),
            _       => Tree::new(Op::Asi).left(tree)
        }
    } else {
        panic!("not assignment operation");
    }
}

pub fn push_op_eqls(tree: Tree, op: String) -> Tree {
    match tree.root {
        Op::Nil => tree.root(Tree::enum_op(op)),
        Op::Asi => Tree::new(tree.root)
                        .left(tree.left)
                        .right(Tree::new(Tree::enum_op(op)).left(tree.right)),
        Op::Val(_) | Op::Lit(_)
                => panic!("not operator"),
        _       => Tree::new(Tree::enum_op(op))
                        .left(tree)
    }
}

pub fn push_op_sums(tree: Tree, op: String)
    -> Tree {
    match tree.root {
        Op::Nil => tree.root(Tree::enum_op(op)),
        Op::Asi | Op::STMT | Op::Fun(_) | Op::ROp(_) =>
                    Tree::new(tree.root)
                    .left(tree.left)
                    .right(Tree::new(Tree::enum_op(op)).left(tree.right)),
        Op::Add | Op::Neg | Op::Mul | Op::Div | Op::Rem | Op::Pow =>
            Tree::new(Tree::enum_op(op)).left(tree),
        Op::Val(_) | Op::Lit(_) => panic!("not operator"),
    }
}

pub fn push_op_products(tree: Tree, op: String)
    -> Tree {
    match tree.root {
        Op::Nil
            => tree.root(Tree::enum_op(op)),
        Op::Asi | Op::STMT | Op::Fun(_) | Op::ROp(_) | Op::Add | Op::Neg
            => Tree::new(tree.root)
                    .left(tree.left)
                    .right(Tree::new(Tree::enum_op(op)).left(tree.right)),
        Op::Mul | Op::Div | Op::Rem | Op::Pow
            => Tree::new(Tree::enum_op(op)).left(tree),
        Op::Val(_) | Op::Lit(_)
            => panic!("not operator"),
    }
}

pub fn push_op_pows(tree: Tree, op: String)
    -> Tree {
        match tree.root {
            Op::Nil
                => tree.root(Tree::enum_op(op)),
            Op::Asi | Op::STMT | Op::Fun(_) | Op::ROp(_) | Op::Add |
            Op::Neg | Op::Mul | Op::Div | Op::Rem
                => Tree::new(tree.root)
                        .left(tree.left)
                        .right(Tree::new(Tree::enum_op(op)).left(tree.right)),
            Op::Pow
                => Tree::new(Tree::enum_op(op))
                        .left(tree),
            Op::Val(_) | Op::Lit(_)
                => panic!("not operator"),
        }
}

// push numbers and values
pub fn push_nv(mut tree: Tree, op: Op<String>) -> Tree {
    if tree.root == Op::Nil {
        tree.left(Tree::new(op))
    } else {
        tree.push_back(op);
        tree
    }
}

pub fn push_tree(mut tree: Tree, insert_tree: Tree) -> Tree {
    if tree.root == Op::Nil {
        tree.left(insert_tree)
    } else {
        tree.push_back(insert_tree);
        tree
    }
}
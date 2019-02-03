use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Op<T> {
    Nil,
    Add,
    Neg,
    Mul,
    Div,
    Rem,
    Pow,
    Asi,    // Assignment operator
    Fun(T),
    ROp(T), // Relational operator
    Lit(T),
    Val(T),
    STMT(Box<Tree>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tree {
    pub root: Op<String>,
    pub left: Option<Box<Tree>>,
    pub right: Option<Box<Tree>>,
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tree({:?})", self.root)
    }
}

impl Tree {
    pub fn new(root: Op<String>) -> Tree {
        Tree {
            root: root,
            left: None,
            right: None,
        }
    }

    pub fn left(mut self, leaf: Option<Box<Tree>>) -> Self {
        self.left = leaf;
        self
    }

    pub fn right(mut self, leaf: Option<Box<Tree>>) -> Self {
        self.right = leaf;
        self
    }

    pub fn enum_op(op: String) -> Op<String> {
        match op.as_str() {
            "*"  => Op::Mul,
            "/"  => Op::Div,
            "+"  => Op::Add,
            "-"  => Op::Neg,
            "%"  => Op::Rem,
            "**" => Op::Pow,
            "==" => Op::ROp(String::from("==")),
            "!=" => Op::ROp(String::from("!=")),
            "<"  => Op::ROp(String::from("<")),
            "<=" => Op::ROp(String::from("<=")),
            ">=" => Op::ROp(String::from(">=")),
            ">"  => Op::ROp(String::from(">")),
            _    => panic!("not operator"),
        }
    }
    pub fn root_op(mut self, root: Op<String>) -> Self {
        self.root = root;
        self
    }

    pub fn root_fn(mut self, root: String) -> Self {
        self.root = Op::Fun(root);
        self
    }

    pub fn root(mut self, root: String) -> Self {
        self.root = Self::enum_op(root);
        self
    }
}

impl Tree {
    pub fn push_back(&mut self, value: String) {
        // 最終要素探索関数
        fn last_node(tree: &mut Option<Box<Tree>>) -> &mut Option<Box<Tree>> {
            if let Some(ref mut _n) = *tree {
                last_node(&mut _n.right)
            }
            else {
                tree
            }
        }
        let _node = last_node(&mut self.right);
        *_node = Some(Box::new(Tree::new(Op::Lit(value))));
    }

    pub fn push_backv(&mut self, value: String) {
        // 最終要素探索関数
        fn last_node(tree: &mut Option<Box<Tree>>) -> &mut Option<Box<Tree>> {
            if let Some(ref mut _n) = *tree {
                last_node(&mut _n.right)
            }
            else {
                tree
            }
        }
        let _node = last_node(&mut self.right);
        *_node = Some(Box::new(Tree::new(Op::Val(value))));
    }

    pub fn push_back_tree(&mut self, tree: Tree) {
        // 最終要素探索関数
        fn last_node(tree: &mut Option<Box<Tree>>) -> &mut Option<Box<Tree>> {
            if let Some(ref mut _n) = *tree {
                last_node(&mut _n.right)
            }
            else {
                tree
            }
        }
        let _node = last_node(&mut self.right);
        *_node = Some(Box::new(tree));
    }
}
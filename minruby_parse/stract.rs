use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum Op<T> {
    Nil,
    Plus,
    Minus,
    Mult,
    Divid,
    Num(T)
}

#[derive(Debug, Clone)]
struct Tree {
    root: Op<String>,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tree({:?})", self.root)
    }
}

impl Tree {
    fn new(root: Op<String>) -> Tree {
        Tree {
            root: root,
            left: None,
            right: None,
        }
    }

    fn left(mut self, leaf: Option<Box<Tree>>) -> Self {
        self.left = leaf;
        self
    }

    fn right(mut self, leaf: Option<Box<Tree>>) -> Self {
        self.right = leaf;
        self
    }

    fn root(mut self, root: String) -> Self {
        self.root = match root.as_str() {
            "*" => Op::Mult,
            "/" => Op::Divid,
            "+" => Op::Plus,
            "-" => Op::Minus,
            _   => panic!("not operator"),
        };
        self
    }
}

fn is_num(c: &char) -> bool {
    match c {
        '1' => true,
        '2' => true,
        '3' => true,
        '4' => true,
        '5' => true,
        '6' => true,
        '7' => true,
        '8' => true,
        '9' => true,
        _   => false
    }
}

fn is_space(c: &char) -> bool {
    match c {
        ' ' => true,
        _   => false,
    }
}

fn is_operator_sums(c: &char) -> bool {
    match c {
        '+' => true,
        '-' => true,
        _   => false,
    }
}

fn is_operator_products(c: &char) -> bool {
    match c {
        '*' => true,
        '/' => true,
        _   => false,
    }
}

fn enum_op(op: String) -> Op<String> {
    match op.as_str() {
        "*" => Op::Mult,
        "/" => Op::Divid,
        "+" => Op::Plus,
        "-" => Op::Minus,
        _   => panic!("not operator"),
    }
}

impl Tree {
    fn push_back(&mut self, value: String) {
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
        *_node = Some(Box::new(Tree::new(Op::Num(value))));
    }
}

fn push_num(mut tree: Tree, num: String) -> Tree {
    if tree.root == Op::Nil {
        tree.left(Some(Box::new(Tree::new(Op::Num(num)))))
    } else {
        tree.push_back(num);
        tree
    }
}

fn push_op(tree: Tree, op: String) -> Tree {
    if tree.root == Op::Nil {
        tree.root(op)
    } else {
        Tree::new(enum_op(op)).left(Some(Box::new(tree)))
    }
}

fn push_op_products(tree: Tree, op: String)
    -> Tree {
    match tree.root {
        Op::Nil => tree.root(op),
        Op::Plus | Op::Minus => Tree::new(tree.root).left(tree.left).right(Some(Box::new(Tree::new(enum_op(op)).left(tree.right)))),
        Op::Mult | Op::Divid => Tree::new(enum_op(op)).left(Some(Box::new(tree))),
        _ => panic!("not operator"),
    }
}


fn parser(code: String) -> Tree {
    let mut num = String::new();
    let mut num_flag = false;

    // Abstract syntax tree
    let mut ast = Tree::new(Op::Nil);

    for c in code.chars() {
        if is_num(&c) {
            num_flag = true;
            num.push(c);
        } else if is_space(&c) {
            if num_flag {
                ast = push_num(ast, num.clone());
                num.clear();
                num_flag = false;
            }
        } else if is_operator_sums(&c) {
            num_flag = false;
            ast = push_op(ast, c.to_string());
        } else if is_operator_products(&c) {
            num_flag = false;
            ast = push_op_products(ast, c.to_string());
        } else {
            panic!("err {} is not supported", c);
        }
    }
    if num_flag {
        ast = push_num(ast, num.clone());
        num.clear();
    }
    ast
}

fn main() {
    let code = String::from("1 + 2 + 3 + 3");
    let ast = parser(code);

    println!("{:?}", ast);
}
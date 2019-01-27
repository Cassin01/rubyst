use std::fmt;

#[derive(Debug)]
#[derive(Default)]
struct Tree {
    root: Option<String>,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tree({:?})", self.root)
    }
}

impl Tree {
  fn new(root: Option<String>) -> Tree {
    Tree {
      root: root,
      ..Default::default()
    }
  }

  fn left(mut self, leaf: Tree) -> Self {
    self.left = Some(Box::new(leaf));
    self
  }

  fn right(mut self, leaf: Tree) -> Self {
    self.right = Some(Box::new(leaf));
    self
  }

  fn root(mut self, root: String) -> Self {
    self.root = Some(root);
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

fn push_num(tree: Tree, num: String) -> Tree {
    if tree.root == None {
        tree.right(Tree::new(Some(num)))
    } else {
        tree.left(Tree::new(Some(num)))
    }
}

fn push_op(tree: Tree, op: String) -> Tree {
    if tree.root == None {
        tree.root(op)
    } else {
        Tree::new(Some(op)).right(tree)
    }
}


fn main() {
    let code = String::from("1 + 2 + 3");
    let mut num = String::new();
    let mut num_flag = false;

    // Abstract syntax tree
    let mut ast = Tree::new(None);

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
        }
        
    }


    println!("{:?}", ast);
}

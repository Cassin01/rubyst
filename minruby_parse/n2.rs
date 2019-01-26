#[derive(Debug)]
enum Tree {
    Cons(Option<String>,
         Option<Box<Tree>>,
         Option<Box<Tree>>),
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


impl Tree {
    fn new() -> Tree {
        Tree::Cons(None, None, None)
    }
    fn leaf(num: String) -> Tree {
        Tree::Cons(Some(num), None, None)
    }
}

fn push_num(tree: Tree, num: String) -> Tree {
    use Tree::Cons;
    match tree {
        Cons(None, _, _)     => Cons(None, Some(Box::new(Tree::leaf(num))), None),
        Cons(Some(op), x, _) => Cons(Some(op), x, Some(Box::new(Tree::leaf(num)))),
    }
}

fn push_op(tree: Tree, new_op: String) -> Tree {
    use Tree::Cons;
    match tree {
        Cons(None, x, _)     => Cons(Some(new_op), x, None),
        Cons(Some(op), x, y) => Cons(Some(new_op), Some(Box::new(Cons(Some(op), x,  y))), None),
    }
}

fn main() {
    let texts = String::from("1 + 2 + 3");
    let mut num = String::new();
    let mut tree = Tree::new();

    for c in texts.chars() {
        if is_num(&c) {
            num.push(c);
        } else if is_space(&c) {
        } else if is_operator_sums(&c) {
            tree = push_op(tree, c.to_string());
        } else {
            tree = push_num(tree, num.clone());
            num.clear();
        }
    }
    println!("{:?}", tree);
}

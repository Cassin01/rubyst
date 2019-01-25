enum Tree {
    Lit(f64),
    Cons(String, Box<Tree>, Box<Tree>),
}

// 行きがけ順
fn preorder(tree: &Tree) {
    match tree {
        Tree::Lit(_) => (),
        Tree::Cons(x, y, z) => {
            println!("{}", x);
            preorder(y);
            preorder(z);
        },
    }
}

// 帰りがけ順
fn postorder(tree: &Tree) {
    match tree {
        Tree::Lit(_) => (),
        Tree::Cons(x, y, z) => {
            postorder(y);
            postorder(z);
            println!("{}", x);
        },
    }
}

fn main() {
    let node = Tree::Cons(
                    String::from("節1"),
                    Box::new(Tree::Cons(
                        String::from("節2"),
                        Box::new(Tree::Cons(
                            String::from("葉A"),
                            Box::new(Tree::Lit(1.0_f64)),
                            Box::new(Tree::Lit(1.0_f64))
                        )),
                        Box::new(Tree::Cons(
                            String::from("葉B"),
                            Box::new(Tree::Lit(1.0_f64)),
                            Box::new(Tree::Lit(1.0_f64))
                        ))
                        )
                    ),
                    Box::new(Tree::Cons(
                        String::from("節3"),
                        Box::new(Tree::Cons(
                            String::from("葉C"),
                            Box::new(Tree::Lit(1.0_f64)),
                            Box::new(Tree::Lit(1.0_f64))
                        )),
                        Box::new(Tree::Cons(
                            String::from("葉D"),
                            Box::new(Tree::Lit(1.0_f64)),
                            Box::new(Tree::Lit(1.0_f64))
                        )),
                        )
                    )
                );
	preorder(&node);
    println!("");
    postorder(&node);
}

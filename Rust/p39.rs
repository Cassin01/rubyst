enum Tree {
    Nil,
    Cons(String, Box<Tree>, Box<Tree>),
}

fn preorder(tree: &Tree) {
    match tree {
        Tree::Nil => (),
        Tree::Cons(x, y, z) => {
            println!("{}", x);
            preorder(y);
            preorder(z);
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
                                Box::new(Tree::Nil),
                                Box::new(Tree::Nil)
                            )),
                            Box::new(Tree::Cons(
                                String::from("葉B"),
                                Box::new(Tree::Nil),
                                Box::new(Tree::Nil)
                            ))
                            )
                        ),
                        Box::new(Tree::Cons(
                            String::from("節3"),
                            Box::new(Tree::Cons(
                                String::from("葉C"),
                                Box::new(Tree::Nil),
                                Box::new(Tree::Nil)
                            )),
                            Box::new(Tree::Cons(
                                String::from("葉D"),
                                Box::new(Tree::Nil),
                                Box::new(Tree::Nil)
                            )),
                            )
                        )
                );
	preorder(&node);
}


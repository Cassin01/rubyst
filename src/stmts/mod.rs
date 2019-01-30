use super::tree::Tree;

// statements (複文)
#[derive(Debug, Clone)]
struct Stmts {
    stms: Vec<Tree>,
}
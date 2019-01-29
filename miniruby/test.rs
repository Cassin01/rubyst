fn main() {
    let i = String::from("unko unko");
    let mut c = i.chars();
    loop {
        if let Some(x) = c.next() {
            println!("{}", x);
        } else {
            break;
        }
    }
}

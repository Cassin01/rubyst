use is;
use std::iter::Peekable;
use std::str::Chars;

pub fn eat_condition(cs: &mut Peekable<Chars>) -> String {
    let mut condition = String::new();
    loop {
        if is::is_this(cs, &is::is_new_line) {
            cs.next();
            break;
        } else {
            if let Some(c) = cs.next() {
                condition.push(c);
            } else {
                panic!("there is no ");
            }
        }
    }
    condition
}

pub fn eat_in_if(cs: &mut Peekable<Chars>, if_num: &mut i64) -> String {
    let mut closure = String::new();
    let mut word = String::new();

    fn eat_and_flesh(cs: &mut Peekable<Chars>, closure: &mut String, word: &mut String) {
        if let Some(c) = cs.next() {
            word.push(c);
        } else {
            panic!("this is not space");
        }
        closure.push_str(&word);
        word.clear();
    }

    loop {
        if is::is_this(cs, &is::is_space) {
            if word == String::from("if") {
                *if_num += 1;
            } else if word == String::from("end") {
                *if_num -= 1;
                if *if_num == 0 {
                    cs.next();
                    return closure
                } else {
                    eat_and_flesh(cs, &mut closure, &mut word);
                }
            } else if word == String::from("else") {
                if *if_num == 1 {
                    cs.next();
                    return closure
                } else {
                    eat_and_flesh(cs, &mut closure, &mut word);
                }
            } else {
                eat_and_flesh(cs, &mut closure, &mut word);
            }
        } else if is::is_this(cs, &is::is_new_line) {
            if word == String::from("end") {
                *if_num -= 1;
                if *if_num == 0 {
                    cs.next();
                    return closure
                } else {
                    eat_and_flesh(cs, &mut closure, &mut word);
                }
            } else if word == String::from("else") {
                if *if_num == 1 {
                    cs.next();
                    return closure
                } else {
                    eat_and_flesh(cs, &mut closure, &mut word);
                }
            } else {
                eat_and_flesh(cs, &mut closure, &mut word);
            }
        } else {
            if let Some(c) = cs.next() {
                word.push(c);
            } else {
                if word == String::from("end") {
                    *if_num -= 1;
                    if *if_num == 0 {
                        cs.next();
                        return closure
                    } else {
                        panic!("code was end without 'end' idnetifer");
                    }
                } else {
                    panic!("code was end without 'end' idnetifer");
                }
            }
        }
    }
}

pub fn eat_codes_in_bracket(cs: &mut Peekable<Chars>) -> String {
    cs.next();
    let mut code_in_brackets = String::new();
    let mut bracket_num = 1;
    loop {
        if is::is_this(cs, &is::is_first_bracket) {
            bracket_num += 1;
        } else if is::is_this(cs, &is::is_second_bracket) {
            bracket_num -= 1;
            if bracket_num == 0 {
                break;
            }
        }
        if let Some(c) = cs.next() {
            code_in_brackets.push(c);
        } else {
            break;
        }
    }
    cs.next();
    code_in_brackets
}
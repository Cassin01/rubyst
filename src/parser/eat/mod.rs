use is;
use std::iter::Peekable;
use std::str::Chars;

pub fn eat_condition(cs: &mut Peekable<Chars>) -> String {
    let mut condition = String::new();
    loop {
        if is::is_this(cs, &is::is_new_line) {
            cs.next();
            return condition
        } else {
            if let Some(c) = cs.next() {
                condition.push(c);
            } else {
                return condition
            }
        }
    }
}


pub fn in_while(cs: &mut Peekable<Chars>) -> String {
    let mut closure = String::new();
    let mut word = String::new();
    let mut called_times = 1;
    let mut begin_times = 0;
    loop {
        if is::is_this(cs, &is::is_space) ||
            is:: is_this(cs, &is::is_new_line) {
            if word == String::from("if") || word == String::from("while") || word == String::from("begin") {
                if word == String::from("while") && begin_times > 0 {
                    begin_times -= 1;
                    called_times -= 1;
                } else if word == String::from("begin") {
                    begin_times += 1;
                }
                called_times += 1;
                eat_and_flesh(cs, &mut closure, &mut word);
            } else if word == String::from("end") {
                called_times -= 1;
                if called_times == 0 {
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
                    called_times -= 1;
                    if called_times == 0 {
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

pub fn in_if(cs: &mut Peekable<Chars>, if_num: &mut i64) -> String {
    let mut closure = String::new();
    let mut word = String::new();
    let mut begin_times = 0;

    loop {
        if is::is_this(cs, &is::is_space) || is::is_this(cs, &is::is_new_line){
            if word == String::from("if") || word == String::from("while") || word == String::from("begin") {
                if word == String::from("while") && begin_times > 0 {
                    begin_times -= 1;
                    *if_num -= 1;
                } else if word == String::from("begin") {
                    begin_times += 1;
                }
                *if_num += 1;
                eat_and_flesh(cs, &mut closure, &mut word);
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

fn eat_and_flesh(cs: &mut Peekable<Chars>, closure: &mut String, word: &mut String) {
    if let Some(c) = cs.next() {
        word.push(c);
    } else {
        panic!("this is not space");
    }
    closure.push_str(&word);
    word.clear();
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
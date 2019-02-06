pub fn is_num(c: &char) -> bool {
    match c {
        '1' ... '9' => true,
                _   => false
    }
}

pub fn is_alphabet(c: &char) -> bool {
    match c {
        'a' ... 'z' => true,
        '_'         => true,
        _           => false,
    }
}

pub fn is_space(c: &char) -> bool {
    match c {
        ' ' => true,
        _   => false,
    }
}

pub fn is_operator(c: &char) -> bool {
    match c {
        '*' => true,
        '/' => true,
        '%' => true,
        '+' => true,
        '=' => true,
        '!' => true,
        _   => false,
    }
}

pub fn is_first_bracket(c: &char) -> bool {
    match c {
        '(' => true,
        _   => false,
    }
}

pub fn is_second_bracket(c: &char) -> bool {
    match c {
        ')' => true,
        _   => false,
    }
}

pub fn is_new_line(c: &char) -> bool {
    match c {
        '\n' => true,
        _    => false,
    }
}

pub fn is_operator_sums(s: &String) -> bool {
    match s.as_str() {
        "+" => true,
        "-" => true,
        _   => false,
    }
}

pub fn is_operator_puroducts(s: &String) -> bool {
    match s.as_str() {
        "*" => true,
        "/" => true,
        "%" => true,
        _   => false,
    }
}

pub fn is_operator_pows(s: &String) -> bool {
    match s.as_str() {
        "**" => true,
        _    => false,
    }
}

pub fn is_operator_eqls(s: &String) -> bool {
    match s.as_str() {
        "==" => true,
        "!=" => true,
        _    => false,
    }
}

pub fn is_operator_assign(s: &String) -> bool {
    match s.as_str() {
        "="  => true,
        _    => false,
    }
}

pub fn is_reserved_word(s: &String) -> bool {
    match s.as_str() {
        "if" => true,
        _    => false,
    }
}
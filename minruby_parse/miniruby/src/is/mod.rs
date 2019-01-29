pub fn is_num(c: &char) -> bool {
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

pub fn is_space(c: &char) -> bool {
    match c {
        ' ' => true,
        _   => false,
    }
}

pub fn is_operator_sums(c: &char) -> bool {
    match c {
        '+' => true,
        '-' => true,
        _   => false,
    }
}

pub fn is_operator_products(c: &char) -> bool {
    match c {
        '*' => true,
        '/' => true,
        '%' => true,
        _   => false,
    }
}

pub fn is_operator(c: &char) -> bool {
    match c {
        '*' => true,
        '/' => true,
        '%' => true,
        '+' => true,
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
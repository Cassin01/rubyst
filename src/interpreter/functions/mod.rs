pub fn eq(l: i64, r: i64) -> bool {
    l == r
}

pub fn gt(l: i64, r: i64) -> bool {
    l > r
}

pub fn lt(l: i64, r: i64) -> bool {
    l < r
}

pub fn add(l: i64, r: i64) -> i64 {
    l + r
}

pub fn neg(l: i64, r: i64) -> i64 {
    l - r
}

pub fn mul(l: i64, r: i64) -> i64 {
    l * r
}

pub fn div(l: i64, r:i64) -> i64 {
    l / r
}

pub fn rem(l: i64, r:i64) -> i64 {
    l % r
}

pub fn pow(l: i64, r: i64) -> i64 {
    if r >= 0 {
        l.pow(r as u32)
    } else {
        1 / l.pow((r * -1) as u32)
    }
}
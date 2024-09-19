fn main() {
    digital_root(16);
}

fn digital_root(n: i64) -> i64 {
    let res = n
        .to_string()
        .chars()
        .fold(0, |acc, c| c.to_digit(10).expect("liars") + acc);

    if n > 9 {
        digital_root(res.into())
    } else {
        res.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(digital_root(16), 7);
        assert_eq!(digital_root(942), 6);
        assert_eq!(digital_root(132189), 6);
        assert_eq!(digital_root(493193), 2);
    }
}

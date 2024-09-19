fn main() {
    println!("Hello, world!");
}

fn descending_order(x: u64) -> u64 {
    let mut chars = x.to_string().chars().collect::<Vec<_>>();
    chars.sort();
    chars
        .iter()
        .rev()
        .fold(String::from(""), |acc, c| format!("{}{}", acc, c))
        .parse()
        .unwrap()
}

#[test]
fn returns_expected() {
    assert_eq!(descending_order(0), 0);
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);
}

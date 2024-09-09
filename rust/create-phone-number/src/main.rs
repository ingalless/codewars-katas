fn create_phone_number(numbers: &[u8]) -> String {
    let mut as_string: String = numbers
        .iter()
        .fold("".to_string(), |acc, n| format!("{}{}", acc, n.to_string()));
    as_string.insert(0, '(');
    as_string.insert(4, ')');
    as_string.insert(5, ' ');
    as_string.insert(9, '-');
    return as_string;
}

#[test]
fn returns_expected() {
    assert_eq!(
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
        "(123) 456-7890"
    );
    assert_eq!(
        create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
        "(111) 111-1111"
    );
    assert_eq!(
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]),
        "(123) 456-7899"
    );
}

fn main() {
    println!(
        "number {}",
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0])
    );
}

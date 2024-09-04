fn create_phone_number(numbers: &[u8]) -> String {
    let mut i: i8 = 0;
    let mut num = "".to_string();
    for n in numbers {
        let r = match i {
            0 => format!("(aaa123{}", n),
            _ => i.to_string(),
        };
        i += 1;
        println!("{} - {}", n, r);
        num = format!("{}{}", num, r);
    }
    return "".into();
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

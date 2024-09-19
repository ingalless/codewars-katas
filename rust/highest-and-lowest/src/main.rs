fn main() {
    println!("{:?}", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}

fn high_and_low(numbers: &str) -> String {
    let as_nums = numbers
        .split(" ")
        .map(|n| n.parse::<i32>().expect("Got a non-number"));

    format!(
        "{} {}",
        as_nums.clone().max().expect("Uh oh"),
        as_nums.clone().min().expect("Uh oh.")
    )
}

#[test]
fn example_test_1() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}

#[test]
fn example_test_2() {
    assert_eq!("3 1", high_and_low("1 2 3"));
}

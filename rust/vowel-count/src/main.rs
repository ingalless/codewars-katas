fn get_count(string: &str) -> usize {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    string.to_lowercase().chars().fold(
        0,
        |acc, curr| {
            if vowels.contains(&curr) {
                acc + 1
            } else {
                acc
            }
        },
    )
}

#[test]
fn my_tests() {
    assert_eq!(get_count("abracadabra"), 5);
}

fn main() {
    println!("Hello, world!");
}

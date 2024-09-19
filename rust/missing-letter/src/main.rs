fn find_missing_letter(chars: &[char]) -> char {
    // The random tests give us any unicode sequence,
    // contrary to the kata's description. Oh well.
    // It was quite good fun to work this one out,
    // and actually resulted in a better solution!
    *(*chars.first().unwrap() as u32..=*chars.last().unwrap() as u32)
        .zip(chars.iter().map(|c| *c as u32).collect::<Vec<_>>())
        .filter(|i| i.0 != i.1)
        .map(|v| char::from_u32(v.0).expect("Not a char, rip"))
        .collect::<Vec<_>>()
        .first()
        .expect("Received `chars` were a valid sequence")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'f']), 'e');
        assert_eq!(find_missing_letter(&['O', 'Q', 'R', 'S']), 'P');
    }

    #[test]
    #[should_panic]
    fn valid_sequence_test() {
        find_missing_letter(&['a', 'b', 'c']);
    }
}

fn main() {
    println!("{:?}", find_missing_letter(&['a', 'b', 'c', 'd', 'f']));
}

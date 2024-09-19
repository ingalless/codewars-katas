fn main() {
    is_pangram("The quick, brown fox jumps over the lazy dog!");
}

fn is_pangram(s: &str) -> bool {
    let target: u32 = "abcdefghijklmnopqrstuvwxyz".chars().map(|c| c as u32).sum();
    let mut chars = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c as u32)
        .collect::<Vec<_>>();
    chars.sort();
    chars.dedup();
    target <= chars.iter().sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::is_pangram;

    fn dotest(s: &str, expected: bool) {
        let actual = is_pangram(s);
        assert!(
            actual == expected,
            "Test failed with s = \"{s}\"\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest("The quick, brown fox jumps over the lazy dog!", true);
        dotest("Cwm fjord bank glyphs vext quiz", true);
        dotest("Pack my box with five dozen liquor jugs.", true);
        dotest("How quickly daft jumping zebras vex.", true);
        dotest("ABCD45EFGH,IJK,LMNOPQR56STUVW3XYZ", true);
        dotest("This isn't a pangram!", false);
        dotest("abcdefghijklmopqrstuvwxyz", false);
        dotest("Aacdefghijklmnopqrstuvwxyz", false);
    }
}

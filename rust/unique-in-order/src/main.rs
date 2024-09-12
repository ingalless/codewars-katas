fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    sequence.into_iter().fold(Vec::new(), |mut acc, item| {
        match acc.last() {
            None => acc.push(item),
            Some(last) => {
                if item != *last {
                    acc.push(item);
                }
            }
        };
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chars() {
        assert_eq!(
            unique_in_order("AAAABBBCCDAABBB".chars()),
            vec!['A', 'B', 'C', 'D', 'A', 'B']
        );
    }

    #[test]
    fn numbers() {
        assert_eq!(unique_in_order(vec![1, 1, 4, 3, 2]), vec![1, 4, 3, 2]);
    }
}

fn main() {
    println!("Hello, world!");
}

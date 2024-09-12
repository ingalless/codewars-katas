use std::collections::HashMap;

/// Converts a number to a string representating roman numeral.
/// Symbol     Value
/// I          1
/// V          5
/// X          10
/// L          50
/// C          100
/// D          500
/// M          1,000
///
/// The numerals for 4 (IV) and 9 (IX) are written using subtractive notation,
/// where the smaller symbol (I) is subtracted from the larger one (V, or X), thus avoiding the clumsier IIII and VIIII.
/// Subtractive notation is also used for 40 (XL), 90 (XC), 400 (CD) and 900 (CM).[6] These are the only subtractive forms in standard use.

fn num_as_roman(num: i32) -> String {
    try_subtract(num, "".into())
}

fn try_subtract(num: i32, as_roman: String) -> String {
    if num - 1000 >= 0 {
        return try_subtract(num - 1000, format!("{}M", as_roman));
    } else if num - 900 >= 0 {
        return try_subtract(num - 900, format!("{}CM", as_roman));
    } else if num - 500 >= 0 {
        return try_subtract(num - 500, format!("{}D", as_roman));
    } else if num - 400 >= 0 {
        return try_subtract(num - 400, format!("{}CD", as_roman));
    } else if num - 100 >= 0 {
        return try_subtract(num - 100, format!("{}C", as_roman));
    } else if num - 90 >= 0 {
        return try_subtract(num - 90, format!("{}XC", as_roman));
    } else if num - 50 >= 0 {
        return try_subtract(num - 50, format!("{}L", as_roman));
    } else if num - 40 >= 0 {
        return try_subtract(num - 40, format!("{}XL", as_roman));
    } else if num - 10 >= 0 {
        return try_subtract(num - 10, format!("{}X", as_roman));
    } else if num - 9 >= 0 {
        return try_subtract(num - 9, format!("{}IX", as_roman));
    } else if num - 5 >= 0 {
        return try_subtract(num - 5, format!("{}V", as_roman));
    } else if num - 4 >= 0 {
        return try_subtract(num - 4, format!("{}IV", as_roman));
    } else if num - 1 >= 0 {
        return try_subtract(num - 1, format!("{}I", as_roman));
    }

    as_roman
}

#[test]
fn returns_expected() {
    // Basics
    assert_eq!(num_as_roman(1), "I");
    assert_eq!(num_as_roman(2), "II");
    assert_eq!(num_as_roman(3), "III");
    assert_eq!(num_as_roman(1000), "M");
    assert_eq!(num_as_roman(1001), "MI");
    assert_eq!(num_as_roman(900), "CM");
    assert_eq!(num_as_roman(500), "D");
    assert_eq!(num_as_roman(100), "C");
    assert_eq!(num_as_roman(50), "L");
    assert_eq!(num_as_roman(10), "X");
    assert_eq!(num_as_roman(5), "V");
    assert_eq!(num_as_roman(4), "IV");

    // Combinatory
    assert_eq!(num_as_roman(182), "CLXXXII");
    assert_eq!(num_as_roman(1990), "MCMXC");
    assert_eq!(num_as_roman(1875), "MDCCCLXXV");
    assert_eq!(num_as_roman(1666), "MDCLXVI");
    assert_eq!(num_as_roman(3999), "MMMCMXCIX");
}

fn main() {
    println!("{:?}", num_as_roman(182));
}

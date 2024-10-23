use std::{cmp::Ordering, collections::HashMap};

struct Solution;

impl Solution {
    /// Runtime: 0ms    | 100.00%
    /// Memory : 2.14MB |  19.50%
    pub fn int_to_roman(mut num: i32) -> String {
        let mut roman: HashMap<i32, &str> = HashMap::new();
        roman.insert(1, "I");
        roman.insert(4, "IV");
        roman.insert(5, "V");
        roman.insert(9, "IX");
        roman.insert(10, "X");
        roman.insert(40, "XL");
        roman.insert(50, "L");
        roman.insert(90, "XC");
        roman.insert(100, "C");
        roman.insert(400, "CD");
        roman.insert(500, "D");
        roman.insert(900, "CM");
        roman.insert(1000, "M");
        let mut entries: Vec<_> = roman
            .iter()
            .filter_map(|(key, _)| match num.cmp(key) {
                Ordering::Less => None,
                _ => Some(key),
            })
            .collect();
        entries.sort_by(|a, b| b.cmp(a));

        let mut result = String::new();
        entries.into_iter().for_each(|v| {
            while num >= *v {
                result.push_str(roman.get(v).unwrap());
                num -= v;
            }
        });

        result
    }
}

#[test]
fn test_integer_to_roman_1() {
    assert_eq!(String::from("MMMDCCXLIX"), Solution::int_to_roman(3749));
}

#[test]
fn test_integer_to_roman_2() {
    assert_eq!(String::from("LVIII"), Solution::int_to_roman(58));
}

#[test]
fn test_integer_to_roman_3() {
    assert_eq!(String::from("MCMXCIV"), Solution::int_to_roman(1994));
}

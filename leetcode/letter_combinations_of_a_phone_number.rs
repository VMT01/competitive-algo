struct Solution;

impl Solution {
    /// Memory : 0ms    | 100.00%
    /// Runtime: 2.24MB |  52.41%
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }

        let map = vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut results = Vec::new();

        fn construct_combination(
            combination: String,
            next_digit: &str,
            map: &Vec<&str>,
            results: &mut Vec<String>,
        ) {
            if next_digit.is_empty() {
                results.push(combination);
                return;
            }

            let index = (next_digit.as_bytes()[0] - b'2') as usize;
            let map_bytes = map[index].as_bytes();

            for &byte in map_bytes {
                let mut new_combination = combination.clone();
                new_combination.push(byte as char);
                construct_combination(new_combination, &next_digit[1..], map, results);
            }
        }

        construct_combination(String::new(), &digits, &map, &mut results);
        results
    }
}

#[test]
fn test_letter_combinations_1() {
    assert_eq!(
        Solution::letter_combinations(String::from("23")),
        vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    );
}

#[test]
fn test_letter_combinations_2() {
    assert_eq!(
        Vec::<String>::new(),
        Solution::letter_combinations(String::from(""))
    );
}

#[test]
fn test_letter_combinations_3() {
    assert_eq!(
        Solution::letter_combinations(String::from("2")),
        vec!["a", "b", "c"]
    );
}

struct Solution;

impl Solution {
    /// Runtime: 0ms     | 100.00%
    /// Memory : 2.24 MB |  64.81%
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();
        Self::recursive(String::with_capacity(n as usize * 2), n, n, &mut results);

        results
    }

    fn recursive(parenthese: String, open: i32, close: i32, results: &mut Vec<String>) {
        if open == 0 && close == 0 {
            results.push(parenthese);
            return;
        }

        if open > 0 {
            Self::recursive(parenthese.clone() + "(", open - 1, close, results);
        }
        if close > open {
            Self::recursive(parenthese + ")", open, close - 1, results);
        }
    }
}

#[test]
fn test_generate_parentheses_1() {
    assert_eq!(
        vec![
            String::from("((()))"),
            String::from("(()())"),
            String::from("(())()"),
            String::from("()(())"),
            String::from("()()()")
        ],
        Solution::generate_parenthesis(3)
    )
}

#[test]
fn test_generate_parentheses_2() {
    assert_eq!(vec![String::from("()")], Solution::generate_parenthesis(1));
}

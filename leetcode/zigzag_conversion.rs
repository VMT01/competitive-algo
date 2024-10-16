use super::Solution;

#[allow(dead_code)]
impl Solution {
    /// Runtime: 0ms     | 100.00%
    /// Memory : 2.24 MB |  47.95%
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }

        let mut rows: Vec<String> = vec![String::new(); num_rows as usize];
        let mut current_line = 0;
        let mut reach_edge = true;

        s.chars().for_each(|c| {
            if current_line == 0 || current_line == num_rows - 1 {
                reach_edge = !reach_edge;
            }

            unsafe {
                rows.get_unchecked_mut(current_line as usize).push(c);
            }

            if reach_edge {
                current_line -= 1;
            } else {
                current_line += 1;
            }
        });

        rows.join("")
    }
}

#[test]
fn test_zigzag_conversion_1() {
    assert_eq!(
        Solution::convert(String::from("PAYPALISHIRING"), 3),
        String::from("PAHNAPLSIIGYIR")
    )
}

#[test]
fn test_zigzag_conversion_2() {
    assert_eq!(
        Solution::convert(String::from("PAYPALISHIRING"), 4),
        String::from("PINALSIGYAHRPI")
    )
}

#[test]
fn test_zigzag_conversion_3() {
    assert_eq!(Solution::convert(String::from("A"), 1), String::from("A"))
}

#[test]
fn test_zigzag_conversion_4() {
    assert_eq!(Solution::convert(String::from("AB"), 1), String::from("AB"))
}

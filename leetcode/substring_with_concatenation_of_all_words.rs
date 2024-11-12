use std::collections::HashMap;

struct Solution;

impl Solution {
    /// Runtime: 5ms    | 85.71%
    /// Memory : 2.4 MB | 37.23%
    // TODO: Optimize this to 0ms runtime
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if words.is_empty() || s.is_empty() || words[0].len() > s.len() {
            return Vec::with_capacity(0);
        }

        let word_size = words[0].len();
        let words_size = words.len();
        let mut word_map = HashMap::new();
        for word in words {
            *word_map.entry(word).or_insert(0) += 1;
        }

        let mut results: Vec<i32> = Vec::new();

        for i in 0..word_size {
            let mut count = 0;
            let mut left = i;
            let mut window_map = HashMap::new();

            for j in (i..=s.len() - word_size).step_by(word_size) {
                let word = &s[j..j + word_size];

                if word_map.contains_key(word) {
                    let entry = window_map.entry(word).or_insert(0);
                    *entry += 1;
                    count += 1;

                    while window_map[word] > word_map[word] {
                        let left_word = &s[left..left + word_size];
                        *window_map.get_mut(left_word).unwrap() -= 1;
                        count -= 1;
                        left += word_size;
                    }

                    if count == words_size {
                        results.push(left as i32);
                    }
                } else {
                    count = 0;
                    left = j + word_size;
                    window_map.clear();
                }
            }
        }

        results
    }
}

#[test]
fn test_substring_with_concatenation_of_all_words_1() {
    assert_eq!(
        vec![0, 9],
        Solution::find_substring(
            String::from("barfoothefoobarman"),
            vec![String::from("foo"), String::from("bar")]
        )
    )
}

#[test]
fn test_substring_with_concatenation_of_all_words_2() {
    assert_eq!(
        Vec::<i32>::new(),
        Solution::find_substring(
            String::from("wordgoodgoodgoodbestword"),
            vec![
                String::from("word"),
                String::from("good"),
                String::from("best"),
                String::from("word")
            ]
        )
    )
}

#[test]
fn test_substring_with_concatenation_of_all_words_3() {
    assert_eq!(
        vec![6, 9, 12],
        Solution::find_substring(
            String::from("barfoofoobarthefoobarman"),
            vec![
                String::from("bar"),
                String::from("foo"),
                String::from("the"),
            ]
        )
    )
}

use super::Solution;

#[allow(dead_code)]
impl Solution {
    /// Runtime: 2ms     | 90.51%
    /// Memory : 2.15 MB | 65.81%
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();

        assert!(n > 0);

        // Vec qui contient les palindromes trouvés
        let mut candidates: Vec<(usize, usize, bool)> = Vec::with_capacity(n);

        // Au début, les candidats sont formés par les suites de lettres identiques
        let chars: Vec<char> = s.chars().collect();
        let mut current_char = chars[0];
        let mut start = 0;

        for (i, &c) in chars.iter().enumerate() {
            if c != current_char {
                candidates.push((start, i - 1, true));
                start = i;
                current_char = c;
            }
        }
        // Pousser la dernière séquence
        candidates.push((start, chars.len() - 1, true));

        // Reparcours la liste des candidats pour trouver la plus longue suite de caractères identiques
        // (= plus grand palindrome pour l'instant)
        let mut max_len: usize = 0;
        for (beg, end, _) in candidates.iter() {
            let l = end - beg + 1;
            if l > max_len {
                max_len = l;
            }
        }

        // Cherche à faire grandir les palindromes
        let mut idx: usize = 0;
        let mut actives: bool = false;

        loop {
            let (beg, end, active) = candidates[idx];
            let l = end - beg + 1;

            if !active && l < max_len {
                // Les candidats inactifs dont la longueur est inférieure à la longueur maximale peuvent
                // être oubliés
                candidates.swap_remove(idx);
                actives = true;
            } else if active {
                if beg == 0 || end == n - 1 {
                    // Si on atteint un bout de la chaîne de caractères, le palindrome ne peut plus grandir
                    candidates[idx] = (beg, end, false);
                    actives = true;
                } else {
                    // Sinon on teste si les caractères à gauche et à droite coïncident
                    if chars[beg - 1] == chars[end + 1] {
                        // Si c'est bon, le palindrome grossit
                        candidates[idx] = (beg - 1, end + 1, true);
                        actives = true;
                        let l = end - beg + 3;
                        if max_len < l {
                            max_len = l;
                        }
                    } else {
                        // Sinon le palindrome ne peut plus grossir
                        candidates[idx] = (beg, end, false);
                    }
                }
                idx += 1;
            } else {
                idx += 1;
            }

            if idx >= candidates.len() {
                if actives {
                    idx = 0;
                    actives = false;
                } else {
                    break;
                }
            }
        }

        let (beg, end, _) = candidates[0];

        s[beg..end + 1].to_string()
    }
}

#[test]
fn test_longest_palindromic_substring_1() {
    assert_eq!(
        Solution::longest_palindrome(String::from("babad")),
        String::from("bab")
    )
}

// #[test]
// fn test_longest_palindromic_substring_2() {
//     assert_eq!(
//         Solution::longest_palindrome(String::from("cbbd")),
//         String::from("bb")
//     )
// }

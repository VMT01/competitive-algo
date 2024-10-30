struct Solution;

impl Solution {
    /// Runtime: 2ms    | 100.00%
    /// Memory : 2.22MB |  20.00%
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return Vec::new();
        }

        let mut nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
        nums.sort_unstable();
        let target = target as i64;
        let mut results = Vec::new();

        for a in 0..nums.len() - 3 {
            if nums[a] * 4 > target {
                break;
            }

            if a > 0 && nums[a] == nums[a - 1] {
                continue;
            }

            for b in a + 1..nums.len() - 2 {
                if nums[b] * 3 > target - nums[a] {
                    continue;
                }
                if b > a + 1 && nums[b] == nums[b - 1] {
                    continue;
                }

                let mut c = b + 1;
                let mut d = nums.len() - 1;

                while c < d {
                    let sum = nums[a] + nums[b] + nums[c] + nums[d];

                    if sum > target {
                        d -= 1;
                    } else if sum < target {
                        c += 1;
                    } else {
                        results.push(vec![
                            nums[a] as i32,
                            nums[b] as i32,
                            nums[c] as i32,
                            nums[d] as i32,
                        ]);
                        c += 1;
                        d -= 1;

                        while nums[c] == nums[c - 1] && c < d {
                            c += 1;
                        }
                        while nums[d] == nums[d + 1] && c < d {
                            d -= 1;
                        }
                    }
                }
            }
        }

        results
    }
}

#[test]
fn test_four_sum_1() {
    assert_eq!(
        Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
        vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
    );
}

#[test]
fn test_four_sum_2() {
    assert_eq!(
        Solution::four_sum(vec![2, 2, 2, 2, 2], 8),
        vec![vec![2, 2, 2, 2]]
    );
}

#[test]
fn test_four_sum_3() {
    assert_eq!(
        Solution::four_sum(
            vec![
                -497, -480, -477, -470, -452, -448, -440, -412, -390, -381, -372, -372, -369, -366,
                -355, -346, -340, -337, -322, -321, -311, -296, -258, -249, -248, -232, -215, -199,
                -174, -154, -128, -122, -122, -117, -115, -113, -110, -89, -86, -84, -78, -71, -69,
                -53, -49, -35, -25, -21, -7, 3, 7, 21, 25, 30, 47, 52, 81, 84, 87, 91, 96, 157,
                161, 167, 178, 184, 210, 217, 228, 236, 274, 277, 283, 286, 290, 301, 302, 341,
                352, 354, 361, 367, 384, 390, 412, 421, 458, 468, 483, 484, 486, 487, 490, 491
            ],
            8377
        ),
        Vec::<Vec<i32>>::new()
    );
}

#[test]
fn test_four_sum_4() {
    assert_eq!(
        Solution::four_sum(
            vec![1000000000, 1000000000, 1000000000, 1000000000],
            -294967296
        ),
        Vec::<Vec<i32>>::new()
    );
}

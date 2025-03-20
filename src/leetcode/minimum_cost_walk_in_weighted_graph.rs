//! 3108. \[**Hard**\] [Minimum Cost Walk in Weighted Graph](https://leetcode.com/problems/minimum-cost-walk-in-weighted-graph)
//!
//! - `Array`
//! - `Bit Manipulation`
//! - `Union Find`
//! - `Graph`
//!
//! cargo test ::minimum_cost_walk_in_weighted_graph
//!
//! Runtime: 11ms     | Beats 100.00%
//! Memory : 14.82 MB | Beats 100.00%
//!
//! Using DSU (Disjoint Set) algorithm to find connected graph.
//! If no path found, the result must be -1
//! If path found, the result is accumulated by back-tracking when constructing spanning tree.

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;

        let mut parent = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
        }
        let mut rank = vec![0; n];
        let mut min_path_cost = vec![-1; n];

        fn find_root(parent: &mut Vec<usize>, node: usize) -> usize {
            if parent[node] != node {
                parent[node] = find_root(parent, parent[node]);
            }
            parent[node]
        }

        for edge in edges {
            let [source, target, weight, ..] = edge.as_slice() else {
                unreachable!()
            };
            let (source, target) = (*source as usize, *target as usize);

            let source_root = find_root(&mut parent, source);
            let target_root = find_root(&mut parent, target);
            min_path_cost[source_root] &= weight;
            min_path_cost[target_root] &= weight;

            if source_root == target_root {
                continue;
            }

            match rank[source_root].cmp(&rank[target_root]) {
                std::cmp::Ordering::Less => {
                    min_path_cost[target_root] &= min_path_cost[source_root];
                    parent[source_root] = target_root;
                }
                std::cmp::Ordering::Equal => {
                    min_path_cost[source_root] &= min_path_cost[target_root];
                    parent[target_root] = source_root;
                    rank[source_root] += 1;
                }
                std::cmp::Ordering::Greater => {
                    min_path_cost[source_root] &= min_path_cost[target_root];
                    parent[target_root] = source_root;
                }
            }
        }

        let mut result = Vec::with_capacity(query.len());
        for q in query {
            let [start, end, ..] = q.as_slice() else {
                unreachable!()
            };
            let (start, end) = (*start as usize, *end as usize);

            if start == end {
                result.push(0);
            } else {
                let start_root = find_root(&mut parent, start);
                let end_root = find_root(&mut parent, end);

                if start_root != end_root {
                    result.push(-1);
                } else {
                    result.push(min_path_cost[start_root]);
                }
            }
        }

        result
    }
}

#[test]
fn minimum_cost_walk_in_weighted_graph_case_1() {
    assert_eq!(
        Solution::minimum_cost(
            5,
            vec![vec![0, 1, 7], vec![1, 3, 7], vec![1, 2, 1]],
            vec![vec![0, 3], vec![3, 4]]
        ),
        vec![1, -1]
    )
}

#[test]
fn minimum_cost_walk_in_weighted_graph_case_2() {
    assert_eq!(
        Solution::minimum_cost(
            3,
            vec![vec![0, 2, 7], vec![0, 1, 15], vec![1, 2, 6], vec![1, 2, 1]],
            vec![vec![1, 2]]
        ),
        vec![0]
    )
}

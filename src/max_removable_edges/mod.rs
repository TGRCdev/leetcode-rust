//! https://leetcode.com/problems/remove-max-number-of-edges-to-keep-graph-fully-traversable

pub struct Solution;

#[derive(Debug, Clone)]
pub struct DisjointSet {
    parent: Vec<usize>,
    size: usize,
}

impl DisjointSet {
    pub fn find(&mut self, i: usize) -> usize {
        assert!(i < self.size);

        if self.parent[i] == i {
            return i;
        }
        else {
            // Rebalance
            let result = self.find(self.parent[i]);
            self.parent[i] = result;
            return result;
        }
    }

    pub fn union(&mut self, i: usize, j: usize) {
        assert!(i < self.size);
        assert!(j < self.size);

        let irep = self.find(i);
        let jrep = self.find(j);

        self.parent[irep] = jrep;
    }

    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            size,
        }
    }

    pub fn count_representatives(&self) -> usize {
        self.parent.iter().enumerate()
            .map(|(i, &parent)| (i == parent) as usize)
            .sum()
    }

    pub fn is_entirely_connected(&self) -> bool {
        self.count_representatives() <= 1
    }
}

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        const BOB_EDGE: usize = 0;
        const ALICE_EDGE: usize = 1;
        const BOTH_EDGE: usize = 2;

        let mut edge_vecs = [
            vec![],
            vec![],
        ];

        let mut both_djset = DisjointSet::new(n as usize);
        let mut removable_paths = 0;

        // Step 1: Associate nodes with BOTH_EDGE type edges
        // Store Bob/Alice edges for later
        for edge in edges {
            let edge_type = edge[0] as usize - 1;
            let from = edge[1] as usize - 1;
            let to = edge[2] as usize - 1;

            if edge_type == BOTH_EDGE {
                if both_djset.find(from) == both_djset.find(to) {
                    //println!("USELESS BOTH EDGE");
                    removable_paths += 1;
                }
                else {
                    both_djset.union(from, to);
                }
            }
            else {
                edge_vecs[edge_type].push((from, to));
            }
        }

        //println!("Remaining representatives: {}", both_djset.count_representatives()jk);
        //println!("Remaining edges: {}  Bob, {} Alice", edge_vecs[0].len(), edge_vecs[1].len());

        // Step 2: Check that the chart is entirely connected
        // (If this is true, all other edges can be discarded)
        if both_djset.is_entirely_connected() {
            return removable_paths + (edge_vecs[0].len() + edge_vecs[1].len()) as i32;
        }

        let mut extend_djset_for_person = |edges: &Vec<(usize, usize)>| -> Result<(),()> {
            let mut person_djset = both_djset.clone();
            for &(from, to) in edges {
                let (from_rep, to_rep) = (person_djset.find(from), person_djset.find(to));
                if from_rep == to_rep {
                    // Both nodes are already reachable through the graph
                    // Useless edge
                    removable_paths += 1;
                    //println!("Removing path from {from} (rep {from_rep}) to {to} (rep {to_rep})")
                }
                else {
                    //println!("Union from {from} (oldrep {from_rep}) to {to} (oldrep {to_rep})");
                    person_djset.union(from, to);
                }
            }

            // Check if graph is traversible
            if !person_djset.is_entirely_connected() {
                return Err(());
            }

            Ok(())
        };

        // Step 3: Create a disjoint set including Bob's paths
        if let Err(()) = extend_djset_for_person(&edge_vecs[BOB_EDGE]) {
            // Bob can't traverse the entire graph
            //println!("Bob is incapable of traversing");
            return -1;
        }

        // Step 4: Create a disjoint set including Alice's paths
        if let Err(()) = extend_djset_for_person(&edge_vecs[ALICE_EDGE]) {
            // Bob can't traverse the entire graph
            //println!("Alice is incapable of traversing");
            return -1;
        }

        return removable_paths;
    }
}

#[test]
fn test() {
    fn test_max_num_edges(n: i32, edges: &[[i32; 3]], expected: i32) {
        use std::time::Instant;
        let edgevec = edges.into_iter().map(|edge| edge.to_vec()).collect();
        let start = Instant::now();
        let result = Solution::max_num_edges_to_remove(n, edgevec);
        let dur = Instant::now() - start;
        println!("n = {n}, edges: = [...; {}]\nResult: {result} (Expected: {expected})", edges.len());
        println!("Took {} millis ({} micros)", dur.as_millis(), dur.as_micros());
        assert_eq!(result, expected);
    }

    test_max_num_edges(4, &[[3,1,2],[3,2,3],[1,1,3],[1,2,4],[1,1,2],[2,3,4]], 2);
    test_max_num_edges(
        13,
        &[[1,1,2],[2,1,3],[3,2,4],[3,2,5],[1,2,6],[3,6,7],[3,7,8],[3,6,9],[3,4,10],[2,3,11],[1,5,12],[3,3,13],[2,1,10],[2,6,11],[3,5,13],[1,9,12],[1,6,8],[3,6,13],[2,1,4],[1,1,13],[2,9,10],[2,1,6],[2,10,13],[2,2,9],[3,4,12],[2,4,7],[1,1,10],[1,3,7],[1,7,11],[3,3,12],[2,4,8],[3,8,9],[1,9,13],[2,4,10],[1,6,9],[3,10,13],[1,7,10],[1,1,11],[2,4,9],[3,5,11],[3,2,6],[2,1,5],[2,5,11],[2,1,7],[2,3,8],[2,8,9],[3,4,13],[3,3,8],[3,3,11],[2,9,11],[3,1,8],[2,1,8],[3,8,13],[2,10,11],[3,1,5],[1,10,11],[1,7,12],[2,3,5],[3,1,13],[2,4,11],[2,3,9],[2,6,9],[2,1,13],[3,1,12],[2,7,8],[2,5,6],[3,1,9],[1,5,10],[3,2,13],[2,3,6],[2,2,10],[3,4,11],[1,4,13],[3,5,10],[1,4,10],[1,1,8],[3,3,4],[2,4,6],[2,7,11],[2,7,10],[2,3,12],[3,7,11],[3,9,10],[2,11,13],[1,1,12],[2,10,12],[1,7,13],[1,4,11],[2,4,5],[1,3,10],[2,12,13],[3,3,10],[1,6,12],[3,6,10],[1,3,4],[2,7,9],[1,3,11],[2,2,8],[1,2,8],[1,11,13],[1,2,13],[2,2,6],[1,4,6],[1,6,11],[3,1,2],[1,1,3],[2,11,12],[3,2,11],[1,9,10],[2,6,12],[3,1,7],[1,4,9],[1,10,12],[2,6,13],[2,2,12],[2,1,11],[2,5,9],[1,3,8],[1,7,8],[1,2,12],[1,5,11],[2,7,12],[3,1,11],[3,9,12],[3,2,9],[3,10,11]],
        114,
    );
    test_max_num_edges(
        66666,
        include!("long_testcase.txt"),
        0
    );
}
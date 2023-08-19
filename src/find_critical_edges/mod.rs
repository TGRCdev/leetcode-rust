//! https://leetcode.com/problems/find-critical-and-pseudo-critical-edges-in-minimum-spanning-tree/description/

pub struct Solution;

pub struct DisjointSet {
    parent: Vec<usize>,
}

#[derive(Clone)]
pub struct Edge {
    num: usize,
    i: i32,
    j: i32,
    weight: i32,
}

impl std::fmt::Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("<{},{}>", self.num, self.weight))
    }
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect()
        }
    }

    pub fn find(&self, i: usize) -> usize {
        assert!(i < self.parent.len());
        if self.parent[i] == i {
            return i;
        }
        else {
            return self.find(self.parent[i]);
        }
    }

    pub fn find_and_optimize(&mut self, i: usize) -> usize {
        assert!(i < self.parent.len());
        if self.parent[i] == i {
            return i;
        }
        else {
            let rep = self.find_and_optimize(self.parent[i]);
            self.parent[i] = rep;
            return rep;
        }
    }

    /// Associates i and j.
    /// Returns false if the edge is redundant, true otherwise.
    pub fn join(&mut self, i: usize, j: usize) -> bool {
        let i_rep = self.find_and_optimize(i);
        let j_rep = self.find_and_optimize(j);
        if i_rep == j_rep {
            return false;
        }
        else {
            self.parent[i_rep] = j_rep;
            self.parent[i] = j_rep;
            return true;
        }
    }
}

use std::collections::HashSet;
impl Solution {
    pub fn find_mst(n: i32, edges: &Vec<Edge>) -> (i32, HashSet<Edge>) {
        let n = n as usize;
        let mut weight = 0;
        let mut mst = Vec::with_capacity(n-1);
        let mut dset = DisjointSet::new(n);

        edges.into_iter().for_each(|edge| {
            if dset.join(edge.i as usize, edge.j as usize) {
                mst.push(edge.clone());
                weight += edge.weight;
            }
        });

        (weight, mst)
    }

    pub fn find_critical_and_pseudo_critical_edges(n: i32, mut edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut edges: Vec<Edge> = edges.into_iter().enumerate().map(|(i, edge)| {
            Edge {
                num: i,
                i: edge[0],
                j: edge[1],
                weight: edge[2],
            }
        }).collect();
        edges.sort_unstable_by_key(|edge| edge.weight);
        let (base_weight, base_mst) = Self::find_mst(n, &edges);

        println!("Weight: {base_weight}\nMST: {base_mst:?}");

        let mut critical = HashSet::new();
        let mut pseudocritical = HashSet::from_iter

        for i in 0..edges.len() {
            let mut edges = edges.clone();
            let old_edge = edges.remove(i);
            let (weight, mst) = Self::find_mst(n, &edges);
            println!("After removing edge {old_edge:?}: Weight: {weight} - MST: {mst:?}");
        }

        todo!()
    }
}

#[test]
fn test_critical_edges() {
    let mut _test = 1;
    let mut test_case = |n: i32, edges: &[[i32;3]], expected: [&[i32]; 2]| {
        let edges = edges.into_iter().map(|edge| edge.to_vec()).collect();
        let result = Solution::find_critical_and_pseudo_critical_edges(n, edges);
        println!("Test {_test}: {}", if result == expected {"SUCCESS"} else {"FAILIURE"});
        _test += 1;
    };

    test_case(
        5,
        &[[0,1,1],[1,2,1],[2,3,2],[0,3,2],[0,4,3],[3,4,3],[1,4,6]],
        [&[],&[]]
    );
}
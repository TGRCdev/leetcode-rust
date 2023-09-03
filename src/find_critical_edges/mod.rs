//! https://leetcode.com/problems/find-critical-and-pseudo-critical-edges-in-minimum-spanning-tree/description/

pub struct Solution;

pub struct DisjointSet {
    parent: Vec<usize>,
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

    pub fn single_set(&self) -> bool {
        let rep = self.find(0);
        self.parent.iter().skip(1).all(|&parent| self.find(parent) == rep)
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

use std::collections::HashSet;
impl Solution {
    pub fn find_mst_force_edge(n: i32, edges: &Vec<Edge>, force_edge: Option<usize>) -> Option<(i32, HashSet<Edge>)> {
        let n = n as usize;
        let mut weight = 0;
        let mut mst = HashSet::with_capacity(n-1);
        let mut dset = DisjointSet::new(n);

        if let Some(edge) = force_edge {
            let edge = &edges[edge];
            dset.join(edge.i as usize, edge.j as usize);
            weight += edge.weight;
            mst.insert(edge.clone());
        }

        edges.into_iter().for_each(|edge| {
            if dset.join(edge.i as usize, edge.j as usize) {
                mst.insert(edge.clone());
                weight += edge.weight;
            }
        });

        if dset.single_set() {Some((weight, mst))} else {None}
    }

    pub fn find_mst(n: i32, edges: &Vec<Edge>) -> Option<(i32, HashSet<Edge>)> {
        Self::find_mst_force_edge(n, edges, None)
    }

    pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut edges: Vec<Edge> = edges.into_iter().enumerate().map(|(i, edge)| {
            Edge {
                num: i,
                i: edge[0],
                j: edge[1],
                weight: edge[2],
            }
        }).collect();
        edges.sort_unstable_by_key(|edge| edge.weight);
        let (base_weight, base_mst) = Self::find_mst(n, &edges).expect("Graph has no MST");

        println!("Weight: {base_weight}\nMST: {base_mst:?}");

        let mut critical = base_mst;
        let mut pseudocritical = HashSet::new();

        for i in 0..edges.len() {
            let mut edges = edges.clone();
            // Test for pseudocritical edge
            {
                let (weight, _) = Self::find_mst_force_edge(n, &edges, Some(i)).expect("Graph has no MST");
                if weight == base_weight {
                    pseudocritical.insert(edges[i].clone());
                }
            }

            // Test for critical edge
            let old_edge = edges.remove(i);
            if let Some((weight, _)) = Self::find_mst(n, &edges) {
                if weight <= base_weight {
                    critical.remove(&old_edge);
                }
            }
        }

        let pseudocritical: Vec<i32> = pseudocritical.into_iter().filter_map(|e| {
            if critical.contains(&e) {
                None
            }
            else {
                Some(e.num as i32)
            }
        }).collect();

        return vec![
            critical.into_iter().map(|e| e.num as i32).collect(),
            pseudocritical
        ];
    }
}

#[test]
fn test_critical_edges() {
    let mut _test = 1;
    let mut test_case = |n: i32, edges: &[[i32;3]], expected: [&[i32]; 2]| {
        let expected: [HashSet<i32>;2] = expected.map(|e| HashSet::from_iter(e.into_iter().copied()));
        let edges = edges.into_iter().map(|edge| edge.to_vec()).collect();
        let result: Vec<HashSet<i32>> = Solution::find_critical_and_pseudo_critical_edges(n, edges)
            .into_iter().map(|r| HashSet::from_iter(r.into_iter())).collect();
        let success = result == expected;
        println!("Test {_test}: {}", if success {"SUCCESS"} else {"FAILIURE"});
        if !success {
            println!("EXPECTED: {expected:?}");
            println!("RESULT: {result:?}");
        }
        _test += 1;
    };

    test_case(
        5,
        &[[0,1,1],[1,2,1],[2,3,2],[0,3,2],[0,4,3],[3,4,3],[1,4,6]],
        [&[0,1],&[2,3,4,5]]
    );
    test_case(
        6,
        &[[0,1,1],[1,2,1],[0,2,1],[2,3,4],[3,4,2],[3,5,2],[4,5,2]],
        [&[3],&[0,1,2,4,5,6]]
    );
    test_case(
        4,
        &[[0,1,1],[0,3,1],[0,2,1],[1,2,1],[1,3,1],[2,3,1]],
        [&[],&[0,1,2,3,4,5]]
    )
}
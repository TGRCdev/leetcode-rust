//! https://leetcode.com/problems/01-matrix/
pub struct Solution;

impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;

        let (m,n) = (mat.len(), mat[0].len());

        let mut queue = VecDeque::new();
        mat.iter_mut().enumerate().for_each(|(i, row)| {
            row.iter_mut().enumerate().for_each(|(j, elem)| {
                if *elem == 0 {
                    queue.push_back((i,j));
                }
                else {
                    *elem = i32::MAX;
                }
            });
        });

        while let Some((i,j)) = queue.pop_front() {
            if mat[i][j] == 0 {
                let mut update_and_push = |i: usize,j: usize| {
                    if mat[i][j] > 1 {
                        mat[i][j] = mat[i][j].min(1);
                        queue.push_back((i,j));
                    }
                };
                if i < m-1 {
                    update_and_push(i+1,j);
                }
                if j < n-1 {
                    update_and_push(i,j+1);
                }
                if i > 0 {
                    update_and_push(i-1,j);
                }
                if j > 0 {
                    update_and_push(i,j-1);
                }
            }
            else {
                let mut check_if_process = |n_i: usize, n_j: usize| {
                    if mat[n_i][n_j] > 0 && mat[i][j]+1 < mat[n_i][n_j] {
                        mat[n_i][n_j] = mat[i][j]+1;
                        queue.push_back((n_i,n_j));
                    }
                };
                if i < m-1 {
                    check_if_process(i+1,j);
                }
                if j < n-1 {
                    check_if_process(i,j+1);
                }
                if i > 0 {
                    check_if_process(i-1,j);
                }
                if j > 0 {
                    check_if_process(i,j-1);
                }
            }
        }

        return mat;
    }
}

#[test]
fn zeroone_matrix_test() {
    let mut test_num = 1;
    let mut test_case = |mat: &[&[i32]], expected: &[&[i32]]| {
        let mat = mat.iter().map(|row| row.to_vec()).collect();
        let result = Solution::update_matrix(mat);
        println!("Test {test_num}: {}", if result == expected {"SUCCESS"} else {"FAILIURE"});
        if result != expected {
            println!("Expected: {expected:#?}\nGot: {result:#?}");
        }
        test_num += 1;
    };

    test_case(&[
            &[0,0,1,0,1,1,1,0,1,1],
            &[1,1,1,1,0,1,1,1,1,1],
            &[1,1,1,1,1,0,0,0,1,1],
            &[1,0,1,0,1,1,1,0,1,1],
            &[0,0,1,1,1,0,1,1,1,1],
            &[1,0,1,1,1,1,1,1,1,1],
            &[1,1,1,1,0,1,0,1,0,1],
            &[0,1,0,0,0,1,0,0,1,1],
            &[1,1,1,0,1,1,0,1,0,1],
            &[1,0,1,1,1,0,1,1,1,0]
        ],
        &[
            &[0,0,1,0,1,2,1,0,1,2],
            &[1,1,2,1,0,1,1,1,2,3],
            &[2,1,2,1,1,0,0,0,1,2],
            &[1,0,1,0,1,1,1,0,1,2],
            &[0,0,1,1,1,0,1,1,2,3],
            &[1,0,1,2,1,1,1,2,1,2],
            &[1,1,1,1,0,1,0,1,0,1],
            &[0,1,0,0,0,1,0,0,1,2],
            &[1,1,1,0,1,1,0,1,0,1],
            &[1,0,1,1,1,0,1,2,1,0]
        ],
    );

    test_case(
        &[
            &[1,1,0,0,1,0,0,1,1,0],
            &[1,0,0,1,0,1,1,1,1,1],
            &[1,1,1,0,0,1,1,1,1,0],
            &[0,1,1,1,0,1,1,1,1,1],
            &[0,0,1,1,1,1,1,1,1,0],
            &[1,1,1,1,1,1,0,1,1,1],
            &[0,1,1,1,1,1,1,0,0,1],
            &[1,1,1,1,1,0,0,1,1,1],
            &[0,1,0,1,1,0,1,1,1,1],
            &[1,1,1,0,1,0,1,1,1,1]
        ],
        &[],
    );
}
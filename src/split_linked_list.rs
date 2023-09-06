//! https://leetcode.com/problems/split-linked-list-in-parts/

pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

#[allow(dead_code)]
impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn split_list_to_parts(mut head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let k = k as usize;
        let mut nodes: Vec<Option<Box<ListNode>>> = Vec::new();
        
        while let Some(mut node) = head {
            head = node.next.take();
            nodes.push(Some(node));
        }

        if nodes.len() <= k {
            nodes.resize(k, None);
            return nodes;
        }

        let list_len = nodes.len() / k;
        let mut extras = nodes.len() % k;
        let mut lists = Vec::with_capacity(k);
        while !nodes.is_empty() {
            let extra = {
                if extras > 0 {
                    extras -= 1;
                    1
                }
                else { 0 }
            };
            let mut list_iter = nodes.drain(..list_len+extra).rev();
            let mut head = None;
            while let Some(Some(mut node)) = list_iter.next() {
                node.next = head;
                head = Some(node);
            }
            lists.push(head);
        }

        lists
    }
}
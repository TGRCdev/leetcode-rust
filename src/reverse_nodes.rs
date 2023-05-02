//! https://leetcode.com/problems/reverse-nodes-in-k-group

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub struct Solution;

impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let k = k as usize;
        let mut chain_bank = Vec::new();
        while head.is_some() {
            let next = {
                let node = head.as_mut().unwrap();
                node.next.take()
            };
            chain_bank.push(head.unwrap());
            head = next;
        }

        for i in (0..chain_bank.len()).step_by(k) {
            if i+k <= chain_bank.len() {
                chain_bank[i..i+k].reverse();
            }
        }

        let mut last = chain_bank.pop().unwrap();
        while let Some(mut current) = chain_bank.pop() {
            current.next = Some(last);
            last = current;
        }

        return Some(last);
    }
}
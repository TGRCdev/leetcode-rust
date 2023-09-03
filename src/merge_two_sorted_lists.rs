//! https://leetcode.com/problems/merge-two-sorted-lists/

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

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }
        else if list2.is_none() {
            return list1;
        }

        let mut root;
        {
            if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
                root = list1.take().unwrap();
                list1 = root.next.take();
            }
            else {
                root = list2.take().unwrap();
                list2 = root.next.take();
            }
        }
        let mut head = &mut root;

        loop {
            match (list1.take(), list2.take()) {
                (Some(mut node1), Some(mut node2)) => {
					if node1.val <= node2.val {
						list1 = node1.next.take();
						list2 = Some(node2);
                        head.next = Some(node1);
                        head = head.next.as_mut().unwrap();
                    }
                    else {
						list1 = Some(node1);
						list2 = node2.next.take();
						head.next = Some(node2);
						head = head.next.as_mut().unwrap();
                    }
                }
				(None, Some(leftover)) | (Some(leftover), None) => {
					head.next = Some(leftover);
					break;
				}
				(None, None) => break,
            }
        }

		Some(root)
    }
}
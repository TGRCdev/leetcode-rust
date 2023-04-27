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
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
		// Trim None nodes
		lists.retain(|node| node.is_some());

		if lists.is_empty() {
			return None;
		}

		fn take_lowest_node(lists: &mut Vec<Option<Box<ListNode>>>) -> Box<ListNode> {
			let mut smallest_i = 0;
			let mut smallest = lists[0].as_ref().unwrap().val;

			for i in 1..lists.len() {
				let node = lists[i].as_ref().unwrap();
				if node.val < smallest {
					smallest = node.val;
					smallest_i = i;
				}
			}

			let mut next = lists[smallest_i].as_mut().unwrap().next.take();
			if next.is_some() {
				std::mem::swap(&mut lists[smallest_i], &mut next);
			}
			else {
				next = lists.swap_remove(smallest_i);
			}

			return next.unwrap();
		}

		let mut root = take_lowest_node(&mut lists);
		let mut head = &mut root;

		while !lists.is_empty() {
			let next_node = take_lowest_node(&mut lists);
			head.next = Some(next_node);
			head = head.next.as_mut().unwrap();
		}

		return Some(root);
    }
}

#[test]
fn test() {
	fn test_merge(lists: Vec<Vec<i32>>) {
		
	}
}
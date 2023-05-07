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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>
    {
      if let Some(mut node1) = l1 {
        if let Some(mut node2) = l2 {
          let result = node1.val + node2.val;
          node1.val = result % 10;
          if result > 9
          {
            if let Some(mut next) = node2.next {
              next.val += 1;
              node2.next = Some(next);
            }
            else
            {
              node2.next = Some(Box::new(ListNode::new(1)));
            }
          }

          node1.next = Self::add_two_numbers(node1.next, node2.next);
          return Some(node1);
        }
        else
        {
          return Some(node1);
        }
      }
      else
      {
        if let Some(mut node) = l2 {
          if node.val > 9 {
              node.val %= 10;
            if let Some(mut next) = node.next {
              next.val += 1;
              node.next = Self::add_two_numbers(None, Some(next));
              return Some(node);
            }
            else
            {
              node.next = Some(Box::new(ListNode::new(1)));
              return Some(node);
            }
          }
          else
          {
            return Some(node);
          }
        }
        else
        {
          return None;
        }
      }
    }
}
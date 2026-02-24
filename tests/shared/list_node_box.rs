#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn create_node_list(values: &[i32]) -> Option<Box<ListNode>> {
    if values.is_empty() {
        return None;
    }

    let mut nodes: Vec<Box<ListNode>> = values
        .iter()
        .rev()
        .map(|v| Box::new(ListNode::new(*v)))
        .collect();

    if nodes.len() == 1 {
        return Some(nodes[0].clone());
    }

    let result = nodes.iter_mut().reduce(|last, item| {
        item.next = Some(Box::clone(last));
        item
    });

    Some(result.unwrap().clone())
}

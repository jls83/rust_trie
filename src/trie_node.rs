use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TrieNodeType {
    Final,
    Intermediate,
}

#[derive(Clone, Eq, PartialEq)]
pub struct TrieNode {
    pub value: Option<char>,
    pub children: HashMap<char, TrieNode>,
    pub node_type: TrieNodeType,
    pub word_score: Option<i64>,
    pub node_score: i64,
    pub children_new: HashMap<char, usize>,
}

impl TrieNode {
    pub fn new(value: Option<char>) -> Self {
        TrieNode {
            value,
            children: HashMap::new(),
            node_type: TrieNodeType::Intermediate,
            word_score: None,
            node_score: 0,
            children_new: HashMap::new(),
        }
    }
}

impl Ord for TrieNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.node_score.cmp(&other.node_score)
    }
}

impl PartialOrd for TrieNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

use std::cmp::Ordering;
use std::collections::hash_map::Values;

use crate::helpers::output_wrapper::OutputWrapper;
use crate::trie_node::{TrieNode, TrieNodeType};

#[derive(Clone, Eq, PartialEq)]
pub struct QueueWrapper<'a> {
    pub nodes: Vec<&'a TrieNode>,
}

impl<'a> QueueWrapper<'a> {
    pub fn last(&self) -> Option<&&'a TrieNode> {
        self.nodes.last()
    }

    pub fn output_score(&self) -> i64 {
        match self.last() {
            Some(node) => node.node_score,
            _ => 0,
        }
    }

    pub fn to_output_wrapper(&self) -> OutputWrapper<'a> {
        OutputWrapper {
            nodes: self.nodes.to_owned(),
        }
    }

    pub fn new_with_node(&self, node: &'a TrieNode) -> Self {
        let mut nodes = self.nodes.to_owned();
        nodes.push(node);
        Self { nodes }
    }

    pub fn children(&self) -> Option<Values<'a, char, TrieNode>> {
        match self.last() {
            Some(node) => Some(node.children.values()),
            _ => None,
        }
    }

    pub fn leaf_type(&self) -> Option<TrieNodeType> {
        match self.last() {
            Some(node) => Some(node.node_type),
            _ => None,
        }
    }
}

impl Ord for QueueWrapper<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.output_score().cmp(&other.output_score())
    }
}

impl PartialOrd for QueueWrapper<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

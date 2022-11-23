use std::cmp::Ordering;

use crate::helpers::queue_wrapper::QueueWrapper;
use crate::trie_node::TrieNode;

#[derive(Clone, Eq, PartialEq)]
pub struct OutputWrapper<'a> {
    pub nodes: Vec<&'a TrieNode>,
}

impl<'a> OutputWrapper<'a> {
    pub fn join(&self) -> String {
        self.nodes
            .iter()
            .map(|n| n.value.unwrap_or_default())
            .collect::<String>()
    }

    pub fn last(&self) -> Option<&&'a TrieNode> {
        self.nodes.last()
    }

    pub fn output_score(&self) -> i64 {
        match self.last() {
            Some(node) => match node.word_score {
                Some(score) => score,
                _ => 0,
            },
            _ => 0,
        }
    }

    pub fn to_queue_wrapper(&self) -> QueueWrapper<'a> {
        QueueWrapper {
            nodes: self.nodes.to_owned(),
        }
    }
}

impl Ord for OutputWrapper<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.output_score().cmp(&other.output_score())
    }
}

impl PartialOrd for OutputWrapper<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

use std::cmp;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Eq, PartialEq)]
enum TrieNodeType {
    Final(String),
    Intermediate,
}

#[derive(Clone, Eq, PartialEq)]
struct TrieNode {
    value: Option<char>,
    children: HashMap<char, TrieNode>,
    node_type: TrieNodeType,
    word_score: Option<i64>,
    node_score: i64,
}

impl TrieNode {
    fn new(value: Option<char>) -> Self {
        TrieNode {
            value,
            children: HashMap::new(),
            node_type: TrieNodeType::Intermediate,
            word_score: None,
            node_score: 0,
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

pub struct Trie {
    root: TrieNode,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
struct QueueWrapper<'a> {
    node: &'a TrieNode,
    nodes_previous: Vec<&'a TrieNode>,
}

#[derive(Clone, Eq, PartialEq)]
struct OutputWrapper<'a> {
    node: &'a TrieNode,
    nodes_previous: Vec<&'a TrieNode>,
}

impl<'a> OutputWrapper<'a> {
    fn wordify(&self, prefix: &String) -> Option<String> {
        if let TrieNodeType::Final(_) = self.node.node_type {
            return Some(
                prefix.to_owned()
                    + &self
                        .nodes_previous
                        .iter()
                        .map(|n| n.value.unwrap())
                        .collect::<String>(),
            );
        }
        None
    }
}

impl Ord for OutputWrapper<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.node.word_score.cmp(&other.node.word_score)
    }
}

impl PartialOrd for OutputWrapper<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(None),
        }
    }

    fn _insert(&mut self, word: String, score: i64) {
        let mut current_node = &mut self.root;

        for char in word.chars() {
            let mut next_node = current_node
                .children
                .entry(char)
                .or_insert(TrieNode::new(Some(char)));
            next_node.node_score = cmp::max(next_node.node_score, score);
            current_node = next_node;
        }

        // Set some properties on the last node so that it can be used as a representation of the
        // incoming `word`.
        current_node.node_type = TrieNodeType::Final(word);
        current_node.word_score = Some(score);
    }

    fn _search(&self, word: &String) -> Option<&TrieNode> {
        let mut current_node = &self.root;

        for char in word.chars() {
            if let Some(next_node) = current_node.children.get(&char) {
                current_node = next_node;
            } else {
                return None;
            }
        }
        Some(current_node)
    }

    // TODO: add `k` param to return only the top `k` results.
    pub fn get_ranked_results(&self, prefix: String) -> Option<Vec<String>> {
        // Our collection of "found" items is represented by `OutputWrapper`
        // instances so that we can use a specific `Ord` trait implementation
        // to order by the underlying word's score before returning.
        let mut found_nodes: BinaryHeap<OutputWrapper> = BinaryHeap::new();

        let mut heap: BinaryHeap<QueueWrapper> = BinaryHeap::new();

        if let Some(node) = self._search(&prefix) {
            for child in node.children.values() {
                heap.push(QueueWrapper {
                    node: child,
                    nodes_previous: vec![],
                });
            }
        } else {
            return None;
        }

        while let Some(QueueWrapper { node, mut nodes_previous }) = heap.pop() {
            nodes_previous.push(node);
            if let TrieNodeType::Final(_) = &node.node_type {
                found_nodes.push(OutputWrapper {
                    node,
                    nodes_previous: nodes_previous.to_owned(),
                });
            }
            for child in node.children.values() {
                heap.push(QueueWrapper {
                    node: child,
                    nodes_previous: nodes_previous.to_owned(),
                });
            }
        }

        // NOTE: It's a bit convoluted to turn a `BinaryHeap` into a `Vec` with the values in heap
        // order. `BinaryHeap.into_iter_sorted` will do what we need, but it is not yet stable (see
        // https://github.com/rust-lang/rust/issues/59278).
        let result: Vec<String> = found_nodes
            .into_sorted_vec()
            .iter()
            .rev()
            .map(|t| t.wordify(&prefix).unwrap())
            .collect();

        Some(result)
    }

    pub fn insert(&mut self, word: String) {
        self._insert(word, 0);
    }

    pub fn insert_with_score(&mut self, word: String, score: i64) {
        self._insert(word, score);
    }

    pub fn search(&self, word: String) -> Option<String> {
        match self._search(&word) {
            Some(TrieNode {
                node_type: TrieNodeType::Final(result),
                ..
            }) => Some(result.to_string()),
            _ => None,
        }
    }

    pub fn starts_with(&self, prefix: String) -> Option<String> {
        match self._search(&prefix) {
            Some(_) => Some(prefix),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Trie;

    #[test]
    fn can_search_for_term() {
        let search_term = "Foo";
        let mut trie = Trie::new();
        trie.insert(search_term.to_string());

        assert_eq!(
            Some(search_term.to_string()),
            trie.search(search_term.to_string())
        );
    }

    #[test]
    fn can_search_for_term_with_score() {
        let search_term = "Foo";
        let mut trie = Trie::new();
        trie.insert_with_score(search_term.to_string(), 10);

        assert_eq!(
            Some(search_term.to_string()),
            trie.search(search_term.to_string())
        );
    }

    #[test]
    fn can_search_for_term_with_similar_entries() {
        let search_term = "Foo";
        let insert_terms = vec!["Foo", "For"];

        let mut trie = Trie::new();
        for term in insert_terms {
            trie.insert(term.to_string());
        }

        assert_eq!(
            Some(search_term.to_string()),
            trie.search(search_term.to_string())
        );
    }

    #[test]
    fn can_find_starts_with_items() {
        let insert_term = "Foo";
        let mut trie = Trie::new();
        trie.insert(insert_term.to_string());

        let prefix = "Fo";

        assert_eq!(
            Some(prefix.to_string()),
            trie.starts_with(prefix.to_string())
        );
    }

    #[test]
    fn missing_search_term_returns_none() {
        let insert_term = "Foo";
        let search_term = "Bar";

        let mut trie = Trie::new();
        trie.insert(insert_term.to_string());

        assert_eq!(None, trie.search(search_term.to_string()));
    }

    #[test]
    fn missing_starts_with_prefix_returns_none() {
        let insert_term = "Foo";
        let prefix = "Ba";

        let mut trie = Trie::new();
        trie.insert(insert_term.to_string());

        assert_eq!(None, trie.starts_with(prefix.to_string()));
    }

    #[test]
    fn get_ranked_results_uses_score_ordering() {
        let words_and_scores = vec![("Foreign", 10), ("For", 8), ("Foo", 0)];

        let expected_words: Vec<String> = words_and_scores
            .iter()
            .map(|(word, _)| word.to_string())
            .collect();

        let mut trie = Trie::new();

        for (word, score) in words_and_scores.iter() {
            trie.insert_with_score(word.to_string(), *score);
        }

        let ranked_results = trie.get_ranked_results("Fo".to_string()).unwrap();

        assert_eq!(expected_words, ranked_results);
    }
}
